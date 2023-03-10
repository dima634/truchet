use rand::{prelude::Distribution, distributions::Standard};
use svg::{node::element::Polygon, Node};

use crate::{to_svg::ToSVG, vec2::Vec2};

use super::traits::Tile;

#[derive(Clone, Copy)]
pub enum ElasticTileType {
    ///
    /// ```
    /// |*       |
    /// |***     |
    /// |*****   |
    /// |********|
    /// 
    A,
    ///
    /// ```
    /// |********|
    /// |*****   |
    /// |***     |
    /// |*       |
    /// 
    B,
    ///
    /// ```
    /// |********|
    /// |  ******|
    /// |     ***|
    /// |       *|
    /// 
    C,
    ///
    /// ```
    /// |       *|
    /// |     ***|
    /// |  ******|
    /// |********|
    /// 
    D
}

impl ToString for ElasticTileType {
    fn to_string(&self) -> String {
        return match self {
            ElasticTileType::A => "A".to_owned(),
            ElasticTileType::B => "B".to_owned(),
            ElasticTileType::C => "C".to_owned(),
            ElasticTileType::D => "D".to_owned(),
        };
    }
}

#[derive(Clone, Copy)]
pub struct ElasticTriangleTile {
    t: f32,
    tile_type: ElasticTileType
}

impl ElasticTriangleTile {
    pub const fn new(t: f32, tile_type: ElasticTileType) -> Self { 
        return Self { t, tile_type };
    }

    pub const fn type_a() -> Self {
        return Self::new(0.5, ElasticTileType::A);
    }

    pub const fn type_b() -> Self {
        return Self::new(0.5, ElasticTileType::B);
    }

    pub const fn type_c() -> Self {
        return Self::new(0.5, ElasticTileType::C);
    }

    pub const fn type_d() -> Self {
        return Self::new(0.5, ElasticTileType::D);
    }

    fn point(&self) -> Vec2<f32> {
        return  match self.tile_type {
            ElasticTileType::A => Vec2::new(1.0 - (0.5  * self.t + 0.25),  0.5 * self.t + 0.25),
            ElasticTileType::B => Vec2::new(1.0 - (0.5 * self.t + 0.25),   1.0 - (0.5 * self.t + 0.25)),
            ElasticTileType::C => Vec2::new(0.5  * self.t + 0.25,  1.0 - (0.5 * self.t + 0.25)),
            ElasticTileType::D => Vec2::new(0.5 * self.t + 0.25,  0.5 * self.t + 0.25),
        };
    }
}

impl Default for ElasticTriangleTile {
    #[inline]
    fn default() -> Self {
        return Self::type_a();
    }
}

impl Tile for ElasticTriangleTile {
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

impl ToSVG for ElasticTriangleTile {
    fn to_svg_node(&self) -> Box<dyn Node> {
        let (mut x, mut y) = match self.tile_type {
            ElasticTileType::A => (vec![1_f32, 0_f32, 0_f32], vec![1_f32, 1_f32, 0_f32]),
            ElasticTileType::B => (vec![0_f32, 0_f32, 1_f32], vec![1_f32, 0_f32, 0_f32]),
            ElasticTileType::C => (vec![0_f32, 1_f32, 1_f32], vec![0_f32, 0_f32, 1_f32]),
            ElasticTileType::D => (vec![1_f32, 1_f32, 0_f32], vec![0_f32, 1_f32, 1_f32])
        };

        let t = self.point();
        x.push(t.x());
        y.push(t.y());

        let points: Vec<f32> = x.into_iter()
            .zip(
                y.into_iter()
            )
            .flat_map(|(x, y)| [x, y])
            .collect();

        let polygon = Polygon::new()
            .set("points", points)
            .set("t", self.t)
            .set("type", self.tile_type.to_string());

        return Box::new(polygon);
    }
}

impl Distribution<ElasticTriangleTile> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ElasticTriangleTile {
        match rng.gen_range(0..=3) {
            0 => ElasticTriangleTile::type_a(),
            1 => ElasticTriangleTile::type_b(),
            2 => ElasticTriangleTile::type_c(),
            _ => ElasticTriangleTile::type_d()
        }
    }
}
