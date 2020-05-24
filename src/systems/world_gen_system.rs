use amethyst::ecs::*;
use simdnoise::*;
use crate::alias::*;
use crate::tiles;
use rand::Rng;

pub struct WorldGenSystem {
    pub world_width  : usize,
    pub world_height : usize,
}


const TILE_SCALE : f32 = 1.0;
const TILE_SIZE : f32 = 64.0;

impl WorldGenSystem {
    fn get_index(&self, x : usize, y : usize) -> (f32, f32) {
        let scaled_size = TILE_SIZE * TILE_SCALE;
        let center = scaled_size * 0.5;

        let (bot_left, bot_right) = (-(self.world_width  as f32 / 2.0) * scaled_size, 
                                     -(self.world_height as f32 / 2.0) * scaled_size);

        
        (bot_left + ((x as f32 * scaled_size) + center), 
         bot_right + ((y as f32 * scaled_size) + center))
    }
}

impl<'s> System<'s> for WorldGenSystem {

    type SystemData =  Read<'s, LazyUpdate>;
    
    fn run(&mut self, lazy : Self::SystemData) {

        //let tile_map = *data;

        let noise = NoiseBuilder::
                        gradient_1d(self.world_width)
                            .generate_scaled((self.world_height / 2) as f32, self.world_height as f32);

        // Generate the World
        let mut rng = rand::thread_rng();

        for x in 0..self.world_width {

            let top_tile = noise[x].round() as usize;
            for y in 0..=top_tile {

                let coords = self.get_index(x, y);

                lazy.exec(move |world| {
                    let (x_pos, y_pos) = coords;

                    if y == top_tile {
                       tiles::create_grassy_dirt(world, Vector3::new(x_pos, y_pos, 0.0), Vector3::new(1.0, 1.0, 1.0));
                    } else {
                       tiles::create_dirt(world, Vector3::new(x_pos, y_pos, 0.0), Vector3::new(1.0, 1.0, 1.0));
                    }

                });

            }

            let max_val = 5;
            let rand_val = rng.gen_range(1, max_val+1);
            if rand_val == max_val {
                let coords = self.get_index(x, top_tile + 1);

                lazy.exec(move |world| {
                    let (x_pos, y_pos) = coords;
                    tiles::create_tree(world, Vector3::new(x_pos, y_pos, 0.0), Vector3::new(1.0, 1.0, 1.0));
                });
            }

        }

    }

} 