use svg::Node;

use crate::vec2::Vec2;

pub trait ToSVG {
    fn to_svg_node(&self, scale: f32, origin: Vec2<f32>) -> Box<dyn Node>;
}
