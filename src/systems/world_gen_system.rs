 use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::core::math::Vector3;
use amethyst::ecs::*;
use amethyst::renderer::{SpriteSheet, SpriteRender};
use crate::components::{Tile, NewTile, Dirt};
use crate::TileSpritesheet;
use simdnoise::*;

pub struct WorldGenSystem {
    pub world_width : usize,
    pub world_height : usize,
}

const TILE_SCALE : f32 = 0.5;
const TILE_SIZE : f32 = 32.0;

impl WorldGenSystem {

    pub fn create_world(&self, world : &mut World) {

        let noise = NoiseBuilder::
                        gradient_1d(self.world_width)
                            .generate_scaled((self.world_height / 3) as f32, self.world_height as f32);

        // Generate the World
        for x in 0..self.world_width {

            for y in 0..(noise[x].round() as usize) {

                let mut transform = Transform::default();
                transform.set_scale(Vector3::new(TILE_SCALE, TILE_SCALE, 1.0));

                let (x_pos, y_pos) = WorldGenSystem::get_index(x, y);
                transform.set_translation_xyz(x_pos, y_pos, 0.0);

                // Place Dirt Block at the Specified Location
                let spritesheet = (&*world.read_resource::<TileSpritesheet>()).spritesheet.clone();

                world
                    .create_entity()
                    .with(transform)
                    .with(SpriteRender {
                        sprite_sheet   : spritesheet,
                        sprite_number : 0,
                    })
                    .build();

            }

        }
    }

    fn get_index( x : usize, y : usize) -> (f32, f32) {
        let scaled_size = TILE_SIZE * TILE_SCALE;
        let center = scaled_size * 0.5;

        ((x as f32 * scaled_size) + center, (y as f32 * scaled_size) + center)
    }
}