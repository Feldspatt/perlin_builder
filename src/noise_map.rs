use nannou::image;
use nannou::prelude::{map_range, pow};
use nannou::image::{RgbImage};
use noise::{NoiseFn, Perlin};
use nannou::rand::random;

pub(crate) struct NoiseBuilder {

}

impl NoiseBuilder {
    pub (crate) fn generate_rgb_image(dimension: u32, scale: f64, octaves: u8, seed: Option<u32>) -> RgbImage {
        let seed = seed.unwrap_or_else(|| random::<u32>());


        fn get_octave_magnitude(octave: u8) -> f32 {
            let mut octave_magnitude = 1.0;
            for i in 0..octave {
                octave_magnitude = 2.0*octave_magnitude;
            }

            octave_magnitude
        }

        let mut max_range = 0.0;
        for octave in 0..octaves {
            println!("octave: {}", octave);
            println!("octave_magnitude: {}", get_octave_magnitude(octave));
            max_range += 1.0/get_octave_magnitude(octave);
        }

        println!("max_range: {}", max_range);

        let dim: f64 = dimension as f64;
        let detail = scale/dim;

        let mut n = 0.0;
        let mut magnitude = 0.0;

        let image = RgbImage::from_fn(dimension, dimension, |x, y| {

            n = 0.0;

            for octave in 0..octaves {

                magnitude = get_octave_magnitude(octave);

                let sca = detail * magnitude as f64;

                let perlin = Perlin::new(seed + octave as u32);
                let increment = perlin.get([x as f64 * sca, y as f64 * sca]) as f32;
                n += increment/magnitude as f32;
            }


            n = map_range(n, -max_range, max_range, 0.0, 1.0);

            let n = (n * 255.0) as u8;


            return image::Rgb([n as u8, n as u8, n as u8]);
        });

        image
    }
}

