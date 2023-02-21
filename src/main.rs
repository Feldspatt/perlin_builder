pub mod noise_map;

use nannou::prelude::*;
use nannou::image::{DynamicImage, GrayImage};
use nannou::wgpu::WithDeviceQueuePair;
use noise_map::NoiseBuilder;



fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    texture: wgpu::Texture,
}

fn event(_app: &App, model: &mut Model, _event: Event) {
    // Match the event against the possible events we care about.
    match _event {
        // If the window is closed, exit the program.
        Event::WindowEvent { simple: Some(KeyPressed(key)), .. } => {
            match key {
                Key::N => {
                    model.texture = new_texture(_app);
                }
                Key::S => {
                    let random_prefix = random::<u32>();
                    _app.main_window().capture_frame(format!("screenshot_{}.png", random_prefix));
                },
                _ => (),
            }
        },
        _ => (),
    }
}


fn new_texture(app: &App)-> wgpu::Texture {
    let img_buf = NoiseBuilder::generate_rgb_image(256, 1.0/5.0, 8, None);

    let gray_image = DynamicImage::ImageRgb8(img_buf);

    let usage = wgpu::TextureUsages::COPY_SRC |
        wgpu::TextureUsages::COPY_DST |
        wgpu::TextureUsages::RENDER_ATTACHMENT |
        wgpu::TextureUsages::TEXTURE_BINDING;

    app.with_device_queue_pair(|device, queue| {
        let texture = wgpu::Texture::load_from_image(device, queue, usage, &gray_image);

        texture
    })
}

fn model(app: &App) -> Model {
    let texture = new_texture(app);

    Model { texture }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    draw.texture(&model.texture);


    draw.to_frame(app, &frame).unwrap();
}
