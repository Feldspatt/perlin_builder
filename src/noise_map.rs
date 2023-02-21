use nannou::image;
use nannou::prelude::{map_range};
use nannou::image::{RgbImage};
use noise::{NoiseFn, Perlin};

pub(crate) struct NoiseBuilder {

}

impl NoiseBuilder {
    pub (crate) fn generate_rgb_image(dimension: u32, scale: f64, octaves: u8, seed: u32) -> RgbImage {

        fn get_octave_magnitude(octave: u8) -> f32 {
            let mut octave_magnitude = 1.0;
            for _i in 0..octave {
                octave_magnitude = 2.0*octave_magnitude;
            }

            octave_magnitude
        }

        let mut perlins = Vec::new();
        for octave in 0..octaves {
            let perlin = Perlin::new(seed + octave as u32);
            perlins.push(perlin);
        }

        let mut max_range = 0.0;
        for octave in 0..octaves {


            max_range += 1.0/get_octave_magnitude(octave);
        }

        let dim: f64 = dimension as f64;
        let detail = scale/dim;

        let mut n = 0.0;
        let mut magnitude = 0.0;

        let image = RgbImage::from_fn(dimension, dimension, |x, y| {

            n = 0.0;

            for octave in 0..octaves {

                magnitude = get_octave_magnitude(octave);

                let sca = detail * magnitude as f64;

                let perlin = perlins[octave as usize];
                let increment = perlin.get([x as f64 * sca, y as f64 * sca]) as f32;
                n += increment/magnitude as f32;
            }


            n = map_range(n, -max_range, max_range, 0.0, 1.0);


            return if n < 0.4 {
                let n = (n * 255.0) as u8;
                image::Rgb([n, n, 255])
            } else if n > 0.7 {
                let n = (n * 255.0) as u8;
                image::Rgb([n, n, n])
            } else {
                let n = ((n+1.0) * 75.0) as u8;
                image::Rgb([n, 255, n])
            }
        });

        image
    }
}

