 use rand::{prelude::Distribution, distributions::Standard};
use svg::{node::element::{Group}, Node};

use crate::{tile::{Tile, ElasticTile}, vec2::Vec2, to_svg::ToSVG};

pub trait Generator {
    type TileType: Tile;

    fn clone_with_brightness(&self, brightness: f32) -> Self;
    fn generator_block_size(&self) -> Vec2<usize>;
    fn source_image_block_size(&self) -> Vec2<usize>;
}

#[derive(Clone)]
pub struct PatternGenerator<TTile: Tile, const GEN_X_SIZE: usize, const GEN_Y_SIZE: usize, const SRC_X_SIZE: usize, const SRC_Y_SIZE: usize> {
    tiles: [[TTile; GEN_Y_SIZE]; GEN_X_SIZE]
}


impl<TTile: Tile, const GEN_X_SIZE: usize, const GEN_Y_SIZE: usize, const SRC_X_SIZE: usize, const SRC_Y_SIZE: usize> 
    PatternGenerator<TTile, GEN_X_SIZE, GEN_Y_SIZE, SRC_X_SIZE, SRC_Y_SIZE>
{
    pub fn new(tiles: [[TTile; GEN_Y_SIZE]; GEN_X_SIZE]) -> Self { 
        return Self { tiles };
    }
}

impl<TTile: Tile, const GEN_X_SIZE: usize, const GEN_Y_SIZE: usize, const SRC_X_SIZE: usize, const SRC_Y_SIZE: usize>
    Generator for
    PatternGenerator<TTile, GEN_X_SIZE, GEN_Y_SIZE, SRC_X_SIZE, SRC_Y_SIZE>
{
    type TileType = TTile;

    fn clone_with_brightness(&self, brightness: f32) -> Self {
        let mut clone = self.clone();
 
        for tile in clone.tiles.iter_mut().flatten()  {
            tile.set_brightness(brightness);
        }

        return clone;
    }

    #[inline]
    fn generator_block_size(&self) -> Vec2<usize> {
        return Vec2::new(GEN_X_SIZE, GEN_Y_SIZE);
    }

    #[inline]
    fn source_image_block_size(&self) -> Vec2<usize> {
        return Vec2::new(SRC_X_SIZE, SRC_Y_SIZE);
    }
}

impl<TTile: Tile + ToSVG, const GEN_X_SIZE: usize, const GEN_Y_SIZE: usize, const SRC_X_SIZE: usize, const SRC_Y_SIZE: usize> 
    ToSVG for 
    PatternGenerator<TTile, GEN_X_SIZE, GEN_Y_SIZE, SRC_X_SIZE, SRC_Y_SIZE>
{
    fn to_svg_node(&self, scale: f32, origin: Vec2<f32>) -> Box<dyn Node> {
        let mut g = Group::new();

        for tile_x in 0..GEN_X_SIZE {
            for tile_y in 0..GEN_Y_SIZE {
                let origin_x = tile_y as f32 * scale + origin.x();
                let origin_y = tile_x as f32 * scale + origin.y();

                let tile = self.tiles[tile_x][tile_y].to_svg_node(scale, Vec2::new(origin_x, origin_y));
                g.append(tile);
            }
        }

        return Box::new(g);
    }
}

pub fn stripes_ac<const IMG_BLOCK_SIZE_X: usize, const IMG_BLOCK_SIZE_Y: usize>() 
    -> PatternGenerator<ElasticTile, 2, 2, IMG_BLOCK_SIZE_X, IMG_BLOCK_SIZE_Y> 
{
    return PatternGenerator::new([
        [ElasticTile::type_a(), ElasticTile::type_c()],
        [ElasticTile::type_c(), ElasticTile::type_a()]
    ]);
}

pub fn stripes_bd<const IMG_BLOCK_SIZE_X: usize, const IMG_BLOCK_SIZE_Y: usize>()
    -> PatternGenerator<ElasticTile, 2, 2, IMG_BLOCK_SIZE_X, IMG_BLOCK_SIZE_Y> 
{
    return PatternGenerator::new([
        [ElasticTile::type_b(), ElasticTile::type_d()],
        [ElasticTile::type_d(), ElasticTile::type_b()]
    ]);
}

pub fn bosh_d<const IMG_BLOCK_SIZE_X: usize, const IMG_BLOCK_SIZE_Y: usize>()
    -> PatternGenerator<ElasticTile, 2, 2, IMG_BLOCK_SIZE_X, IMG_BLOCK_SIZE_Y> 
{
    return PatternGenerator::new([
        [ElasticTile::type_b(), ElasticTile::type_a()],
        [ElasticTile::type_c(), ElasticTile::type_d()]
    ]);
}


pub fn fan<const IMG_BLOCK_SIZE_X: usize, const IMG_BLOCK_SIZE_Y: usize>()
    -> PatternGenerator<ElasticTile, 2, 2, IMG_BLOCK_SIZE_X, IMG_BLOCK_SIZE_Y> 
{
    return PatternGenerator::new([
        [ElasticTile::type_a(), ElasticTile::type_b()],
        [ElasticTile::type_d(), ElasticTile::type_c()]
    ]);
}

#[derive(Clone)]
pub struct RandomGenerator<
    TTile: Tile, 
    const GEN_X_SIZE: usize, const GEN_Y_SIZE: usize, 
    const SRC_X_SIZE: usize, const SRC_Y_SIZE: usize
>(PatternGenerator<TTile, GEN_X_SIZE, GEN_Y_SIZE, SRC_X_SIZE, SRC_Y_SIZE>);


impl<TTile, const GEN_X_SIZE: usize, const GEN_Y_SIZE: usize, const SRC_X_SIZE: usize, const SRC_Y_SIZE: usize>
    Default for
    RandomGenerator<TTile, GEN_X_SIZE, GEN_Y_SIZE, SRC_X_SIZE, SRC_Y_SIZE>
where 
    TTile: Tile,
    Standard: Distribution<TTile>
{
    fn default() -> Self {
        let mut tiles = [[rand::random(); GEN_Y_SIZE]; GEN_X_SIZE];

        for tile in tiles.iter_mut().flatten() {
            *tile = rand::random();
        }

        return Self(PatternGenerator::new(tiles));
    }
}

impl<TTile, const GEN_X_SIZE: usize, const GEN_Y_SIZE: usize, const SRC_X_SIZE: usize, const SRC_Y_SIZE: usize>
    Generator for
    RandomGenerator<TTile, GEN_X_SIZE, GEN_Y_SIZE, SRC_X_SIZE, SRC_Y_SIZE>
where 
    TTile: Tile,
    Standard: Distribution<TTile>
{
    type TileType = TTile;

    fn clone_with_brightness(&self, brightness: f32) -> Self {
        let mut clone = Self::default();

        for tile in clone.0.tiles.iter_mut().flatten() {
            tile.set_brightness(brightness);
        }

        return clone;
    }

    fn generator_block_size(&self) -> Vec2<usize> {
        return self.0.generator_block_size();
    }

    fn source_image_block_size(&self) -> Vec2<usize> {
        return self.0.source_image_block_size();
    }
}

impl<TTile: Tile + ToSVG, const GEN_X_SIZE: usize, const GEN_Y_SIZE: usize, const SRC_X_SIZE: usize, const SRC_Y_SIZE: usize> 
    ToSVG for 
    RandomGenerator<TTile, GEN_X_SIZE, GEN_Y_SIZE, SRC_X_SIZE, SRC_Y_SIZE>
{
    #[inline]
    fn to_svg_node(&self, scale: f32, origin: Vec2<f32>) -> Box<dyn Node> {
        return self.0.to_svg_node(scale, origin);
    }
}

pub fn random<const IMG_BLOCK_SIZE_X: usize, const IMG_BLOCK_SIZE_Y: usize>()
    -> RandomGenerator<ElasticTile, 2, 2, IMG_BLOCK_SIZE_X, IMG_BLOCK_SIZE_Y> 
{
    return RandomGenerator::default();
}

