use std::{sync::{Arc, RwLock}};

use crate::{ray::Ray};

use super::{node::Node, renderable::HitRecord, camera::Camera};

pub struct Scene {
    children: Vec<Arc<RwLock<Node>>>,
    cameras: Vec<Camera>
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            children: Vec::new(),
            cameras: Vec::new()
        }
    }

    pub fn add_child(&mut self, child: Arc<RwLock<Node>>) {
        self.children.push(child);
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.cameras.push(camera);
    }

    pub fn get_camera(&self, index: usize) -> Option<&Camera> {
        return self.cameras.get(index);
    }

    pub fn update_transforms(&mut self, f: f32) {
        let mut stack: Vec<Arc<RwLock<Node>>> = Vec::new();

        for i in &self.children {
            stack.push(i.clone());
        }

        while !stack.is_empty() {
            let node_arc = stack.last().unwrap().clone();
            let mut node = node_arc.write().unwrap();
            node.update_transforms_per_frame(f);
            stack.pop();

            for i in node.get_children() {
                stack.push(i.clone());
            }
        }
    }

    pub fn trace(&self, ray: &Ray, t_min: f32, t_max: f32, f: f32) -> Option<HitRecord> {
        let mut hit_opt = None;

        let mut stack: Vec<Arc<RwLock<Node>>> = Vec::new();

        for i in &self.children {
            stack.push(i.clone());
        }

        while !stack.is_empty() {
            let node_arc = stack.last().unwrap().clone();
            let node = node_arc.read().unwrap();

            hit_opt = Scene::trace_child(&node, ray, t_min, t_max, hit_opt, f);

            stack.pop();

            for i in node.get_children() {
                stack.push(i.clone());
            }
        }

        return hit_opt;
    }

    fn trace_child(node: &Node, ray: &Ray, t_min: f32, t_max: f32, hit_opt: Option<HitRecord>, f: f32) -> Option<HitRecord> {
        let rend_opt = node.renderable();

        if rend_opt.is_none() {
            return hit_opt;
        }

        let rend = rend_opt.as_ref().unwrap();
        let new_hit_opt = rend.trace(ray, t_min, t_max, f, node.get_transform_matrix());

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