use svg::{node::element::{Group, Path, path::Data}, Node};

use crate::{to_svg::ToSVG, vec2::Vec2};

use super::traits::Tile;

#[derive(Clone, Copy)]
pub struct ElasticCircleTile {
    flipped: bool,
    half_width: f32,
    radius: f32
}

impl ElasticCircleTile {
    pub fn new(flipped: bool, width: f32, radius: f32) -> Self {
        let width = width.clamp(0.0, 1.0) / 2.0;
        return Self { flipped, half_width: width, radius };
    }

    pub fn flipped(mut self) -> Self {
        self.flipped = !self.flipped;
        return self;
    }

    fn upper_arc(self) -> (Vec2<f32>, Vec2<f32>) {
        return ( 
            Vec2::new(0.0, 0.5),
            Vec2::new(0.5, 0.0)
        );
    }

    fn lower_arc(self) -> (Vec2<f32>, Vec2<f32>) {
        return (
            Vec2::new(1.0, 0.5),
            Vec2::new(0.5, 1.0)
        );
    }
}

impl Default for ElasticCircleTile {
    #[inline]
    fn default() -> Self {
        return Self::new(false, 0.1, 0.5);
    }
}

impl Tile for ElasticCircleTile {
    #[inline]
    fn set_brightness(&mut self, brightness: f32) {
        self.half_width = (1.0 - brightness) / 4.0;
    }
}

impl ToSVG for ElasticCircleTile {
    fn to_svg_node(&self) -> Box<dyn svg::Node> {
        let (upper_start, upper_end) = self.upper_arc();
        let (lower_start, lower_end) = self.lower_arc();

        let r_plus_hw = self.radius + self.half_width;
        let r_minus_hw = self.radius - self.half_width;

        let c1_data = Data::new()
            .move_to((upper_start.x(), upper_start.y() + self.half_width))
            .elliptical_arc_to((
                r_plus_hw, r_plus_hw,
                0, 0, 0, 
                upper_end.x() + self.half_width, upper_end.y()
            ))
            .line_to((upper_end.x() - self.half_width, upper_end.y()))
            .elliptical_arc_to((
                r_minus_hw, r_minus_hw,
                0, 0, 1, 
                upper_start.x(), upper_start.y() - self.half_width
            ))
            .line_to((upper_start.x(), upper_start.y() + self.half_width));

        let c2_data = Data::new()
            .move_to((lower_start.x(), lower_start.y() - self.half_width))
            .elliptical_arc_to((
                r_plus_hw, r_plus_hw,
                0, 0, 0, 
                lower_end.x() - self.half_width, lower_end.y()
            ))
            .line_to((lower_end.x() + self.half_width, lower_end.y()))
            .elliptical_arc_to((
                r_minus_hw, r_minus_hw,
                0, 0, 1, 
                lower_start.x(), lower_start.y() + self.half_width
            ))
            .line_to((lower_start.x(), lower_start.y() - self.half_width));

        let c1 = Path::new()
            .set("stroke", "black")
            .set("stroke-width", "0.01")
            .set("d", c1_data);


        let c2 = c1.clone().set("d", c2_data);

        let mut g = Group::new()
            .add(c1)
            .add(c2);

        if self.flipped {
            g.assign("transform", "rotate(90, 0.5, 0.5)");
        }

        return Box::new(g);
    }
}
