use crate::{math::vector3::Vector3, ray::Ray, color::Color};

use super::renderable::{Renderable, HitRecord};

pub struct Sphere {
    center: Vector3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius
        }
    }

    pub fn normal(&self, p: &Vector3) -> Vector3 {
        let diff = Vector3::diff(p, &self.center);
        let n = diff.normalize();
        return n;
    }
}

impl Renderable for Sphere {
    fn trace(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let r_o = ray.get_origin();
        let r_d = ray.get_direction();

        let oc = Vector3::diff(r_o, &self.center);
        let a = Vector3::dot(r_d, r_d);
        let half_b = Vector3::dot(&oc, r_d);
        let c = Vector3::dot(&oc, &oc) - self.radius * self.radius;
        let dis = half_b*half_b - a*c;

        if dis < 0.0 {
            return None;
        }

        let disqrt = dis.sqrt();
        
        let t = (-half_b - disqrt) / a;

        if t < t_min || t > t_max {
            let t = (-half_b + disqrt) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let vec = ray.at(t);
        let mut n = self.normal(&vec);
        let is_front_face = if Vector3::dot(&r_d, &n) > 0.0 {
            n = Vector3::scale(&n, -1.0);
            false
        } else {
            true
        };

        return Some(
            HitRecord::new(vec, n, t, is_front_face)
        );
    }

    fn get_color(&self, hit_record: &HitRecord) -> Color {
        let n = hit_record.n();
        Color {
            r: n.x * 0.5 + 0.5,
            g: n.y * 0.5 + 0.5,
            b: n.z * 0.5 + 0.5
        }
    }
}