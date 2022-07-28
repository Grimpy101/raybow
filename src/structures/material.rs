use crate::{ray::Ray, color::Color};

use super::renderable::HitRecord;

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Color
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult>;
    fn copy(&self) -> Box<dyn Material>;
}