use crate::vector::{Vec3, Clamp};
use float::{Float};
use image::{Rgba};

const CLAMP_MAX: f64 = 1.0;

pub fn colour<T> (r: T, g: T, b: T) -> Vec3<T> {
    Vec3 {
        x: r,
        y: g,
        z: b,
    }
}

fn component_linear_to_srgb<F: Float> (lin_component: F) -> F {
    if lin_component <= F::from_f64(0.0031308) {
        lin_component * F::from_f64(12.92)
    } else {
        F::from_f64(1.055) * lin_component.powf(F::from_f64(1.0) / F::from_f64(2.4)) - F::from_f64(0.055)
    }
}

fn linear_to_srgb<F: Float> (lin: Vec3<F>) -> Vec3<F> {
    Vec3 {
        x: component_linear_to_srgb(lin.x),
        y: component_linear_to_srgb(lin.y),
        z: component_linear_to_srgb(lin.z),
    }
}

pub fn frag_to_pixel (fragment: Vec3<f64>) -> Rgba<u8> {
    let fragment = linear_to_srgb(fragment.clamp(CLAMP_MAX) / CLAMP_MAX)* 255 as f64;
    Rgba([
        fragment.x as u8,
        fragment.y as u8,
        fragment.z as u8,
        255,
    ])
}