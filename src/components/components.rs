use specs_derive::*;
use specs::{Component, VecStorage};
use ggez::graphics;

type Point2 = ggez::nalgebra::Point2<f32>;
type Vector2   = ggez::nalgebra::Vector2<f32>;


#[derive(Component)]
#[storage(VecStorage)]
pub struct Sprite {
    pub image : graphics::Image,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Transform {
    pub position : Point2,
    pub scale    : Vector2,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Camera {}