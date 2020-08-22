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
pub struct Air;

pub fn create_air(
    entities : &Entities,
    lazy     : &LazyUpdate,
    position : Point2,
) -> Entity {
    lazy
        .create_entity(entities)
        .with(components::Transform { 
            position,
            scale : Vector2::new(0.25, 0.25) 
        })
        .with(Tile)
        .with(Air)
        .with(Named::new("Air"))
        .with(TileSpritesheet{ x : 128.0, y : 0.0})
        .build()
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Dirt;

pub fn create_dirt(
    entities : &Entities,
    lazy     : &LazyUpdate,
    position : Point2,
) -> Entity {
    lazy
        .create_entity(entities)
        .with(components::Transform { 
            position,
            scale : Vector2::new(0.25, 0.25) 
        })
        .with(Tile)
        .with(Dirt)
        .with(Named::new("Dirt"))
        .with(TileSpritesheet{ x : 0.0, y : 0.0})
        .build()
}


#[derive(Component)]
#[storage(VecStorage)]
pub struct GrassyDirt;

pub fn create_grassy_dirt(
    entities : &Entities,
    lazy     : &LazyUpdate,
    position : Point2,
) -> Entity {
    lazy
        .create_entity(entities)
        .with(components::Transform { 
            position,
            scale : Vector2::new(0.25, 0.25) 
        })
        .with(Tile)
        .with(GrassyDirt)
        .with(Named::new("Grassy Dirt"))
        .with(TileSpritesheet{ x : 64.0, y : 0.0})
        .build()
}