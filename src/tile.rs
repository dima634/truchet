use svg::{node::element::Polygon, Node};

use crate::{to_svg::ToSVG, vec2::Vec2};

pub trait Tile: Clone + Copy {
    fn set_brightness(&mut self, brightness: f32);
}

#[derive(Clone, Copy)]
pub struct ElasticTile {
    t: f32,
    black_vertex_index: u8
}

impl ElasticTile {
    pub const fn new(t: f32, black_vertex_index: u8) -> Self { 
        return Self { t, black_vertex_index };
    }

    pub const fn with_black_vertex(black_vertex_index: u8) -> Self {
        return Self::new(0.5, black_vertex_index);
    }

    pub const fn type_a() -> Self {
        return Self::with_black_vertex(0);
    }

    pub const fn type_b() -> Self {
        return Self::with_black_vertex(1);
    }

    pub const fn type_c() -> Self {
        return Self::with_black_vertex(2);
    }

    pub const fn type_d() -> Self {
        return Self::with_black_vertex(3);
    }

    fn point(&self) -> Vec2<f32> {
        return  match self.black_vertex_index {
            0 => Vec2::new(1.0 - (0.5  * self.t + 0.25),  1.0 - (0.5 * self.t + 0.25)),
            1 => Vec2::new(1.0 - (0.5 * self.t + 0.25),   (0.5 * self.t + 0.25)),
            2 => Vec2::new((0.5  * self.t + 0.25),  (0.5 * self.t + 0.25)),
            _ => Vec2::new(0.5 * self.t + 0.25,   1.0 - (0.5 * self.t + 0.25)),
        };
    }
}

impl Tile for ElasticTile {
    fn set_brightness(&mut self, brightness: f32) {
        if brightness < 0.25 {
            self.t = 0.0;
        } else if brightness > 0.75 {
            self.t = 1.0;
        } else {
            self.t = 2.0 * brightness - 0.5;
        }
    }
}

impl ToSVG for ElasticTile {
    fn to_svg_node(&self, scale: f32, origin: Vec2<f32>) -> Box<dyn Node> {
        let (mut x, mut y) = match self.black_vertex_index {
            0 => (vec![1_f32, 0_f32, 0_f32], vec![0_f32, 0_f32, 1_f32]),
            1 => (vec![0_f32, 0_f32, 1_f32], vec![0_f32, 1_f32, 1_f32]),
            2 => (vec![0_f32, 1_f32, 1_f32], vec![1_f32, 1_f32, 0_f32]),
            _ => (vec![1_f32, 1_f32, 0_f32], vec![1_f32, 0_f32, 0_f32])
        };

        let t = self.point();
        x.push(*t.x());
        y.push(*t.y());

        let points: Vec<f32> = x.into_iter()
            .map(|x| x * scale + origin.x())
            .zip(
                y.into_iter().map(|y| y * scale + origin.y())
            )
            .map(|(x, y)| [x, y])
            .flatten()
            .collect();

        let polygon = Polygon::new()
            .set("points", points)
            .set("t", self.t)
            .set("type", self.black_vertex_index);

        return Box::new(polygon);
    }
}
