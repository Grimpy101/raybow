use crate::{math::{vector3::Vector3, matrix3::Matrix3}, ray::Ray};
use rand::prelude::*;

pub struct CameraBuilder {
    camera: Camera
}

impl CameraBuilder {
    pub fn set_aspect_ratio(mut self, ar: f32) -> CameraBuilder {
        self.camera.aspect_ratio = ar;
        return self;
    }

    pub fn set_vertical_field_of_view(mut self, fov: f32) -> CameraBuilder {
        self.camera.vfov = fov;
        return self;
    }

    pub fn build(mut self) -> Camera {
        self.camera.update_viewport_dim();
        return self.camera;
    }
}

pub struct Camera {
    aspect_ratio: f32,
    vfov: f32,
    width: f32,
    height: f32,

    location: Vector3,
    rotation: Vector3,
    rotation_matrix: Matrix3,

    focus_dist: f32,
    aperture_size: f32
}

impl Camera {
    /*pub fn new(ar: f32, vfov: f32, location: Vector3, rotation: Vector3,
        focus_dist: f32, ap_size: f32) -> Self {
        let mut c = Camera {
            aspect_ratio: ar,
            vfov,
            location,
            rotation,
            rotation_matrix: Matrix3::identity(),
            width: 1.0,
            height: 1.0,
            focus_dist,
            aperture_size: ap_size
        };
        c.update_veiwport_dim();
        c.update_rotation_matrix();
        return c;
    }*/

    pub fn new() -> CameraBuilder {
        let c = Camera {
            aspect_ratio: 16.0 / 9.0,
            vfov: 60.0,
            location: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            rotation_matrix: Matrix3::identity(),
            width: 1.0,
            height: 1.0,
            focus_dist: 1.0,
            aperture_size: 0.0
        };
        return CameraBuilder { camera: c };
    }

    pub fn set_location(&mut self, v: Vector3) {
        self.location = v;
    }

    pub fn set_rotation(&mut self, r: Vector3) {
        self.rotation = r;
        self.update_rotation_matrix();
    }

    pub fn set_focus_distance(&mut self, fd: f32) {
        self.focus_dist = fd;
    }

    pub fn set_aperture_size(&mut self, ap_s: f32) {
        self.aperture_size = ap_s;
    }

    pub fn set_vertical_field_of_view(&mut self, vfov: f32) {
        self.vfov = vfov;
        self.update_viewport_dim();
    }

    pub fn update_viewport_dim(&mut self) {
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
        let rotated_dir = (&self.rotation_matrix * &dir).normalize();
        //println!("{}, {}", dir, rotated_dir);

        let focal_point = &self.location + &(&rotated_dir * self.focus_dist);
        
        let x: f32 = (rand::random::<f32>() - 0.5) * self.aperture_size + self.location.x;
        let y: f32 = (rand::random::<f32>() - 0.5) * self.aperture_size + self.location.y;
        let z: f32 = (rand::random::<f32>() - 0.5) * self.aperture_size + self.location.z;
        let or = Vector3::new(x, y, z);

        let new_dir = (&focal_point - &or).normalize();

        return Ray::new(or, new_dir);
    }
}