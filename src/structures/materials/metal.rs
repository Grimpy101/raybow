use crate::{color::Color, structures::{material::{Material, ScatterResult}, renderable::HitRecord}, math::vector3::Vector3, ray::Ray};

pub struct Metal {
    albedo: Color,
    roughness: f32
}

impl Metal {
    pub fn new(albedo: Color, roughness: f32) -> Self {
        Metal {
            albedo, roughness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &crate::ray::Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let rand = Vector3::random_in_unit_sphere();
        let d = r_in.get_direction().normalize();
        let n = hit.n();
        let p = hit.p();
        let p_fixed = if hit.front_face() {
            p.copy() + n * 0.0001
        } else {
            p.copy() - n * 0.0001
        };
        
        let reflected = Vector3::reflection(&d, n);
        let dir = reflected + rand * self.roughness;
        let scattered = Ray::new(p_fixed, dir);
        let attenuation = self.albedo.copy();

        if scattered.get_direction() * n > 0.0 {
            return Some(ScatterResult {
                ray: scattered,
                attenuation
            })
        }
        return None;
    }

    fn copy(&self) -> Box<dyn Material> {
        let met = Metal {
            albedo: self.albedo.copy(),
            roughness: self.roughness
        };
        Box::new(met)
    }
}