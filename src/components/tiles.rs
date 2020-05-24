use amethyst::ecs::*; 
use amethyst::core::{Transform, Parent};
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

#[derive(Component)]
pub struct TreeTrunk;

fn create_tree_trunk(world : &mut World, position : Vector3, scale : Vector3, parent : Entity) -> Entity {
    let spritesheet = (&*world.read_resource::<crate::SpriteSheetManager>()).tiles.clone();

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    world.create_entity_unchecked()
        .with(transform)
        .with(Tile)
        .with(TreeTrunk)
        .with(Parent { entity : parent })
        .with(SpriteRender {
            sprite_sheet  : spritesheet,
            sprite_number : 4,
        })
        .build()
}

#[derive(Component)]
pub struct TreeLeaves;

fn create_tree_leaves(world : &mut World, position : Vector3, scale : Vector3, parent : Entity) -> Entity {
    let spritesheet = (&*world.read_resource::<crate::SpriteSheetManager>()).tiles.clone();

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    world.create_entity_unchecked()
        .with(transform)
        .with(Tile)
        .with(TreeTrunk)
        .with(Parent { entity : parent })
        .with(SpriteRender {
            sprite_sheet  : spritesheet,
            sprite_number : 3,
        })
        .build()
}

#[derive(Component)]
pub struct Tree;

pub fn create_tree(world : &mut World, position : Vector3, scale : Vector3) -> Entity {

    let spritesheet = (&*world.read_resource::<crate::SpriteSheetManager>()).tiles.clone();

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    let tree = world.create_entity_unchecked()
        .with(transform)
        .with(Tile)
        .with(Tree)
        .build();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let tree_size = rng.gen_range(6, 11);

    for i in 0..tree_size {
        create_tree_trunk(world, Vector3::new(0.0, 64.0 * i as f32, 0.0), Vector3::new(1.0, 1.0, 2.0), tree);
    }

    create_tree_leaves(world, Vector3::new(0.0, 64.0 * (tree_size) as f32, 0.0), Vector3::new(1.0, 1.0, 1.0), tree);
    
    tree
}

