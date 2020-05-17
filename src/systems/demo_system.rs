use specs::{WriteStorage, System, SystemData, Join};
use crate::components::{Transform, Sprite};


pub struct DemoSystem {}


impl<'s> System<'s> for DemoSystem {

    type SystemData = WriteStorage<'s, Transform>;

    fn run(&mut self, mut transforms : Self::SystemData) {

        for transform in (&mut transforms).join() {
            let pos_x = &mut transform.position.x;

            match *pos_x {
                x if x <  250.0 => { *pos_x = x + 1.0; },
                x if x >= 250.0 => { *pos_x = -250.0; }
                _ => {}
            }
        }

    }

}