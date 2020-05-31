// Modules
mod systems;
mod components;
mod resources;
mod utils;

use resources::{
    asset_handler::AssetHandler,
    active_camera::ActiveCamera,
    screen_dimensions::ScreenDimensions,
};

// Imports
use ggez::*;
use specs::*;
use ggez::nalgebra as math;


// Game State Struct
struct State {
    world : World,
}

// Connects the Game State to the Main Event Handler
impl event::EventHandler for State {

    // Core Game Loop
    fn update(&mut self, _ctx : &mut Context) -> GameResult<()> {
        Ok(())
    }

    // Core Render LoOp
    fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {

        let mut culling_system = systems::culling_system::CullingSystem { };
        let mut render_system  = systems::render_system::RenderSystem { };
        culling_system.run_now(&self.world);
        render_system.draw(ctx, &self.world);

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(
            0.0,
            0.0,
            width as f32,
            height as f32,
        );
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    
        let mut screen_size = self.world.write_resource::<ScreenDimensions>();
        screen_size.x = width;
        screen_size.y = height;
        screen_size.rect = new_rect;
    }

}

fn main() {

    // Initalize our asset Path
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        std::path::PathBuf::from("./assets")
    };

    // Create and Configure the ggez Context
    let context_builder = ggez::ContextBuilder::new("Boundless", "Anthony Brigante")
        .window_setup(conf::WindowSetup::default().title("Boundless").samples(ggez::conf::NumSamples::One))
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true),
        )
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut context_builder.build().unwrap();
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    // Initialize specs
    let mut world = World::new();

    // Register Components
    world.register::<components::Sprite>();
    world.register::<components::Transform>();
    world.register::<components::Camera>();
    world.register::<components::Culled>();

    let (w, h) = graphics::drawable_size(ctx);

    // Build Entity
    world
        .create_entity()
        .with(components::Sprite::dirt())
        .with(components::Transform {
            position : math::Point2::new(0.0, 0.0),
            scale    : math::Vector2::new(1.0, 1.0),
        })
        .build();

    world
        .create_entity()
        .with(components::Sprite::grass())
        .with(components::Transform {
            position : math::Point2::new(64.0, 0.0),
            scale    : math::Vector2::new(1.0, 1.0),
        })
        .build();

    world
        .create_entity()
        .with(components::Sprite {
            spritesheet_dir : "/characters_spritesheet.png".into(),
            ..components::Sprite::default()
        })
        .with(components::Transform {
            position : math::Point2::new(-64.0, 0.0),
            scale    : math::Vector2::new(1.0, 1.0),
        })
        .build();
    
    let camera_entity = 
        world
            .create_entity()
            .with(components::Camera {})
            .with(components::Transform {
                position : math::Point2::new(0.0, 0.0),
                scale    : math::Vector2::new(1.0, 1.0),
            })
            .build();


    // Add Resources
    let mut asset_handler = AssetHandler::new();
    load_assets(ctx, &mut asset_handler);
    world.insert(asset_handler);

    let active_camera = ActiveCamera {
        entity : Some(camera_entity),
    };
    world.insert(active_camera);

    let screen_dimensions = ScreenDimensions::default();
    world.insert(screen_dimensions);
    
    // Create and Launch the Game State
    let mut state = State { world };
    event::run(ctx, event_loop,&mut state).unwrap();
}

fn load_assets( ctx : &mut Context, asset_handler : &mut AssetHandler ) {

    asset_handler.add_asset(ctx, "/characters_spritesheet.png").expect("Asset Could Not Be Loaded!");
    asset_handler.add_asset(ctx, "/tiles_spritesheet.png").expect("Asset Could Not Be Loaded!");

}