use specs::{self, Join, World, WorldExt};
use ggez::graphics;
use ggez::nalgebra as math;

use math::{Matrix3};

use crate::components::{Transform, Sprite, Culled, Dirt, GrassyDirt, TileSpritesheet};
use crate::alias::*;



#[derive(Default)]
pub struct ScreenDimensions {
    pub x : f32,
    pub y : f32,
}

#[derive(Default)]
pub struct ActiveCamera {
    pub entity : Option<specs::Entity>,
}


pub struct RenderSystem {
}

impl RenderSystem {
    
    /// Renders entire frame based on all world information.
     pub fn draw(&mut self, ctx : &mut ggez::Context, world: &World) {

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let transforms       = world.read_storage::<Transform>();
        let sprites          = world.read_storage::<Sprite>();
        let culled           = world.read_storage::<Culled>();
        let tile_spritesheet = world.read_storage::<TileSpritesheet>();
        

        let (w, h) = graphics::drawable_size(ctx);

        let mut asset_handler   = world.write_resource::<crate::AssetHandler>();
        
        // Get the Camera's Transform
        let active_camera    = world.read_resource::<ActiveCamera>();
        let camera_transform = transforms.get(active_camera.entity.unwrap()).unwrap();

        let spritesheet_rect = graphics::Rect::new(0.0, 0.0, 64.0 * 4.0, 64.0 * 1.0);

        for (transform, sprite, _) in (&transforms, &sprites, !&culled).join() {

            let screen_pos = self.world_to_screen_coords(Point2::new(w, h), camera_transform, transform.position);

            let draw_params = graphics::DrawParam::new()
                .offset(Point2::new(0.5, 0.5)) // Moves origin to center of image
                .scale(Vector2::new(transform.scale.x  * (1.0/camera_transform.scale.x), transform.scale.y * (1.0/camera_transform.scale.y)))
                .dest(screen_pos);

            graphics::draw(ctx, &sprite.image, draw_params).expect("Failed to load Image!");
        }

        for (transform, tile_spritesheet, _) in (&transforms,  &tile_spritesheet, !&culled).join() {

            let TileSpritesheet { x, y } = tile_spritesheet;

            let screen_pos = self.world_to_screen_coords(Point2::new(w, h), camera_transform, transform.position);

            let draw_params = graphics::DrawParam::new()
                .src(graphics::Rect::fraction(*x, *y, 64.0, 64.0, &spritesheet_rect))
                .offset(Point2::new(0.5, 0.5)) // Moves origin to center of image
                .scale(Vector2::new(transform.scale.x  * (1.0/camera_transform.scale.x), transform.scale.y * (1.0/camera_transform.scale.y)))
                .dest(screen_pos);

            asset_handler.tile_spritesheet.add(draw_params);            
        }
        graphics::draw(ctx, &asset_handler.tile_spritesheet, graphics::DrawParam::new()).unwrap();
        asset_handler.tile_spritesheet.clear();


        graphics::present(ctx).expect("Failed to present!");
    }
    
    /// Take in-game position (pixels) and convert into pixel location within game window.
    pub fn world_to_screen_coords(&mut self, screen_size : Point2, camera_transform : &Transform , point : Point2) -> Point2 {

        let width_scalar  = screen_size.x / (screen_size.x * camera_transform.scale.x);
        let height_scalar = screen_size.y / (screen_size.y * camera_transform.scale.y); 

        // Construct Matrixes
        let world2camera = Matrix3::new(1.0,  0.0, -camera_transform.position.x, 
                                        0.0, -1.0, camera_transform.position.y,
                                        0.0,  0.0,  1.0);

        let camera2screen = Matrix3::new(width_scalar, 0.0,  screen_size.x / 2.0, 
                                         0.0, height_scalar, screen_size.y / 2.0,
                                         0.0, 0.0,  1.0);

        let pos = camera2screen * world2camera * Point3::new(point.x, point.y, 1.0);

        Point2::new(pos.x, pos.y)
    }
    
    /// Take pixel location in game window and convert to in-game position (pixels).
    pub fn screen_to_world_coords(screen_size : Point2, camera_transform : &Transform , point : Point2) -> Point2 {

        let width_scalar  = screen_size.x / (screen_size.x * camera_transform.scale.x);
        let height_scalar = screen_size.y / (screen_size.y * camera_transform.scale.y); 

        // Construct Matrixes
        let world2camera = Matrix3::new(1.0,  0.0, -camera_transform.position.x, 
                                        0.0, -1.0, camera_transform.position.y,
                                        0.0,  0.0,  1.0);

        let camera2screen = Matrix3::new(width_scalar, 0.0,  screen_size.x / 2.0, 
                                         0.0, height_scalar, screen_size.y / 2.0,
                                         0.0, 0.0,  1.0);

        use math::inverse;

        let matrix = camera2screen * world2camera;

        let pos = matrix.try_inverse().unwrap() * Point3::new(point.x, point.y, 1.0);

        Point2::new(pos.x, pos.y)
    }


    
}


