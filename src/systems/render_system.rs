use specs::{Join, World, WorldExt};
use ggez::{
    graphics,
};
use crate::utils::{
    camera_utils,
};
use ggez::nalgebra as math;

// Renders the Current Scene
pub struct RenderSystem {}

impl RenderSystem {
    pub fn draw(&mut self, ctx : &mut ggez::Context, world: &World) {
        
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let (w, h) = graphics::drawable_size(ctx);

        let sprites           = world.read_storage::<crate::components::Sprite>();
        let transforms        = world.read_storage::<crate::components::Transform>(); 
        let culled_ents       = world.read_storage::<crate::components::Culled>();

        let camera            = world.read_resource::<crate::resources::ActiveCamera>().entity;
        let camera_transform  = transforms.get(camera.unwrap()).unwrap();

        let mut asset_handler = world.write_resource::<crate::AssetHandler>();

        // Add All Visible Sprites to the Batch
        for (sprite, transform, _) in (&sprites, &transforms, !&culled_ents).join() {

            let spritesheet = asset_handler.get_asset(&sprite.spritesheet_dir).unwrap();

            let rect = spritesheet.clone().into_inner().dimensions();

            let screen_pos = camera_utils::world_to_screen(&camera_transform,math::Point2::new(w, h))  
                             * math::Point3::new(transform.position.x, transform.position.y, 1.0);

            // Creates the Draw Params that selects the image from the SpriteSheet and Scales it Appropriately
            let draw_params = graphics::DrawParam::new()
                .src(graphics::Rect::fraction(sprite.x_offset as f32, sprite.y_offset as f32, sprite.width as f32, sprite.height as f32, &rect))
                .offset(math::Point2::new(0.5, 0.5))
                .scale(math::Vector2::new(1.0, 1.0))
                .dest(math::Point2::new(screen_pos.x, screen_pos.y));

            spritesheet.add(draw_params);
        }

        // Draw the Batch
        for ( _ , spritesheet) in &mut asset_handler.asset_list {
            graphics::draw(ctx, spritesheet, graphics::DrawParam::new()).expect("Failed to load Image!");
            spritesheet.clear();
        }

        graphics::present(ctx).expect("Failed to present!");

    }
}
