pub(crate) mod terrain_generation;
pub(crate) mod heightmap_generation;

use nannou::prelude::map_range;
use nannou::image::{ImageBuffer, Rgba};
use noise::{NoiseFn, Perlin};

pub fn generate_texture(coloring_method: fn(f32) -> Rgba<u8>, width: u32, heigth: u32, scale: f64, octaves: u8, seed: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

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

    let detail = 1./(scale*100.);

    let mut n = 0.0;
    let mut magnitude = 0.0;

    let image = ImageBuffer::from_fn(width, heigth, |x, y| {

        n = 0.0;

        for octave in 0..octaves {

            magnitude = get_octave_magnitude(octave);

            let sca = detail * magnitude as f64;

            let perlin = perlins[octave as usize];
            let increment = perlin.get([x as f64 * sca, y as f64 * sca]) as f32;
            n += increment/magnitude as f32;
        }

        n = map_range(n, -max_range, max_range, -100.0, 100.0);

        coloring_method(n)
    });

    image
}


