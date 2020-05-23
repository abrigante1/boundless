use specs::{self, Join, World, WorldExt};
use ggez::graphics;
use ggez::nalgebra as math;

use math::{Matrix3};

use crate::components::{Transform, Sprite};
use crate::alias::*;




pub struct ScreenDimensions {
    pub x : f32,
    pub y : f32,
}

pub struct ActiveCamera {
    pub entity : Option<specs::Entity>,
}


pub struct RenderSystem {
}

impl RenderSystem {
    
     pub fn draw(&mut self, ctx : &mut ggez::Context, world: &World) {

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let transforms  = world.read_storage::<Transform>();
        let sprites     = world.read_storage::<Sprite>();

        let (w, h) = graphics::drawable_size(ctx);

        // Get the Camera's Transform
        let active_camera    = world.read_resource::<ActiveCamera>();
        let camera_transform = transforms.get(active_camera.entity.unwrap()).unwrap();

        for (transform, sprite) in (&transforms, &sprites).join() {

            let screen_pos = self.world_to_screen_coords(Point2::new(w, h), camera_transform, transform.position);

            let draw_params = graphics::DrawParam::new()
                .offset(Point2::new(0.5, 0.5)) // Moves origin to center of image
                .scale(Vector2::new(transform.scale.x * (1.0/camera_transform.scale.x), transform.scale.y * (1.0/camera_transform.scale.y)))
                .dest(screen_pos);

            graphics::draw(ctx, &sprite.image, draw_params).expect("Failed to load Image!");
        }

        graphics::present(ctx).expect("Failed to present!");
    }
    
    
    fn world_to_screen_coords(&mut self, screen_size : Point2, camera_transform : &Transform , point : Point2) -> Point2 {

        let width_scalar  = screen_size.x / (screen_size.x * camera_transform.scale.x);
        let height_scalar = screen_size.y / (screen_size.y * camera_transform.scale.y); 

        // Construct Matrixes
        let world2camera = Matrix3::new(width_scalar, 0.0, -camera_transform.position.x, 
                                        0.0, -height_scalar, camera_transform.position.y,
                                        0.0, 0.0,  1.0);

        let camera2screen = Matrix3::new(1.0, 0.0, (screen_size.x / 2.0), 
                                         0.0, 1.0, (screen_size.y / 2.0),
                                         0.0, 0.0,  1.0);

                                


        let dude = Point3::new(point.x, point.y, 1.0);
        let pos = camera2screen * world2camera * dude;

        Point2::new(pos.x, pos.y)
    }

    
}


