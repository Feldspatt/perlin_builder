use nannou::image;
use nannou::prelude::map_range;
use nannou::image::{GrayImage, ImageBuffer, Luma, Pixel, RgbImage};
use noise::{NoiseFn, Perlin, PerlinSurflet, Seedable};
use nannou::rand::random;

use rand::prelude::*;

use pennereq::circ::ease_in_out;

pub(crate) struct NoiseBuilder {

}

impl NoiseBuilder {
    pub (crate) fn generate_gray_image(resolution: u32, scale: u8, octaves: u8, seed: Option<u32>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let seed = seed.unwrap_or_else(|| random::<u32>());

        let image = GrayImage::from_fn(resolution, resolution, |x, y| {
            let mut perlin = Perlin::new(seed);


            let mut  n = perlin.get([x as f64 / resolution as f64, y as f64 / resolution as f64]) as f32;

            if n < -1.0 || n > 1.0 {
                println!("n: {}", n)
            }

            n = map_range(n, -1.0, 1.0, 0.0, 1.0);

            let n = (n * 255.0) as u8;


            Luma([n])
        });

        image
    }

    pub (crate) fn generate_rgb_image(resolution: u32, scale: f64, octaves: u8, seed: Option<u32>) -> RgbImage {
        let seed = seed.unwrap_or_else(|| random::<u32>());

        let image = RgbImage::from_fn(resolution, resolution, |x, y| {
            let perlin = Perlin::new(seed);

            let mut n = perlin.get([x as f64 / scale / resolution as f64, y as f64 / scale / resolution as f64]) as f32;


            n = map_range(n, -1.0, 1.0, 0.0, 1.0);


            n = (n * 255.0);

            return image::Rgb([n as u8, n as u8, n as u8]);
        });

        image
    }
}

