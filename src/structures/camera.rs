use crate::{math::vector3::Vector3, ray::Ray};
use rand::prelude::*;

pub struct Camera {
    height: f32,
    width: f32,
    focal_length: f32,
    origin: Vector3,
    location: Vector3
}

impl Camera {
    pub fn new(h: f32, w: f32, foc: f32, orig: Vector3, location: Vector3) -> Self {
        Camera {
            height: h,
            width: w,
            focal_length: foc,
            origin: orig,
            location
        }
    }

    pub fn get_origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn get_ray(&self, i: u64, j: u64, pixel_width: u64, pixel_height: u64) -> Ray {
        let rand1: f32 = rand::thread_rng().gen();
        let rand2: f32 = rand::thread_rng().gen();
        
        let w = self.width;
        let h = self.height;
        let v = pixel_width as f32;
        let c = pixel_height as f32;
        let i = i as f32;
        let j = j as f32;
        let f = self.focal_length;

        let x = (w * (-v + rand1 + 2.0*i)) / (2.0 * v);
        let y = (h * (c - rand2 - 2.0*j)) / (2.0 * c);
        let z = -f;

        let dir = Vector3::new(x, y, z);
        let or = self.location.copy();
        return Ray::new(or, dir);
    }
}