use specs::{WriteStorage, ReadStorage, System, Join};
use crate::components::{Transform, Camera};


pub struct DemoSystem {}


impl<'s> System<'s> for DemoSystem {

    type SystemData = (WriteStorage<'s, Transform>,
                       ReadStorage<'s,  Camera>);

    fn run(&mut self, (mut transforms, cameras) : Self::SystemData) {


        for (transform, _) in (&mut transforms, !&cameras).join() {
            let pos_x = &mut transform.position.x;

            match *pos_x {
                x if x <  400.0 => { *pos_x = x + 2.0; },
                x if x >= 400.0 => { *pos_x = -400.0; }
                _ => {}
            }
        }

    }

}