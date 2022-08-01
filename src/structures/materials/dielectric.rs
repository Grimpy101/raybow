use crate::{structures::{material::{Material, ScatterResult}, renderable::HitRecord}, color::Color, math::vector3::Vector3, ray::Ray};

pub struct Dielectric {
    ior: f32
}

impl Dielectric {
    pub fn new(ior: f32) -> Self {
        Dielectric {
            ior
        }
    }

    pub fn schlick_reflectance(cosine: f32, iof: f32) -> f32 {
        let mut r0 = (1.0 - iof) / (1.0 + iof);
        r0 = r0 * r0;
        let res = r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
        return res;
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let refr_ratio = if hit.front_face() {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_dir = r_in.get_direction().normalize();
        let cosa = (&-&unit_dir*hit.n()).min(1.0);
        let sina = (1.0 - cosa*cosa).sqrt();
        let cannot_refract = refr_ratio * sina > 1.0;
        let reflectance = Dielectric::schlick_reflectance(cosa, refr_ratio);
        let rand_num: f32 = rand::random();

        let refracted = if cannot_refract || reflectance > rand_num {
            Vector3::reflection(&unit_dir, &hit.n())
        } else {
            Vector3::refraction(&unit_dir, &hit.n(), refr_ratio)
        };

        let p = hit.p();
        let p_fixed = if hit.front_face() {
            p.copy() + hit.n() * 0.01
        } else {
            p.copy() - hit.n() * 0.01
        };
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let scattered = Ray::new(p_fixed, refracted);
        Some(ScatterResult {
            ray: scattered,
            attenuation
        })
    }

    fn copy(&self) -> Box<dyn Material> {
        let mat = Dielectric {
            ior: self.ior
        };
        return Box::new(mat);
    }
}