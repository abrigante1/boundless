use specs::*;
use crate::alias::*;
use crate::components::*;
use crate::systems::ActiveCamera;
use crate::systems::ScreenDimensions;

pub struct PhysicsSystem {}

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (Entities<'s>,
                       Read<'s, ActiveCamera>,
                       Read<'s, ScreenDimensions>,
                       ReadStorage<'s, Transform>,
                       ReadStorage<'s, Tile>,
                       WriteStorage<'s, Culled>);

    fn run(&mut self, (entities, active_camera, screen_size, transforms, tiles, mut culled_ents) : Self::SystemData) {

        // Get Screen Bottom Left and Top Right
        let camera_transform = transforms.get(active_camera.entity.unwrap()).unwrap();
        let camera_size = (screen_size.x * (camera_transform.scale.x + 0.5), screen_size.y * (camera_transform.scale.y + 0.5) );
        
        let bot_left = Point2::new(camera_transform.position.x - (camera_size.0 / 2.0), 
                                   camera_transform.position.y - (camera_size.1 / 2.0));

        let top_right = Point2::new(camera_transform.position.x + (camera_size.0 / 2.0), 
                                    camera_transform.position.y + (camera_size.1 / 2.0));

        for (entity, transform, _tile) in (&entities, &transforms, &tiles).join() {
    
            if bot_left.x  < transform.position.x && bot_left.y  < transform.position.y &&
               top_right.x > transform.position.x && top_right.y > transform.position.y {
                culled_ents.remove(entity);
            } else {
                culled_ents.insert(entity, Culled {}).expect("Could not add 'Culled' Tag");
            }

        }

    }
}