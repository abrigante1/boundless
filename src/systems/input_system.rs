use amethyst::{
    prelude::*,
    input::{InputHandler, VirtualKeyCode, StringBindings},
    winit::{MouseButton},
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    renderer::{ActiveCamera, Camera},
    ecs::{Read, Write, WriteStorage, System, SystemData, },
};

#[derive(SystemDesc)]
pub struct GodCameraSystem;

impl<'s> System<'s> for GodCameraSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>,
                       Write<'s, ActiveCamera>,
                       WriteStorage<'s, Transform>);


    fn run(&mut self, (input, active_camera, mut transform_storage) : Self::SystemData) {

        if let Some((x, y)) = input.mouse_position() {

            // Get the Entity from the Active Camera
            let entity = active_camera.entity.expect("Active Camera Doesn't Have A Target!");

            // Get the Camera Component from the Storage
            let transform = transform_storage.get_mut(entity).expect("The Entity Doesn't have a Transform!");

            
            transform.set_translation_xyz(x, y, 1.0);

        }

    }

}