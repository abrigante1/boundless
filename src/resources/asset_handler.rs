use std::collections::HashMap;
use ggez::{
    graphics,
    graphics::spritebatch::SpriteBatch,
};


// Loads and Managers Game Assets
#[derive(Default)]
pub struct AssetHandler {
    pub asset_list : HashMap<String, SpriteBatch>,
}

impl AssetHandler {
    
    // Construct the Asset Handler
    pub fn new() -> Self {
        Self { asset_list : HashMap::new() }
    }

    // Adds an Item to the Asset Handler
    pub fn add_asset(&mut self, ctx : &mut ggez::Context, asset_name : &str) -> Result<(), String> {

        // Check if the Asset Already Exists and If So Return an Error
        if self.asset_list.contains_key(asset_name) {
            return Err("Asset Already Exists.".to_string())
        }

        // Otherwise Create a SpriteBatch from the SpriteSheet
        let image = graphics::Image::new(ctx, asset_name).unwrap();

        self.asset_list.insert(asset_name.to_string(), SpriteBatch::new(image));

        Ok(())
    }

    // Return the Asset if it Exists, Otherwise Return None
    pub fn get_asset(&mut self, asset_name : &str) -> Option<&mut SpriteBatch> {
        self.asset_list.get_mut(asset_name)
    }
}

