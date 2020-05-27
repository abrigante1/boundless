use crate::alias::*;
use crate::components::{tiles, tiles::TileType};
use crate::tile_map::{MapTile, CustomTileMap, TileMap2D};
use amethyst::ecs::*;
use amethyst::tiles::{Map, MapStorage, MortonEncoder, TileMap};
use rand::Rng;
use simdnoise::*;

pub struct WorldGenSystem {
    pub world_width: usize,
    pub world_height: usize,
}

impl<'s> System<'s> for WorldGenSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        WriteStorage<'s, TileMap2D>,
    );

    fn run(&mut self, (mut entities, lazy, mut maps): Self::SystemData) {
        for map in (&mut maps).join() {
            let noise = NoiseBuilder::gradient_1d(self.world_width)
                .generate_scaled((self.world_height / 2) as f32, self.world_height as f32);

            // Generate the World
            let mut rng = rand::thread_rng();

            for x in 0..self.world_width {
                let top_tile = noise[x].round() as usize;
                for y in 0..=top_tile {

                    let map_point = map.index_to_tile(uiPoint3::new(x as u32, y as u32, 0));

                    if y == top_tile {
                        tiles::create_grassy_dirt(&mut entities, &lazy, map, map_point, Vector3::new(1.0, 1.0, 1.0));
                    } else {    
                        tiles::create_dirt(&mut entities, &lazy, map, map_point, Vector3::new(1.0, 1.0, 1.0));
                    }


                }

                let odds      = 5;
                let rand_val  = rng.gen_range(1, odds + 1);
                if rand_val == odds && (top_tile <= self.world_height - 5) {
                   let map_point = map.index_to_tile( uiPoint3::new( x as u32, (top_tile + 1) as u32, 0));
                   tiles::create_tree(&mut entities, &lazy, map, map_point, Vector3::new(1.0, 1.0, 1.0));
                }
            }
        }
    }
}
