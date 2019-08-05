mod vector;
mod ray;
mod colour;
mod occluders;
mod scene;

use piston_window::{PistonWindow, EventLoop, WindowSettings, Texture, TextureSettings};
use image::{ImageBuffer, Rgba};
use crate::vector::{Vec2};
use crate::colour::{colour, frag_to_pixel};
use crate::ray::{Ray, Light};
use crate::occluders::{Circle, Occludes};
use crate::scene::Scene;
use std::fs::File;
use std::env;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene_description = &args[1];

    let scene = Scene::from_ron(&mut File::open(scene_description).unwrap());

    let mut frame_buffer = ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgba([0,0,0,255]));

    for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
        let world_space_position = Vec2 {
            x: x as f64 / WIDTH as f64,
            y: y as f64 / WIDTH as f64,
        };

        let mut fragment = colour(0.0,0.0,0.0);

        for light in &scene.lights {
            let ray_to = Ray::new(world_space_position, light);

            if !scene.occluders.iter().any(|occluder| {
                occluder.hit_by(&ray_to)
            }) {
                fragment += ray_to.shade();
            }
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
