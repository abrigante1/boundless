use amethyst::{
    ecs::{
        RunNow,
        Entity
    },
    core::transform::{
        TransformBundle,
        Transform,
        Parent
    },
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
        SpriteSheet,
        SpriteRender,
        SpriteSheetFormat,
        Texture,
        ActiveCamera,
        sprite_visibility::SpriteVisibilitySortingSystem,
    },
    input::{
        InputBundle,
        StringBindings
    },
};

mod components;
pub use components::*;

mod systems;
pub use systems::*;

mod alias;
pub use alias::*;

pub struct SpriteSheetManager {
    pub tiles       : Handle<SpriteSheet>,
    pub characters  : Handle<SpriteSheet>,
    pub backgrounds : Handle<SpriteSheet>,
}

struct Game;

impl Game {
    fn run_systems(&mut self, world : &mut World) {
        world.maintain();
    }
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let mut world = data.world;

        // Register Components
        world.register::<Tile>();
        world.register::<Dirt>();
        world.register::<GrassyDirt>();
        world.register::<TreeTrunk>();
        world.register::<TreeLeaves>();
        world.register::<Tree>();

        let tile_spritesheet = SpriteSheetManager {
            tiles       : load_tiles_spritesheet(world),
            characters  : load_characters_spritesheet(world),
            backgrounds : load_backgrounds_spritesheet(world),
        };

        // Insert the Spritesheet Resource
        world.insert(tile_spritesheet);

        let player     = initalize_player(world, Vector3::new(64.0, 64.0 * 11.5, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let camera = initalize_camera(world, player);

        let background = initalize_background(world, Vector3::new(0.0, 0.0, -10.0), Vector3::new(2.0, 2.0, 2.0), camera);


        let (width, height) = (256, 32);

        let mut world_generator = WorldGenSystem {
            world_height : height as usize,
            world_width  : width as usize,
        };

        world_generator.run_now(&mut world);


        world.maintain();
    }

    fn update(&mut self, data : &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        self.run_systems(world);

        Trans::None
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
    let bindings_config = app_root.join("config").join("bindings.ron");
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
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config)?)?
        .with(systems::GodCameraSystem { reader_id : None }, "god_camera_system", &[])
        .with(SpriteVisibilitySortingSystem::default(), "visibility_system", &["transform_system"]);


    let mut game = Application::new(assets_dir, Game, game_data)?;
    game.run();

    Ok(())
}

fn initalize_camera(world: &mut World, parent : Entity) -> Entity {

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    let camera = world
        .create_entity()
        .with(Parent { entity: parent })
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();

    let active_camera = ActiveCamera {
        entity: Some(camera)
    };

    world.insert(active_camera);

    camera
}

fn initalize_player(world : &mut World, position : Vector3, scale : Vector3) -> Entity {

    let spritesheet = (&*world.read_resource::<crate::SpriteSheetManager>()).characters.clone();

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    world.create_entity_unchecked()
        .with(transform)
        .with(Tile)
        .with(GrassyDirt)
        .with(SpriteRender {
            sprite_sheet  : spritesheet,
            sprite_number : 0,
        })
        .build()
}

fn initalize_background(world : &mut World, position : Vector3, scale : Vector3, parent : Entity) -> Entity {

    let spritesheet = (&*world.read_resource::<crate::SpriteSheetManager>()).backgrounds.clone();

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    world.create_entity_unchecked()
        .with(transform)
        .with(Tile)
        .with(Parent { entity : parent })
        .with(SpriteRender {
            sprite_sheet  : spritesheet,
            sprite_number : 0,
        })
        .build()
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

fn load_backgrounds_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let spritesheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    let tex_handle = loader.load(
        "backgrounds/backgrounds_spritesheet.png",
        ImageFormat::default(),
        (),
        &texture_storage);

    loader.load(
        "backgrounds/backgrounds.ron",
        SpriteSheetFormat(tex_handle),
        (),
        &spritesheet_storage,
    )
}

fn load_characters_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let spritesheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    let tex_handle = loader.load(
        "characters/characters_spritesheet.png",
        ImageFormat::default(),
        (),
        &texture_storage);

    loader.load(
        "characters/characters.ron",
        SpriteSheetFormat(tex_handle),
        (),
        &spritesheet_storage,
    )
}