 use svg::{node::element::{Group}, Node};

use crate::{tile::{Tile, ElasticTile}, vec2::Vec2, to_svg::ToSVG};

pub trait Generator {
    type TileType: Tile;

    fn clone_with_brightness(&self, brightness: f32) -> Self;
    fn generator_block_size(&self) -> Vec2<usize>;
    fn source_image_block_size(&self) -> Vec2<usize>;
}

#[derive(Clone)]
pub struct RectGenerator<TTile: Tile> {
    generator_block_size: Vec2<usize>,
    source_image_block_size: Vec2<usize>,
    tiles: Vec<TTile>
}

impl<TTile: Tile> RectGenerator<TTile> {
    pub fn new<const X: usize, const Y: usize>(tiles: [[TTile; Y]; X], source_image_block_size: Vec2<usize>) -> Self { 
        let mut vec = Vec::with_capacity(X * Y);
        vec.extend(tiles.iter().flatten());

        return Self { 
            generator_block_size: Vec2::new(X, Y),
            tiles: vec,
            source_image_block_size
        };
    }
}

impl<TTile: Tile> Generator for RectGenerator<TTile> {
    type TileType = TTile;

    fn clone_with_brightness(&self, brightness: f32) -> Self {
        let mut clone = self.clone();
 
        for tile in &mut clone.tiles {
            tile.set_brightness(brightness);
        }

        return clone;
    }

    #[inline]
    fn generator_block_size(&self) -> Vec2<usize> {
        return self.generator_block_size;
    }

    #[inline]
    fn source_image_block_size(&self) -> Vec2<usize> {
        return self.source_image_block_size;
    }
}

impl<TTile: Tile + ToSVG> ToSVG for RectGenerator<TTile> {
    fn to_svg_node(&self, scale: f32, origin: Vec2<f32>) -> Box<dyn Node> {
        let mut g = Group::new();

        for tile_x in 0..*self.generator_block_size.x() {
            for tile_y in 0..*self.generator_block_size.y() {
                let origin_x = tile_x as f32 * scale + origin.x();
                let origin_y = tile_y as f32 * scale + origin.y();

                let tile = self.tiles[tile_y * self.generator_block_size.x() + tile_x].to_svg_node(scale, Vec2::new(origin_x, origin_y));
                g.append(tile);
            }
        }

        return Box::new(g);
    }
}

pub fn stripes_ac() -> RectGenerator<ElasticTile> {
    return RectGenerator::new([
        [ElasticTile::type_a(), ElasticTile::type_c()],
        [ElasticTile::type_c(), ElasticTile::type_a()]
    ], Vec2::new(8, 8));
}

pub fn stripes_bd() -> RectGenerator<ElasticTile> {
    return RectGenerator::new([
        [ElasticTile::type_b(), ElasticTile::type_d()],
        [ElasticTile::type_d(), ElasticTile::type_b()]
    ], Vec2::new(8, 8));
}

pub fn rand() -> RectGenerator<ElasticTile> {
    return RectGenerator::new([
        [ElasticTile::type_b(), ElasticTile::type_a()],
        [ElasticTile::type_c(), ElasticTile::type_d()]
    ], Vec2::new(8, 8));
}

