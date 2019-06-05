use crate::vector::{Vec2,dot,Squared};
use crate::ray::{Ray};

pub trait Occludes {
    fn hit_by(&self, ray: &Ray) -> bool;
}

#[derive(Copy,Clone,Debug)]
pub struct Circle {
    pub centre: Vec2<f64>,
    pub radius: f64,
}

impl Occludes for Circle {
    fn hit_by(&self, ray: &Ray) -> bool {
        //vector from ray origin to centre of circle
        let origin_to_centre = self.centre - ray.origin;
        //if the ray starts inside the circle...
        if origin_to_centre.squared() < self.radius.powi(2) {
            //test if the light is outside the circle
            (self.centre - ray.target.loc).squared() > self.radius.powi(2)
        } else {
            //measure how far along the ray is the point of closest approach
            let distance_along_ray = dot(origin_to_centre, ray.dir);
            //if the ray is going away from the point of closest approach
            if distance_along_ray < 0.0 {
                false
            //if we'd hit the light before the distance of closest approach
            } else if distance_along_ray > ray.length {
                //test if the light is outside the circle
                (self.centre - ray.target.loc).squared() < self.radius.powi(2)
            } else {
                //test if the distance of closest approach is less than the circle radius
                origin_to_centre.squared() - distance_along_ray.powi(2) < self.radius.powi(2)
            }
        }
    }
}