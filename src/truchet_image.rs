use svg::{node::element::Group, Node};

use crate::{vec2::Vec2, image::Image, generator::{Generator}, to_svg::ToSVG, utils::flatten_2d_index};

pub struct TruchetImage<TGenerator: Generator> {
    generators_size: Vec2<usize>,
    generator: TGenerator, 
    generators: Vec<TGenerator>
}

pub fn generate<TImage: Image, TGenerator: Generator>(image: &TImage, generator: TGenerator) -> TruchetImage<TGenerator> {
    let generators_size = image.size() / generator.source_image_block_size();
    let mut truchet = TruchetImage {
        generators_size,
        generator,
        generators: Vec::with_capacity(generators_size.x() * generators_size.y())
    };

    let source_block_size = truchet.generator.source_image_block_size().x() * truchet.generator.source_image_block_size().y();

    for generator_x in 0..generators_size.x() {
        for generator_y in 0..generators_size.y() {
            let start_x = truchet.generator.source_image_block_size().x() * generator_x;
            let start_y = truchet.generator.source_image_block_size().y() * generator_y;
            let end_x = start_x + truchet.generator.source_image_block_size().x();
            let end_y = start_y + truchet.generator.source_image_block_size().y();
            
            let mut brightness = 0.0;
            for pixel_x in start_x..end_x {
                for pixel_y in start_y..end_y {
                    brightness += image.get_pixel_brightness((pixel_x, pixel_y).into());
                }
            }

            brightness /= source_block_size as f32;
            
            truchet.generators.push(truchet.generator.clone_with_brightness(brightness));
        }
    }

    return truchet;
}

impl<TGenerator: Generator + ToSVG> ToSVG for TruchetImage<TGenerator> {
    fn to_svg_node(&self) -> Box<dyn Node> {
        let mut g = Group::new();
    
        for gen_x in 0..self.generators_size.x() {
            for gen_y in 0..self.generators_size.y() {
                let pattern_origin = Vec2::new(
                    (self.generator.generator_block_size().y() * gen_x) as f32,
                    (self.generator.generator_block_size().x() * gen_y) as f32
                );

                let gen = self.generators[flatten_2d_index(gen_x, gen_y, self.generators_size.y())]
                    .to_svg_node();
                let gen_translated = Group::new()
                    .set("transform", format!("translate({} {})", pattern_origin.x(), pattern_origin.y()))
                    .add(gen);

                g.append(gen_translated);
            }
        }

        return Box::new(g);
    }
}
