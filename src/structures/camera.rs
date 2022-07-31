use crate::{math::{vector3::Vector3, matrix3::Matrix3}, ray::Ray};
use rand::prelude::*;

pub struct Camera {
    aspect_ratio: f32,
    vfov: f32,
    width: f32,
    height: f32,

    location: Vector3,
    rotation: Vector3,
    rotation_matrix: Matrix3
}

impl Camera {
    pub fn new(ar: f32, vfov: f32, location: Vector3, rotation: Vector3) -> Self {
        let mut c = Camera {
            aspect_ratio: ar,
            vfov,
            location,
            rotation,
            rotation_matrix: Matrix3::identity(),
            width: 1.0,
            height: 1.0
        };
        c.update_veiwport_dim();
        c.update_rotation_matrix();
        return c;
    }

    pub fn update_veiwport_dim(&mut self) {
        let theta = self.vfov.to_radians();
        let height = (theta / 2.0).tan() * 2.0;
        let width = self.aspect_ratio * height;
        self.height = height;
        self.width = width;
    }

    pub fn update_rotation_matrix(&mut self) {
        self.rotation_matrix = Matrix3::rotation_matrix(
            self.rotation.x, self.rotation.y, self.rotation.z);
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

        let x = (w * (-v + rand1 + 2.0*i)) / (2.0 * v);
        let y = (h * (c - rand2 - 2.0*j)) / (2.0 * c);
        let z = -1.0;

        let dir = Vector3::new(x, y, z);
        let rotated_dir = &self.rotation_matrix * &dir;
        //println!("{}, {}", dir, rotated_dir);
        let or = self.location.copy();
        return Ray::new(or, rotated_dir);
    }
}