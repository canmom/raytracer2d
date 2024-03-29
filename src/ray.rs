use crate::vector::{Vec2, Vec3};
use serde::{Serialize, Deserialize};

const FALLOFF_RATE: f64 = 16.0;

#[derive(Debug,Serialize,Deserialize)]
pub struct Light {
    pub loc: Vec2<f64>,
    pub col: Vec3<f64>,
}

#[derive(Debug)]
pub struct Ray<'a> {
    pub origin: Vec2<f64>,
    pub dir: Vec2<f64>,
    pub length: f64,
    pub target: &'a Light,
}

impl<'a> Ray<'a> {
    pub fn new(from: Vec2<f64>, to: &Light) -> Ray {
        let fullvector = to.loc - from;
        Ray {
            origin: from,
            dir: fullvector.normalise(),
            length: fullvector.length(),
            target: &to,
        }
    }

    pub fn shade(&self) -> Vec3<f64> {
        self.target.col / (FALLOFF_RATE * self.length * self.length + 1.0)
    }
}