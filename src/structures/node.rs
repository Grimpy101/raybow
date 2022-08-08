use std::{collections::HashMap, rc::Rc, cell::{RefCell}, sync::{Arc, RwLock}, borrow::BorrowMut};

use crate::{math::{vector3::Vector3, matrix4::Matrix4}, animation::animation::AnimationChannel};

use super::{renderable::Renderable};

pub struct Node {
    renderable: Option<Box<dyn Renderable + Send + Sync>>,
    translation: Vector3,
    rotation: Vector3,
    scale: Vector3,
    transform_matrix: Matrix4,
    animation_channels: HashMap<String, AnimationChannel>,
    parent: Option<Arc<RwLock<Node>>>,
    children: Vec<Arc<RwLock<Node>>>
}

impl Node {
    pub fn new() -> Self {
        Node {
            renderable: None,
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            transform_matrix: Matrix4::identity(),
            animation_channels: HashMap::new(),
            parent: None,
            children: Vec::new()
        }
    }

    pub fn set_parent(&mut self, p: Arc<RwLock<Node>>) {
        self.parent = Some(p);
    }

    pub fn get_parent(&self) -> Option<Arc<RwLock<Node>>> {
        return self.parent.clone();
    }

    pub fn add_child(p: Arc<RwLock<Node>>, c: Arc<RwLock<Node>>) -> Arc<RwLock<Self>> {
        p.write().unwrap().children.push(c.clone());
        c.write().unwrap().set_parent(p.clone());
        return p;
    }

    pub fn get_children(&self) -> &Vec<Arc<RwLock<Node>>> {
        return &self.children;
    }

    pub fn set_translation(&mut self, t: Vector3) {
        self.translation = t;
        let m = Matrix4::from_srt(
            &self.translation, &self.rotation, &self.scale
        );
        self.transform_matrix = m;
    }

    pub fn set_rotation(&mut self, r: Vector3) {
        self.rotation = r;
        let m = Matrix4::from_srt(
            &self.translation, &self.rotation, &self.scale
        );
        self.transform_matrix = m;
    }

    pub fn set_scale(&mut self, s: Vector3) {
        self.scale = s;
        let m = Matrix4::from_srt(
            &self.translation, &self.rotation, &self.scale
        );
        self.transform_matrix = m;
    }

    pub fn update_transform_matrix(&mut self) {
        let m = Matrix4::from_srt(
            &self.translation, &self.rotation, &self.scale
        );
        self.transform_matrix = m;
    }

    pub fn set_animation_channel(&mut self, name: String, ch: AnimationChannel) {
        self.animation_channels.insert(name, ch);
    }

    pub fn get_transform_matrix(&self) -> &Matrix4 {
        return &self.transform_matrix;
    }

    pub fn update_transforms_per_frame(&mut self, f: f32) {
        let ch_tx_opt = self.animation_channels.get("translation_x");
        let ch_ty_opt = self.animation_channels.get("translation_y");
        let ch_tz_opt = self.animation_channels.get("translation_z");

        let ch_rx_opt = self.animation_channels.get("rotation_x");
        let ch_ry_opt = self.animation_channels.get("rotation_y");
        let ch_rz_opt = self.animation_channels.get("rotation_z");

        let ch_sx_opt = self.animation_channels.get("scale_x");
        let ch_sy_opt = self.animation_channels.get("scale_y");
        let ch_sz_opt = self.animation_channels.get("scale_z");

        let t = AnimationChannel::get_vector_by_frame(ch_tx_opt, ch_ty_opt, ch_tz_opt, &self.translation, f);
        let r = AnimationChannel::get_vector_by_frame(ch_rx_opt, ch_ry_opt, ch_rz_opt, &self.rotation, f);
        let s = AnimationChannel::get_vector_by_frame(ch_sx_opt, ch_sy_opt, ch_sz_opt, &self.scale, f);

        let m = Matrix4::from_srt(
            &t, &r, &s
        );
        match &self.parent {
            Some(p) => {
                let pm = &p.read().unwrap().transform_matrix;
                self.transform_matrix = Matrix4::mul(pm, &m);
            },
            None => {
                self.transform_matrix = m;
            }
        }
    }

    pub fn set_renderable(&mut self, r: Box<dyn Renderable + Send + Sync>) {
        self.renderable = Some(r);
    }

    pub fn renderable(&self) -> &Option<Box<dyn Renderable + Send + Sync>> {
        &self.renderable
    }
}