mod vector;
mod ray;
mod colour;

use piston_window::{PistonWindow, EventLoop, WindowSettings, Texture, TextureSettings};
use image::{ImageBuffer, Rgba};
use crate::vector::{Vec2};
use crate::colour::{colour, frag_to_pixel};
use crate::ray::{Ray, Light};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let mut frame_buffer = ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgba([0,0,0,255]));

    let lights = vec![
        Light{
            loc: Vec2 {
                x: 0.5,
                y: 0.2,
            },
            col: colour(1.0,0.0,0.0)
        }
    ];

    for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
        let world_space_position = Vec2 {
            x: x as f64 / WIDTH as f64,
            y: y as f64 / WIDTH as f64,
        };

        let mut fragment = colour(0.0,0.0,0.0);

        for light in &lights {
            let ray_to = Ray::new(world_space_position, light);
            fragment += ray_to.shade();
        }

        *pixel = frag_to_pixel(fragment);
    }

    let mut window: PistonWindow =
    WindowSettings::new("Raytracer", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|_e| { panic!("Could not create window!")});

    let tex = Texture::from_image(
        &mut window.create_texture_context(),
        &frame_buffer,
        &TextureSettings::new())
        .unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            piston_window::image(&tex, c.transform, g)
        });
    }
}
