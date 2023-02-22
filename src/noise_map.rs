use nannou::image;
use nannou::prelude::{map_range};
use nannou::image::{RgbImage};
use noise::{NoiseFn, Perlin};


enum Layer {
    Water,
    Grove,
    Grass,
    Rock,
    Snow,
}

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


            n = map_range(n, -max_range, max_range, -100.0, 100.0);

            let layer = Layer::get_layer(&n);
            layer.get_color(&n)

        });

        image
    }
}


impl Layer {
    fn get_height(&self) -> f32 {
        match self {
            Layer::Water => -30.0,
            Layer::Grove => -10.0,
            Layer::Grass => 30.0,
            Layer::Rock => 60.0,
            Layer::Snow => 100.0,
        }
    }

    fn get_layer(&height: &f32) -> Layer {
        if height < Layer::Water.get_height() {
            Layer::Water
        } else if height < Layer::Grove.get_height() {
            Layer::Grove
        } else if height < Layer::Grass.get_height() {
            Layer::Grass
        } else if height < Layer::Rock.get_height() {
            Layer::Rock
        } else {
            Layer::Snow
        }
    }

    fn get_previous_layer(&self) -> Option<Layer> {
        match self {
            Layer::Water => None,
            Layer::Grove => Some(Layer::Water),
            Layer::Grass => Some(Layer::Grove),
            Layer::Rock => Some(Layer::Grass),
            Layer::Snow => Some(Layer::Rock),
        }
    }

    fn get_plage(&self) -> (f32, f32) {
        let height = self.get_height();
        let previous_height = match self.get_previous_layer() {
            Some(layer) => layer.get_height(),
            None => -100.
        };

        (previous_height, height)
    }

    fn get_color(&self, &height: &f32) -> image::Rgb<u8> {
        let (min_height, max_height) = self.get_plage();
        let norm_height = map_range(height as f32, min_height, max_height, 0.0, 255.0);

        match self {
            Layer::Water => {
                image::Rgb([0, (255. - norm_height) as u8/3, norm_height as u8])
            },
            Layer::Grove => image::Rgb([20 + norm_height as u8/3, 128, norm_height as u8/3]),
            Layer::Grass => image::Rgb([50 +(norm_height/3.) as u8 , 215 - norm_height as u8/5, 50 +(norm_height/4.) as u8]),
            Layer::Rock => image::Rgb([ 75 + norm_height as u8/2, 75 + norm_height as u8/2, 100 + norm_height as u8/2]),
            Layer::Snow => image::Rgb([255, 255, 255])
        }
    }
}

