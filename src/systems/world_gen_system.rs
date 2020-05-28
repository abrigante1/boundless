use specs::*;
use simdnoise::*;
use crate::alias::*;
use crate::components;

pub struct WorldGenSystem {
    pub world_width  : usize,
    pub world_height : usize,
}


const TILE_SCALE : f32 = 0.25;
const TILE_SIZE : f32 = 64.0;

impl WorldGenSystem {
    fn to_world_coords(&self, x : usize, y : usize) -> (f32, f32) {
        let scaled_size = TILE_SIZE * TILE_SCALE;
        let center = scaled_size * 0.5;

        let (bot_left, bot_right) = (-(self.world_width  as f32 / 2.0) * scaled_size, 
                                     -(self.world_height as f32 / 2.0) * scaled_size);

        
        (bot_left + ((x as f32 * scaled_size) + center), 
         bot_right + ((y as f32 * scaled_size) + center))
    }

    pub fn to_tile_coords(&self, x : f32, y : f32) -> (usize) {
        let scaled_size = TILE_SIZE * TILE_SCALE;
        let center = scaled_size * 0.5;

        let (tile_center_x ,tile_center_y) = ((self.world_width  as f32 / 2.0) * scaled_size, 
                                             (self.world_height as f32 / 2.0) * scaled_size);
                                
        let tile_x1 = (x + tile_center_x) as f32; 
        let tile_x  = (tile_x1 / scaled_size as f32) as usize;


        let tile_y1 = (y + tile_center_y) as f32; 
        let tile_y  = (tile_y1 / scaled_size as f32) as usize;


        println!("tile coords1: {:?} -- tile coords: {:?}", (tile_x1, tile_y1), (tile_x, tile_y));


        (tile_x as usize * self.world_width as usize) + (tile_y as usize)
    }
}

impl<'s> System<'s> for WorldGenSystem {

    type SystemData =  (Entities<'s>,
                        Read<'s, LazyUpdate>,
                        Write<'s, TileMap>);
    
    fn run(&mut self, (mut entities, mut lazy, mut tile_map) : Self::SystemData) {

        let noise = NoiseBuilder::
                        gradient_1d(self.world_width)
                            .generate_scaled((self.world_height / 2) as f32, (self.world_height-1) as f32);

        // Generate the World
        for x in 0..self.world_width {

            let top_tile = noise[x].round() as usize;
            for y in 0..self.world_height {

                let coords = self.to_world_coords(x, y);

                    let (x_pos, y_pos) = coords;

                    let ent : Entity;

                    if y < top_tile {
                       ent = components::create_dirt(&mut entities, &mut lazy, Point2::new(x_pos, y_pos));
                       println!("Dirt Tile Stored At: {}", tile_map.len());
                    } else if y == top_tile {
                        ent = components::create_grassy_dirt(&mut entities, &mut lazy, Point2::new(x_pos, y_pos));
                        println!("Grass Tile Stored At: {}", tile_map.len());
                    } else {
                        ent = components::create_air(&mut entities, &mut lazy, Point2::new(x_pos, y_pos));
                        println!("Air Tile Stored At: {}", tile_map.len());
                    }

                    tile_map.push(ent);
                

            }

        }

    }

}