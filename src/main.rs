use ggez::*;
use ggez::input;
use specs::*;
use nalgebra as math;

use std::path;
use std::env;

mod systems;
mod components;
mod alias;
pub use alias::*;

pub type TileMap = Vec<Entity>;

struct InputHandler {
    pos_x : f32,
    pos_y : f32,
    mouse_down : bool,
}

struct AssetHandler {
    player                 : graphics::Image,
    dirt_tile_batch        : graphics::spritebatch::SpriteBatch,
    grassy_dirt_tile_batch : graphics::spritebatch::SpriteBatch,
    tile_spritesheet       : graphics::spritebatch::SpriteBatch,
    background             : graphics::Image,
}

struct State {
    world : World,
    world_gen_system : systems::WorldGenSystem,
}   

impl State {
    fn new( world : World ) -> GameResult<State> {

        let mut world_gen_system = systems::WorldGenSystem { 
            world_width : 128,
            world_height : 128,
        };

        world_gen_system.run_now(&world);

        let state = State {
            world,
            world_gen_system,
        };

        Ok(state)
    }
}

impl event::EventHandler for State {


    fn update(&mut self, _ctx : &mut Context) -> GameResult<()> {

       //let mut demo_system = systems::DemoSystem {};
       //demo_system.run_now(&self.world);
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, ctx : &mut Context) -> GameResult<()> {

        let mut physics_system = systems::PhysicsSystem{};
        physics_system.run_now(&mut self.world);

        let mut render_system = systems::RenderSystem{};
        render_system.draw(ctx, &self.world);


        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: input::mouse::MouseButton, x: f32, y: f32) {
        let mut input_handler = self.world.write_resource::<InputHandler>();

        if button == input::mouse::MouseButton::Middle {
            input_handler.mouse_down = true;
        }

        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: input::mouse::MouseButton, x: f32, y: f32) {

        let (w, h) = graphics::drawable_size(ctx);
        
        // Get the Camera's Transform
        let transforms       = self.world.read_storage::<components::Transform>();
        let names            = self.world.read_storage::<components::Named>();

        let entities         = self.world.entities();
        let lazy             = self.world.read_resource::<LazyUpdate>();
        let active_camera    = self.world.read_resource::<systems::ActiveCamera>();
        let camera_transform = transforms.get(active_camera.entity.unwrap()).unwrap();

        let point = systems::RenderSystem::screen_to_world_coords(Point2::new(w, h), 
                                                                camera_transform, 
                                                                Point2::new(x, y));

        println!("world coords: {:?}", (point.x, point.y));


        let indx = self.world_gen_system.to_tile_coords(point.x, point.y);

        println!("Index: {}", indx);

        // Get Map
        let mut tile_map = self.world.write_resource::<TileMap>();

        if let Some(ent) = tile_map.get_mut(indx as usize) {
            let name = names.get(*ent).unwrap();
            println!("Name: {}", name.name);

            if button == input::mouse::MouseButton::Right {

                let pos = transforms.get(*ent).unwrap().position;

                if name.name == "Dirt" {
                    entities.delete(*ent).expect("Entity Not Found!");
                    *ent = components::create_air(&entities, &lazy, pos);
                } else if name.name == "Grassy Dirt" {
                    entities.delete(*ent).expect("Entity Not Found!");
                    *ent = components::create_air(&entities, &lazy, pos);
                }

            }

        }

        let mut input_handler = self.world.write_resource::<InputHandler>();
        if button == input::mouse::MouseButton::Middle {
            input_handler.mouse_down = false;
        }

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
                transform.position.y = transform.position.y + yrel;

            }


        }
        
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {

        let mut transforms = self.world.write_storage::<components::Transform>();

        let transform = {
            let camera_entity = self.world.read_resource::<systems::ActiveCamera>().entity.unwrap();
            transforms.get_mut(camera_entity).unwrap()
        };

        let delta = y / 10.0;

        transform.scale.x += -delta;
        if transform.scale.x < 0.1 {
            transform.scale.x = 0.1;
        }
        
        transform.scale.y += -delta;
        if transform.scale.y < 0.1 {
            transform.scale.y = 0.1;
        }

        println!("Zoom: ({}, {})", transform.scale.x, transform.scale.y);
    }

     fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        println!("Resized screen to {}, {}", width, height);
        let new_rect = graphics::Rect::new(
            0.0,
            0.0,
            width as f32,
            height as f32,
        );
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();

        let mut screen_size = self.world.write_resource::<systems::ScreenDimensions>();
        screen_size.x = width;
        screen_size.y = height;
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
        .window_setup(conf::WindowSetup::default().title("Sandbox!").samples(ggez::conf::NumSamples::One))
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true),
        )
        //.window_mode(conf::WindowMode::default().dimensions(500.0, 500.0))
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut context_builder.build().unwrap();
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);


    // Create the World and Register Components
    let mut world = World::new();
    world.register::<components::Transform>();
    world.register::<components::Sprite>();
    world.register::<components::Camera>();
    world.register::<components::Tile>();
    world.register::<components::Air>();
    world.register::<components::Dirt>();
    world.register::<components::GrassyDirt>();
    world.register::<components::Culled>();
    world.register::<components::TileSpritesheet>();
    world.register::<components::Named>();

    // Create Camera at Origin
    let player_pos = Point2::new(-32.0 * 5.0, 32.0 * 7.0);
    let camera = world.create_entity()
    .with(components::Transform {
        position : Point2::new(0.0, 0.0),
        scale    : Vector2::new(1.0, 1.0),
    })
    .with(components::Camera {})
    .build();

    // Create Resources
    let active_camera = systems::ActiveCamera{ entity: Some(camera) };
    world.insert(active_camera);
    
    let input_handler = InputHandler { pos_x : 0.0, pos_y : 0.0, mouse_down : false };
    world.insert(input_handler);

    let asset_handler = register_assets(ctx);

    let tile_map : TileMap = Vec::new();

    world.insert(tile_map);

    world.insert(asset_handler);

    world.insert(systems::ScreenDimensions { x: 0.0, y: 0.0 });
    
    create_background(&mut world, Point2::new(player_pos.x, player_pos.y + 1000.0));
    create_player(&mut world, player_pos);

    let state = &mut State::new(world).unwrap();
    event::run(ctx, event_loop, state).unwrap();
}

fn create_player(world : &mut World, position : Point2) {
    let asset_handler = world.write_resource::<crate::AssetHandler>();
    
    world.create_entity_unchecked()
        .with(components::Transform { 
            position,
            scale : Vector2::new(1.0, 1.0) 
        })
        .with(components::Sprite { image :    {
                asset_handler.player.clone() 
            }
        })
        .build();
}

fn create_background(world : &mut World, position : Point2) {
    let asset_handler = world.write_resource::<crate::AssetHandler>();
    
    world.create_entity_unchecked()
        .with(components::Transform {
            position : position, 
            scale : Vector2::new(5.0, 5.0),
        })
        .with(components::Sprite { image :    {
            asset_handler.background.clone() 
        }
    })
    .build();
}

fn register_assets(ctx : &mut Context) -> AssetHandler {

    let dirt_tile = graphics::Image::new(ctx, "/DirtBlock.png" ).unwrap();
    let grassy_dirt_tile = graphics::Image::new(ctx, "/GrassyDirtBlock.png" ).unwrap();
    let spritesheet = graphics::Image::new(ctx, "/tiles_spritesheet.png" ).unwrap();

    let dirt_tile_batch = graphics::spritebatch::SpriteBatch::new(dirt_tile);
    let grassy_dirt_tile_batch = graphics::spritebatch::SpriteBatch::new(grassy_dirt_tile);
    let tile_spritesheet  = graphics::spritebatch::SpriteBatch::new(spritesheet);

    AssetHandler {
        player   : graphics::Image::new(ctx, "/Player.png" ).unwrap(),
        dirt_tile_batch,
        grassy_dirt_tile_batch,
        tile_spritesheet,
        background : graphics::Image::new(ctx, "/background.png").unwrap(),
    }
}