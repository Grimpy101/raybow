use crate::{ray::Ray, math::vector3::Vector3, color::Color};

use super::material::{Material};

pub struct HitRecord {
    p: Vector3,
    n: Vector3,
    t: f32,
    front_face: bool,
    material: Box<dyn Material>
}

impl HitRecord {
    pub fn new(p: Vector3, n: Vector3, t: f32, front_face: bool, material: Box<dyn Material>) -> Self {
        HitRecord {
            p, n, t, front_face, material
        }
    }

    pub fn p(&self) -> &Vector3 {
        &self.p
    }

    pub fn n(&self) -> &Vector3 {
        &self.n
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn material(&self) -> &Box<dyn Material> {
        &self.material
    }
}

pub trait Renderable {
    fn trace(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn get_color(&self, hit_record: &HitRecord) -> Color;
}