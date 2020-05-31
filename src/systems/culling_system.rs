use specs::*; // TODO - Fix This Kitchen Sink 
use crate::{
    resources::{
        ActiveCamera,
        ScreenDimensions,
    },
    components
};
use ggez::nalgebra as math;

pub struct CullingSystem;

impl<'s> System<'s> for CullingSystem {
    type SystemData = (Entities<'s>,
                       Read<'s, ActiveCamera>,
                       Read<'s, ScreenDimensions>,
                       ReadStorage<'s,  components::Transform>,
                       WriteStorage<'s, components::Culled>);

    fn run(&mut self, (entities, active_camera, screen_size, transforms, mut culled_ents) : Self::SystemData) {

        // Get the ActiveCamera's View Rect
        let camera_entity    = active_camera.entity.unwrap();
        let camera_transform = transforms.get(camera_entity).unwrap();
        let camera_rect      = camera_transform.get_rect_from_point(math::Point2::new(screen_size.x, screen_size.y));

        // Cull all Entities that whose Position is Not Inside the Camera's View Rect
        for (entity, transform) in (&entities, &transforms).join() {
            if camera_rect.contains(transform.position) {
                culled_ents.remove(entity);
            } else {
                culled_ents.insert(entity, components::Culled {}).expect("Could not add 'Culled' Tag");
            }
        }

    }
}