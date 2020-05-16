use amethyst::{
    core::transform::TransformBundle,
    core::transform::Transform,
    prelude::*,
    utils::application_root_dir,
    window::ScreenDimensions,
    winit::{
        Event, 
        WindowEvent,
    },
    assets::{
        AssetStorage,
        Loader,
        Handle,
    },
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        Camera,
        ImageFormat,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
        ActiveCamera,
    },
    input::{
        InputBundle,
        StringBindings
    },
};

mod components;
pub use components::tiles::*;

mod systems;
pub use systems::*;

pub struct TileSpritesheet {
    pub spritesheet : Handle<SpriteSheet>
}

struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;

        // Register Components
        world.register::<Tile>();
        world.register::<Dirt>();

        let tile_spritesheet = TileSpritesheet {
            spritesheet : load_tiles_spritesheet(world),
        };

        // Insert the Spritesheet Resource
        world.insert(tile_spritesheet);

        initalize_camera(world);

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };


        let world_generator = WorldGenSystem {
            world_height : (height as f32 / 16.0) as usize,
            world_width  : (width as f32 / 16.0) as usize,
        };

        world_generator.create_world(world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {


        if let StateEvent::Window(event) = &event {
            if let Event::WindowEvent { event, .. } = event {
                if let WindowEvent::Resized(size) = event {

                    let mut fetched_camera = data.world.try_fetch_mut::<ActiveCamera>();

                    if let Some(fetched_camera) = &mut fetched_camera {

                        let camera_option = fetched_camera.entity;
                        if let Some(camera_entity) = camera_option {

                            let mut storage = data.world.write_storage::<Camera>();
                            let camera = storage.get_mut(camera_entity).expect("Camera Entity Does Not Exist");
                            *camera = Camera::standard_2d(size.width as f32, size.height as f32);

                        }

                    }
                }
            }

        }

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?;

    let mut game = Application::new(assets_dir, Game, game_data)?;
    game.run();

    Ok(())
}

fn initalize_camera(world: &mut World) {

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    let camera = world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();

    let active_camera = ActiveCamera {
        entity: Some(camera)
    };

    world.insert(active_camera);
}

fn load_tiles_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let spritesheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    let tex_handle = loader.load(
        "tiles/tiles_spritesheet.png",
        ImageFormat::default(),
        (),
        &texture_storage);

    loader.load(
        "tiles/tiles.ron",
        SpriteSheetFormat(tex_handle),
        (),
        &spritesheet_storage,
    )
}