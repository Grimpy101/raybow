use crate::{ray::Ray};

use super::{node::Node, renderable::HitRecord};

pub struct Scene {
    children: Vec<Node>
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            children: Vec::new()
        }
    }

    pub fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }

    pub fn trace(&self, ray: &Ray, t_min: f32, t_max: f32, f: f32) -> Option<HitRecord> {
        let mut hit_opt = None;

        for child in &self.children {
            hit_opt = Scene::trace_child(child, ray, t_min, t_max, hit_opt, f);
        }
        return hit_opt;
    }

    pub fn trace_child(node: &Node, ray: &Ray, t_min: f32, t_max: f32, hit_opt: Option<HitRecord>, f: f32) -> Option<HitRecord> {
        let rend_opt = node.renderable();

        if rend_opt.is_none() {
            return hit_opt;
        }

        let rend = rend_opt.as_ref().unwrap();
        let new_hit_opt = rend.trace(ray, t_min, t_max, f);

        if hit_opt.is_none() {
            return new_hit_opt;
        }

        if new_hit_opt.is_none() {
            return hit_opt;
        }

        let hit = hit_opt.unwrap();
        let new_hit = new_hit_opt.as_ref().unwrap();
        
        if hit.t() > new_hit.t() {
            return new_hit_opt;
        }

        return Some(hit);
    }
}