use specs_derive::*;
use specs::{Component, VecStorage};
use ggez::graphics;
use crate::alias::*;


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