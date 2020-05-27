use crate::alias::*;
use amethyst::core::{Parent, Transform};
use amethyst::ecs::*;
use amethyst::renderer::SpriteRender;
use crate::tile_map::{MapTile, TileMap2D, CustomTileMap};

#[derive(Clone, PartialEq)]
pub enum TileType {
    Air = 0,
    Dirt,
    GrassyDirt,
    Target,
    Tree,
    TreeLeaves,
    TreeTrunk,
}

impl TileType {
    pub fn get_index(&self) -> usize {
        use TileType::*;

        match *self {
            Air => Air as usize,
            Dirt => 1,
            GrassyDirt => 2,
            Target => 3,
            Tree => 5,
            TreeLeaves => 4,
            TreeTrunk => 5,
        }
    }
}

#[derive(Component)]
pub struct NewTile;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Dirt;

pub fn create_dirt(
    entities : &mut Entities,
    lazy     : &LazyUpdate,
    map      : &mut TileMap2D,
    map_pos  : uiPoint3,
    scale    : Vector3,
) -> Entity {

    let position = map.to_world_coords(&map_pos);

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    let entity = lazy
        .create_entity(entities)
        .with(transform)
        .with(Tile)
        .with(Dirt)
        .build();

    let tile = map.get_mut(&map_pos).expect("Error - No Tile Found!");
    tile.tile_type = TileType::Dirt;
    tile.entity    = Some(entity);

    entity
}

#[derive(Component)]
pub struct GrassyDirt;

pub fn create_grassy_dirt(
    entities : &mut Entities,
    lazy     : &LazyUpdate,
    map      : &mut TileMap2D,
    map_pos  : uiPoint3,
    scale    : Vector3,
) -> Entity {

    let position = map.to_world_coords(&map_pos);

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    let entity = lazy
        .create_entity(entities)
        .with(transform)
        .with(Tile)
        .with(GrassyDirt)
        .build();

    let tile = map.get_mut(&map_pos).expect("Error - No Tile Found!");
    tile.tile_type = TileType::GrassyDirt;
    tile.entity    = Some(entity);

    entity
}

#[derive(Component)]
pub struct TreeTrunk;

fn create_tree_trunk(
    entities : &mut Entities,
    lazy     : &LazyUpdate,
    map      : &mut TileMap2D,
    map_pos  : uiPoint3,
    scale    : Vector3,
    parent   : Entity,
) -> Entity {

    let position = map.to_world_coords(&map_pos);

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    let entity = lazy
        .create_entity(entities)
        .with(transform)
        .with(Tile)
        .with(TreeTrunk)
        .with(Parent { entity: parent })
        .build();

    let tile = map.get_mut(&map_pos).expect("Error - No Tile Found!");
    tile.tile_type = TileType::TreeTrunk;
    tile.entity    = Some(entity);

    entity
}

#[derive(Component)]
pub struct TreeLeaves;

fn create_tree_leaves(
    entities : &mut Entities,
    lazy     : &LazyUpdate,
    map      : &mut TileMap2D,
    map_pos  : uiPoint3,
    scale    : Vector3,
    parent   : Entity,
) -> Entity {

    let position = map.to_world_coords(&map_pos);

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    let entity = lazy
        .create_entity(entities)
        .with(transform)
        .with(Tile)
        .with(TreeLeaves)
        .with(Parent { entity: parent })
        .build();

    let tile = map.get_mut(&map_pos).expect("Error - No Tile Found!");
    tile.tile_type = TileType::TreeLeaves;
    tile.entity    = Some(entity);

    entity
}

#[derive(Component)]
pub struct Tree;

pub fn create_tree(
    entities : &mut Entities,
    lazy     : &LazyUpdate,
    map      : &mut TileMap2D,
    map_pos  : uiPoint3,
    scale    : Vector3,
) -> Entity {

    let position = map.to_world_coords(&map_pos);

    let mut transform = Transform::default();
    transform.set_translation(position);
    transform.set_scale(scale);

    let tree = lazy
        .create_entity(entities)
        .with(transform)
        .with(Tile)
        .with(Tree)
        .build();

    let tile = map.get_mut(&map_pos).expect("Error - No Tile Found!");
    tile.tile_type = TileType::Tree;
    tile.entity    = Some(tree);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let tree_size = rng.gen_range(3, map_pos.y);

    for i in 0..tree_size {
        create_tree_trunk(
            entities,
            lazy,
            map,
            uiPoint3::new(map_pos.x, map_pos.y - i, map_pos.z),
            Vector3::new(1.0, 1.0, 2.0),
            tree,
        );
    }

    create_tree_leaves(
        entities,
        lazy,
        map,
        uiPoint3::new(map_pos.x, map_pos.y - tree_size, map_pos.z),
        Vector3::new(10.0, 10.0, 1.0),
        tree,
    );

    tree
}
