use crate::ray::{Light};
use crate::occluders::{Circle};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use ron;

#[derive(Serialize,Deserialize)]
pub struct Scene {
    pub lights: Vec<Light>,
    pub occluders: Vec<Circle>,
}

impl Scene {
    pub fn from_ron(description: &mut File) -> Scene {
        let mut contents = String::new();
        description.read_to_string(&mut contents).unwrap();
        ron::de::from_str(&contents).unwrap()
    }
}