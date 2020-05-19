use ggez::*;
use ggez::input;
use specs::*;
use nalgebra as math;

use std::path;
use std::env;

mod systems;
mod components;

type Point2 = math::Point2<f32>;
type Vector2   = math::Vector2<f32>;

struct InputHandler {
    pos_x : f32,
    pos_y : f32,
    mouse_down : bool,
}

struct State {
    world : World,
}   

impl State {
    fn new( world : World ) -> GameResult<State> {

        
        let state = State {
            world,
        };

        Ok(state)
    }
}

impl event::EventHandler for State {


    fn update(&mut self, _ctx : &mut Context) -> GameResult<()> {

        //let mut demo_system = systems::DemoSystem {};
       // demo_system.run_now(&self.world);

        Ok(())
    }

    fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {


        let mut render_system = systems::RenderSystem{};
        render_system.draw(ctx, &self.world);

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: input::mouse::MouseButton, x: f32, y: f32) {
        let mut input_handler = self.world.write_resource::<InputHandler>();

        input_handler.mouse_down = true;

        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: input::mouse::MouseButton, x: f32, y: f32) {
        let mut input_handler = self.world.write_resource::<InputHandler>();

        input_handler.mouse_down = false;

        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, xrel: f32, yrel: f32) {
        let mut input_handler = self.world.write_resource::<InputHandler>();
        let mut transforms = self.world.write_storage::<components::Transform>();

        let transform = {
            let camera_entity = self.world.read_resource::<systems::ActiveCamera>().entity.unwrap();
            let entity = transforms.get_mut(camera_entity).unwrap();
            entity
        };


        
        if input_handler.mouse_down {

            //println!(
            //    "Mouse motion, x: {}, y: {}, Prev x: {}, Prev y: {}, relative x: {}, relative y: {}",
            //    x, y, input_handler.pos_x, input_handler.pos_y, xrel, yrel
            //);

            if !(input_handler.pos_x == x && input_handler.pos_y == y) {

                input_handler.pos_x = x;
                input_handler.pos_y = y;

                transform.position.x = transform.position.x - xrel;
                transform.position.y = transform.position.y - yrel;

            }


        }
        
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {

        let mut transforms = self.world.write_storage::<components::Transform>();

        let transform = {
            let camera_entity = self.world.read_resource::<systems::ActiveCamera>().entity.unwrap();
            transforms.get_mut(camera_entity).unwrap()
        };

        transform.scale.x += (y / 100.0);
        transform.scale.y += (y / 100.0);

        
        let (w, h) = graphics::drawable_size(ctx);
        let new_rect = graphics::Rect::new(0.0, 0.0, w as f32 * transform.scale.x, h as f32 * transform.scale.y);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
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
        //.window_mode(conf::WindowMode::default().dimensions(500.0, 500.0))
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut context_builder.build().unwrap();

    // Create the World and Register Components
    let mut world = World::new();
    world.register::<components::Transform>();
    world.register::<components::Sprite>();
    world.register::<components::Camera>();

    // Create Dummy Awesome Face
    world.create_entity()
        .with(components::Transform { 
            position : Point2::new(0.0, 0.0), 
            scale : Vector2::new(0.5, 0.5) 
        })
        .with(components::Sprite { image : graphics::Image::new(ctx, "/awesome_face.png" ).unwrap() })
        .build();


    // Create Camera at Origin
    let camera = world.create_entity()
        .with(components::Transform {
            position : Point2::new(250.0, 0.0),
            scale    : Vector2::new(1.0, 1.0),
        })
        .with(components::Camera {})
        .build();

    // Create Resources
    let active_camera = systems::ActiveCamera{ entity: Some(camera) };
    world.insert(active_camera);
    let input_handler = InputHandler { pos_x : 0.0, pos_y : 0.0, mouse_down : false };
    world.insert(input_handler);

    let state = &mut State::new(world).unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
