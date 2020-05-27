use crate::alias::*;
use crate::components::tiles;
use amethyst::{
    ecs::*,
    assets::Handle,
    renderer::SpriteSheet,
};
use amethyst::tiles::{MortonEncoder, RenderTiles2D, Tile as TileTrait, TileMap, Map, MapStorage};

pub type TileMap2D = amethyst::tiles::TileMap<MapTile, amethyst::tiles::MortonEncoder>;

pub trait CustomTileMap {
    fn index_to_tile(&self, map_coords : uiPoint3 ) -> uiPoint3;

    fn to_world_coords(&self, map_coords : &uiPoint3 ) -> Vector3;

    fn to_tile_coords(&self, world_coords : &Vector3 ) -> uiPoint3;
}

impl CustomTileMap for TileMap2D {
    fn index_to_tile(&self, map_coords : uiPoint3 ) -> uiPoint3 {
      uiPoint3::new(map_coords.x, self.dimensions().y - map_coords.y, map_coords.z)
    }

    fn to_world_coords(&self, map_coords : &uiPoint3 ) -> Vector3 {
        self.to_world(&map_coords, None)
    }

    fn to_tile_coords(&self, world_coords : &Vector3 ) -> uiPoint3 {
        self.to_tile(&world_coords, None).expect("Error - Tile Not In Map!")
    }

}

#[derive(Clone)]
pub struct MapTile {
    pub tile_type: tiles::TileType,
    pub entity: Option<Entity>,
}

impl Default for MapTile {
    fn default() -> Self {
        MapTile {
            tile_type: tiles::TileType::Air,
            entity: None,
        }
    }
}

impl TileTrait for MapTile {
    fn sprite(&self, _coordinates: uiPoint3, _world: &World) -> Option<usize> {
        Some(self.tile_type.get_index())
    }
}
