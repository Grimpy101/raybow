use std::collections::HashMap;

use crate::{math::vector3::Vector3, ray::Ray, color::Color, animation::animation::AnimationChannel};

use super::{renderable::{Renderable, HitRecord}, material::Material};

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Box<dyn Material + Send + Sync>,
    animation_channels: HashMap<String, AnimationChannel>
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: Box<dyn Material + Send + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            animation_channels: HashMap::new()
        }
    }

    pub fn add_animation_channel(&mut self, name: String, ch: AnimationChannel) {
        self.animation_channels.insert(name, ch);
    }

    pub fn normal(&self, p: &Vector3) -> Vector3 {
        let diff = Vector3::diff(p, &self.center);
        let n = diff * (1.0 / self.radius);
        return n;
    }

    pub fn get_center_by_frame(&self, f: f32) -> Vector3 {
        let ch_x = self.animation_channels.get("center_x");
        let ch_y = self.animation_channels.get("center_y");
        let ch_z = self.animation_channels.get("center_z");

        return AnimationChannel::get_vector_by_frame(ch_x, ch_y, ch_z, &self.center, f);
    }

    pub fn get_radius_by_frame(&self, f: f32) -> f32 {
        let ch_r = self.animation_channels.get("radius");
        if ch_r.is_some() {
            return match ch_r.unwrap().get_value_at_frame(f) {
                Some(s) => s,
                None => self.radius
            }
        }
        return self.radius;
    }
}

impl Renderable for Sphere {
    fn trace(&self, ray: &Ray, t_min: f32, t_max: f32, f: f32) -> Option<HitRecord> {
        let v_ro = ray.get_origin();
        let v_rd = ray.get_direction();
        let v_sc = &self.get_center_by_frame(f);
        let sr = self.get_radius_by_frame(f);

        let v_oc = v_ro - v_sc;
        let a = v_rd * v_rd;
        let half_b = &v_oc * v_rd;
        let c = &v_oc * &v_oc - sr * sr;
        let dis = half_b * half_b - a * c;

        if dis < 0.0 {
            return None;
        }

        let disqrt = dis.sqrt();
        
        let mut t = (-half_b - disqrt) / a;

        if t < t_min || t > t_max {
            t = (-half_b + disqrt) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let point = ray.at(t);
        let mut v_n = self.normal(&point);
        let is_front_face = if v_rd * &v_n >= 0.0 {
            v_n = v_n * -1.0;
            false
        } else {
            true
        };

        return Some(
            HitRecord::new(point, v_n, t, is_front_face, self.material.copy())
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