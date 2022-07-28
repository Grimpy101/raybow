use crate::{color::Color, ray::Ray, structures::{renderable::HitRecord, material::{ScatterResult, Material}}, math::vector3::Vector3};

pub struct Diffuse {
    albedo: Color
}

impl Diffuse {
    pub fn new(albedo: Color) -> Self {
        Diffuse {
            albedo
        }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let n = hit.n();
        let p = hit.p();
        let r = Vector3::random_in_unit_sphere();

        let mut scattered_dir = n + &r;
        if scattered_dir.near_zero() {
            scattered_dir = n.copy();
        }

        let scattered = Ray::new(p.copy(), scattered_dir);
        let attenuation = self.albedo.copy();

        Some(ScatterResult {
            ray: scattered,
            attenuation
        })
    }

    fn copy(&self) -> Box<dyn Material> {
        let diff = Diffuse {
            albedo: self.albedo.copy()
        };
        Box::new(diff)
    }
}