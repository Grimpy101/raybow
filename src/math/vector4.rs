/*use std::fmt::Display;

use super::vector3::Vector3;

pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 {
            x,
            y,
            z,
            w
        }
    }

    pub fn add(v1: &Vector4, v2: &Vector4) -> Self {
        Vector4 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z,
            w: v1.w + v2.w
        }
    }

    pub fn sub(v1: &Vector4, v2: &Vector4) -> Self {
        Vector4 {
            x: v1.x - v2.x,
            y: v1.y - v2.y,
            z: v1.z - v2.z,
            w: v1.w - v2.w
        }
    }

    pub fn dot(v1: &Vector4, v2: &Vector4) -> f32 {
        let sum = v1.x * v2.x
                + v1.y * v2.y
                + v1.z * v2.z
                + v1.w * v2.w;
        return sum;
    }

    pub fn from_vector3(v: &Vector3, w: f32) -> Self {
        Vector4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w
        }
    }
}

impl Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {} {} {}]", self.x, self.y, self.z, self.w)
    }
}*/