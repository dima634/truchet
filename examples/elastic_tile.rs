use std::path::Path;

use image::GrayImage;
use truchet::{image::Image, vec2::Vec2, svg::node::element::SVG, to_svg::ToSVG};

struct ImageAdapter {
    image: GrayImage
}

impl ImageAdapter {
    fn new(image: GrayImage) -> Self { Self { image } }
}

impl Image for ImageAdapter {
    fn size(&self) -> Vec2<usize> {
        return Vec2::new(self.image.dimensions().0 as usize, self.image.dimensions().1 as usize);
    }

    fn get_pixel_brightness(&self, pos: Vec2<usize>) -> f32 {
        return self.image.get_pixel(pos.x() as u32, pos.y() as u32).0[0] as f32 / 255.0;
    }
}

fn main() {
    // Read image from fs
    let dog_path = Path::new(".")
        .join("examples")
        .join("dog.jpg");
    let image = image::open(dog_path).unwrap();
    let grayscale_image = ImageAdapter::new(image.into_luma8());

    // Generate "tiles" image
    let truchet = truchet::truchet_image::generate(&grayscale_image, truchet::generator::stripes_ac(Vec2::new(8, 8)));

    // Convert to svg and save to fs
    let svg = truchet.to_svg_node(10.0, Vec2::new(0.0, 0.0));
    let svg_doc = SVG::new()
        .add(svg)
        .set("height", "10000px")
        .set("width", "10000px");
    truchet::svg::save("./examples/dog_truchet.svg", &svg_doc).expect("Should save to file");
}
