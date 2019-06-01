extern crate image;
extern crate piston_window;

mod vector;

use piston_window::{PistonWindow, EventLoop, WindowSettings, Texture, TextureSettings};
use image::{ImageBuffer, Rgba, Rgb};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let linear_buffer = ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgb([0.,0.,0.]));
    let srgb_buffer = ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgba([0,0,0,255]));

    let mut window: PistonWindow =
    WindowSettings::new("Raytracer", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|_e| { panic!("Could not create window!")});

    let tex = Texture::from_image(
        &mut window.create_texture_context(),
        &srgb_buffer,
        &TextureSettings::new())
        .unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            piston_window::image(&tex, c.transform, g)
        });
    }
}
