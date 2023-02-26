pub trait Tile: Default + Clone + Copy {
    fn set_brightness(&mut self, brightness: f32);
}
