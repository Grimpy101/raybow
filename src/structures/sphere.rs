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
        let v_ro = ray.get_origin();
        let v_rd = ray.get_direction();
        let v_sc = &self.center;
        let sr = self.radius;

        let v_oc = v_ro - v_sc;
        let a = v_rd * v_rd;
        let half_b = &v_oc * v_rd;
        let c = &v_oc * &v_oc - sr * sr;
        let dis = half_b * half_b - a * c;

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

        let point = ray.at(t);
        let mut v_n = self.normal(&point);
        let is_front_face = if v_rd * &v_n > 0.0 {
            v_n = v_n * -1.0;
            false
        } else {
            true
        };

        return Some(
            HitRecord::new(point, v_n, t, is_front_face)
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