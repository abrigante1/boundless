use specs_derive::*;
use specs::*;
use ggez::graphics;
use ggez::nalgebra as math;
use crate::components::*;

type Point2  = math::Point2<f32>;
type Vector2 = math::Vector2<f32>;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Tile;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Dirt;

pub fn create_dirt(world : &mut World, position : Point2) -> Entity {
    let asset_handler = world.write_resource::<crate::AssetHandler>();
    
    world.create_entity_unchecked()
        .with(components::Transform { 
            position,
            scale : Vector2::new(0.25, 0.5) 
        })
        .with(Tile)
        .with(Dirt)
        .build()
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct GrassyDirt;

pub fn create_grassy_dirt(world : &mut World, position : Point2) -> Entity {
    let asset_handler = world.write_resource::<crate::AssetHandler>();
    
    world.create_entity_unchecked()
        .with(components::Transform { 
            position,
            scale : Vector2::new(0.25, 0.25) 
        })
        .with(Tile)
        .with(GrassyDirt)
        .build()
}