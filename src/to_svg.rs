use svg::Node;

pub trait ToSVG {
    fn to_svg_node(&self) -> Box<dyn Node>;
}
