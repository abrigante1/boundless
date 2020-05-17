use ggez::*;
use specs::*;
use nalgebra as math;

use std::path;
use std::env;

mod systems;
mod components;

type Point2 = math::Point2<f32>;
type Vector2   = math::Vector2<f32>;

struct State {
    world : World,
    render_system : systems::RenderSystem,
}   

impl State {
    fn new( world : World, ctx : &mut Context ) -> GameResult<State> {

        let (w, h) = graphics::drawable_size(ctx);
        
        let state = State {
            world,
            render_system : systems::RenderSystem { screen_size : Point2::new(w, h) },
        };

        Ok(state)
    }
}

impl event::EventHandler for State {


    fn update(&mut self, _ctx : &mut Context) -> GameResult<()> {

        let mut demo_system = systems::DemoSystem {};
        demo_system.run_now(&self.world);

        Ok(())
    }

    fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {

        self.render_system.draw(ctx, &mut self.world);

        Ok(())
    }

}

fn main() {

    // Initalize our Resource Path
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Create the ggez Context
    let context_builder = ggez::ContextBuilder::new("Sandbox for ggez", "Anthony Brigante")
        .window_setup(conf::WindowSetup::default().title("Sandbox!"))
        .window_mode(conf::WindowMode::default().dimensions(500.0, 500.0))
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut context_builder.build().unwrap();

    // Create the World and Register Components
    let mut world = World::new();
    world.register::<components::Transform>();
    world.register::<components::Sprite>();

    // Create Dummy Awesome Face
    world.create_entity()
        .with(components::Transform { 
            position : Point2::new(0.0, 0.0), 
            scale : Vector2::new(0.5, 0.5) 
        })
        .with(components::Sprite { image : graphics::Image::new(ctx, "/awesome_face.png" ).unwrap() })
        .build();

    let state = &mut State::new(world, ctx).unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
