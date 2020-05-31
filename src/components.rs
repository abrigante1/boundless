use specs::{Component, VecStorage};
use specs_derive::*;
use ggez::nalgebra as math;
use ggez::graphics::Rect;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet_dir: std::borrow::Cow<'static, str>,
    pub x_offset : u32,
    pub y_offset : u32,
    pub width    : u32,
    pub height   : u32,
}

impl Sprite {

    pub fn default() -> Self {
        Self {
            spritesheet_dir : "DefaultSpriteSheet".into(),
            x_offset : 0,
            y_offset : 0,
            width    : 64,
            height   : 64,
        }
    }

    pub fn tile() -> Self {
        Self { 
            spritesheet_dir : "/tiles_spritesheet.png".into(),
            x_offset  : 0,
            y_offset  : 0,
            width     : 64,
            height    : 64,
        }
    }

    pub fn dirt() -> Self {
        Self { 
            ..Sprite::tile()
        }
    }

    pub fn grass() -> Self {
        Self {
            x_offset : 64,
            ..Sprite::dirt()
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Transform {
    pub position : math::Point2::<f32>,
    pub scale    : math::Vector2::<f32>,
}

impl Transform {
    
    #[allow(dead_code)]
    pub fn get_rect_from_sprite(&self, sprite : &Sprite ) -> Rect {
        let mut rect = Rect::new(0.0, 0.0, sprite.width as f32, sprite.height as f32);
        rect.scale(self.scale.x, self.scale.y);
        rect.translate(math::Vector2::new(self.position.x - (rect.w / 2.0), self.position.y - (rect.h / 2.0)));

        rect
    }

    pub fn get_rect_from_point(&self, point : math::Point2::<f32>) -> Rect {
        let mut rect = Rect::new(0.0, 0.0, point.x as f32, point.y as f32);
        rect.scale(self.scale.x, self.scale.y);
        rect.translate(math::Vector2::new(self.position.x - (rect.w / 2.0), self.position.y - (rect.h / 2.0)));

        rect
    }

}


#[derive(Component)]
#[storage(VecStorage)]
pub struct Camera;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Culled;