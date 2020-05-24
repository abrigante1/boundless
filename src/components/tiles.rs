use amethyst::ecs::*; 
use amethyst::core::Transform;
use amethyst::renderer::{SpriteSheet, SpriteRender};
use crate::alias::*;

#[derive(Component)]
pub struct NewTile;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Dirt;

pub fn create_dirt(world : &mut World, position : Vector3, scale : Vector3) -> Entity {

    let spritesheet = (&*world.read_resource::<crate::SpriteSheetManager>()).tiles.clone();
    
    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    world.create_entity_unchecked()
        .with(transform)
        .with(Tile)
        .with(Dirt)
        .with(SpriteRender {
            sprite_sheet  : spritesheet,
            sprite_number : 0,
        })
        .build()
}

#[derive(Component)]
pub struct GrassyDirt;

pub fn create_grassy_dirt(world : &mut World, position : Vector3, scale : Vector3) -> Entity {

    let spritesheet = (&*world.read_resource::<crate::SpriteSheetManager>()).tiles.clone();

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    world.create_entity_unchecked()
        .with(transform)
        .with(Tile)
        .with(GrassyDirt)
        .with(SpriteRender {
            sprite_sheet  : spritesheet,
            sprite_number : 1,
        })
        .build()
}


