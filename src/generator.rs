use rand::{prelude::Distribution, distributions::Standard};
use svg::{node::element::{Group}, Node};

use crate::{vec2::Vec2, to_svg::ToSVG, utils::flatten_2d_index, tile::{traits::Tile, triangle::ElasticTriangleTile, circle::ElasticCircleTile}};

pub trait Generator {
    type TileType: Tile;

    fn clone_with_brightness(&self, brightness: f32) -> Self;
    fn generator_block_size(&self) -> Vec2<usize>;
    fn source_image_block_size(&self) -> Vec2<usize>;
}

#[derive(Clone)]
pub struct PatternGenerator<TTile: Tile> {
    gen_size: Vec2<usize>,
    src_img_block_size: Vec2<usize>,
    tiles: Vec<TTile>
}


impl<TTile: Tile> PatternGenerator<TTile> {
    pub fn from_slice<const GEN_X_SIZE: usize, const GEN_Y_SIZE: usize>(tiles: [[TTile; GEN_Y_SIZE]; GEN_X_SIZE], src_img_block_size: Vec2<usize>) -> Self {
        let mut vectorized = Vec::with_capacity(GEN_X_SIZE * GEN_Y_SIZE);
        for tile in tiles.into_iter().flatten() {
            vectorized.push(tile);
        }

        return Self { 
            tiles: vectorized,
            gen_size: Vec2::new(GEN_X_SIZE, GEN_Y_SIZE),
            src_img_block_size
         };
    }

    pub fn from_vec(mut tiles: Vec<TTile>, gen_size: Vec2<usize>, src_img_block_size: Vec2<usize>) -> Self {
        let missing_tiles_count = gen_size.x() * gen_size.y() - tiles.len();

        if  missing_tiles_count > 0 {
            for _ in 0..missing_tiles_count {
                tiles.push(TTile::default());
            }
        }

        return Self { 
            tiles,
            gen_size,
            src_img_block_size
         };
    }
}

impl<TTile: Tile> Generator for PatternGenerator<TTile> {
    type TileType = TTile;

    fn clone_with_brightness(&self, brightness: f32) -> Self {
        let mut clone = self.clone();
 
        for tile in &mut clone.tiles  {
            tile.set_brightness(brightness);
        }

        return clone;
    }

    #[inline]
    fn generator_block_size(&self) -> Vec2<usize> {
        return self.gen_size;
    }

    #[inline]
    fn source_image_block_size(&self) -> Vec2<usize> {
        return self.src_img_block_size;
    }
}

impl<TTile: Tile + ToSVG> ToSVG for PatternGenerator<TTile> {
    fn to_svg_node(&self) -> Box<dyn Node> {
        let mut g = Group::new();

        for tile_x in 0..self.gen_size.x() {
            for tile_y in 0..self.gen_size.y() {
                let tile_origin = Vec2::new(tile_y as f32, tile_x as f32);
                let tile = self.tiles[flatten_2d_index(tile_x, tile_y, self.gen_size.y())].to_svg_node();
                let tile_translated = Group::new()
                    .set("transform", format!("translate({} {})", tile_origin.x(), tile_origin.y()))
                    .add(tile);
                g.append(tile_translated);
            }
        }

        return Box::new(g);
    }
}

#[derive(Clone)]
pub struct RandomGenerator<TTile: Tile>(PatternGenerator<TTile>);

impl<TTile> RandomGenerator<TTile>
where 
    TTile: Tile,
    Standard: Distribution<TTile>
{
    fn new(gen_size: Vec2<usize>, src_img_block_size: Vec2<usize>) -> Self {
        let tiles_count = gen_size.x() * gen_size.y();
        let tiles: Vec<TTile> = (0..tiles_count).map(|_| rand::random()).collect();
        return Self(PatternGenerator::from_vec(tiles, gen_size, src_img_block_size));
    }
}

impl<TTile> Generator for RandomGenerator<TTile>
where 
    TTile: Tile,
    Standard: Distribution<TTile>
{
    type TileType = TTile;

    fn clone_with_brightness(&self, brightness: f32) -> Self {
        let mut clone = Self::new(self.generator_block_size(), self.source_image_block_size());

        for tile in &mut clone.0.tiles {
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

impl<TTile: Tile + ToSVG> ToSVG for RandomGenerator<TTile> {
    #[inline]
    fn to_svg_node(&self) -> Box<dyn Node> {
        return self.0.to_svg_node();
    }
}

pub fn stripes_ac(image_block_size: Vec2<usize>) -> PatternGenerator<ElasticTriangleTile> {
    return PatternGenerator::from_slice([
        [ElasticTriangleTile::type_a(), ElasticTriangleTile::type_c()],
        [ElasticTriangleTile::type_c(), ElasticTriangleTile::type_a()]
    ], image_block_size);
}

pub fn stripes_bd(image_block_size: Vec2<usize>) -> PatternGenerator<ElasticTriangleTile> {
    return PatternGenerator::from_slice([
        [ElasticTriangleTile::type_b(), ElasticTriangleTile::type_d()],
        [ElasticTriangleTile::type_d(), ElasticTriangleTile::type_b()]
    ], image_block_size);
}

pub fn bosh_d(image_block_size: Vec2<usize>) -> PatternGenerator<ElasticTriangleTile> {
    return PatternGenerator::from_slice([
        [ElasticTriangleTile::type_b(), ElasticTriangleTile::type_a()],
        [ElasticTriangleTile::type_c(), ElasticTriangleTile::type_d()]
    ], image_block_size);
}

pub fn fan(image_block_size: Vec2<usize>) -> PatternGenerator<ElasticTriangleTile> {
    return PatternGenerator::from_slice([
        [ElasticTriangleTile::type_a(), ElasticTriangleTile::type_b()],
        [ElasticTriangleTile::type_d(), ElasticTriangleTile::type_c()]
    ], image_block_size);
}

pub fn circles(image_block_size: Vec2<usize>) -> PatternGenerator<ElasticCircleTile> {
    return PatternGenerator::from_slice([
        [ElasticCircleTile::default(), ElasticCircleTile::default().flipped()],
        [ElasticCircleTile::default().flipped(), ElasticCircleTile::default()]
    ], image_block_size);
}

pub fn waves(image_block_size: Vec2<usize>) -> PatternGenerator<ElasticCircleTile> {
    return PatternGenerator::from_slice([
        [ElasticCircleTile::default(), ElasticCircleTile::default().flipped()],
        [ElasticCircleTile::default(), ElasticCircleTile::default().flipped()]
    ], image_block_size);
}

pub fn random(gen_size: Vec2<usize>, src_img_block_size: Vec2<usize>) -> RandomGenerator<ElasticTriangleTile> {
    return RandomGenerator::new(gen_size, src_img_block_size);
}

