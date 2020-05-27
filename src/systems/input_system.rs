use amethyst::{
    core::shrev::{EventChannel, ReaderId},
    core::Transform,
    ecs::{Read, System, SystemData, Write, WriteStorage},
    input::{InputEvent, ScrollDirection, StringBindings},
    prelude::*,
    renderer::ActiveCamera,
    winit::MouseButton,
};

#[derive(Default)]
pub struct InputManager {
    pub left_click_pressed: bool,
    pub left_click_down: bool,
}

impl InputManager {
    pub fn frame_reset(&mut self) {
        self.left_click_pressed = false;
    }
}

pub struct GodCameraSystem {
    pub reader_id: Option<ReaderId<InputEvent<StringBindings>>>,
}

impl<'s> System<'s> for GodCameraSystem {
    type SystemData = (
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Write<'s, InputManager>,
        Write<'s, ActiveCamera>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (input, mut input_manager, active_camera, mut transform_storage): Self::SystemData,
    ) {
        // Get the Entity from the Active Camera
        let entity = active_camera
            .entity
            .expect("Active Camera Doesn't Have A Target!");

        // Get the Camera Component from the Storage
        let transform = transform_storage
            .get_mut(entity)
            .expect("The Entity Doesn't have a Transform!");

        for event in input.read(&mut self.reader_id.as_mut().unwrap()) {
            match *event {
                InputEvent::MouseButtonPressed(mouse_button) => {
                    if let MouseButton::Left = mouse_button {
                        input_manager.left_click_down = true;
                        input_manager.left_click_pressed = true;
                    }
                }
                InputEvent::MouseButtonReleased(mouse_button) => {
                    if let MouseButton::Left = mouse_button {
                        input_manager.left_click_down = false;
                    }
                }
                InputEvent::MouseMoved { delta_x, delta_y } => {
                    if input_manager.left_click_down {
                        transform.set_translation_xyz(
                            transform.translation().x - delta_x,
                            transform.translation().y + delta_y,
                            1.0,
                        );
                    }
                }
                InputEvent::MouseWheelMoved(scroll_dir) => {
                    let mut scale = transform.scale().clone();

                    if let ScrollDirection::ScrollUp = scroll_dir {
                        scale.x -= 0.1;
                        if scale.x < 0.1 {
                            scale.x = 0.1;
                        }

                        scale.y -= 0.1;
                        if scale.y < 0.1 {
                            scale.y = 0.1;
                        }
                    } else if let ScrollDirection::ScrollDown = scroll_dir {
                        scale.x += 0.1;
                        scale.y += 0.1;
                    }

                    transform.set_scale(scale);
                }
                _ => {}
            }
        }

        input_manager.frame_reset();
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        world.insert(InputManager::default());

        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        );
    }
}
