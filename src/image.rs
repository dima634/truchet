use crate::vec2::Vec2;

pub trait Image {
    fn size(&self) -> Vec2<usize>;
    fn get_pixel_brightness(&self, pos: Vec2<usize>) -> f32;
}
