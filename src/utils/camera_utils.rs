use ggez::nalgebra as math;
use specs::Entity;
use crate::components;

pub fn world_to_camera( camera_transform : &components::Transform ) -> math::Matrix3::<f32> {
    math::Matrix3::new(1.0,  0.0, -camera_transform.position.x, 
                       0.0, -1.0,  camera_transform.position.y,
                       0.0,  0.0,  1.0)
}

pub fn camera_to_screen(camera_transform : &components::Transform, screen_size : math::Point2::<f32>) -> math::Matrix3::<f32> {
    let width_scalar  = screen_size.x / (screen_size.x * camera_transform.scale.x);
    let height_scalar = screen_size.y / (screen_size.y * camera_transform.scale.y); 
    
    math::Matrix3::new(width_scalar, 0.0,   screen_size.x / 2.0, 
                       0.0,  height_scalar, screen_size.y / 2.0,
                       0.0,  0.0,    1.0)
}

pub fn world_to_screen(camera_transform : &components::Transform, screen_size : math::Point2::<f32>) -> math::Matrix3::<f32> {
    camera_to_screen(camera_transform, screen_size) * world_to_camera(camera_transform)
}