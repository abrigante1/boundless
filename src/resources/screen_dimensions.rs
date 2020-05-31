use ggez::graphics;

#[derive(Default)]
pub struct ScreenDimensions {
    pub x : f32,
    pub y : f32,
    pub rect : graphics::Rect,
}