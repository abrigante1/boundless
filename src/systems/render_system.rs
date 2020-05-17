use specs::{self, Join, World, WorldExt};
use ggez::graphics;
use ggez::nalgebra as math;

use crate::components::{Transform, Sprite};

type Point2 = math::Point2<f32>;

pub struct RenderSystem {
    pub screen_size : Point2,
}

impl RenderSystem {
    
    pub fn draw(&mut self, ctx : &mut ggez::Context, world: &mut World) {

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let transforms = world.read_storage::<Transform>();
        let sprites    = world.read_storage::<Sprite>();

        for (transform, sprite) in (&transforms, &sprites).join() {

            let screen_pos = self.world_to_screen_coords(transform.position);

            let draw_params = graphics::DrawParam::new()
                .dest(screen_pos)
                .scale(transform.scale)
                .offset(Point2::new(0.5, 0.5)); // Moves origin to center of image

            graphics::draw(ctx, &sprite.image, draw_params).expect("Failed to load Image!");
        }

        graphics::present(ctx).expect("Failed to present!");
    }
    
    
    fn world_to_screen_coords(&mut self, point: Point2) -> Point2 {

        let screen_width = self.screen_size.x;
        let screen_height = self.screen_size.y;

        let x = point.x + (screen_width / 2.0);
        let y = screen_height - (point.y + screen_height / 2.0);

        Point2::new(x, y)
    }

    
}


