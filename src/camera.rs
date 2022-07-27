use crate::math::vector3::Vector3;

pub struct Camera {
    height: f32,
    width: f32,
    focal_length: f32,
    origin: Vector3
}

impl Camera {
    pub fn new(h: f32, w: f32, foc: f32, orig: Vector3) -> Self {
        Camera {
            height: h,
            width: w,
            focal_length: foc,
            origin: orig
        }
    }

    pub fn get_origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn get_ray_direction(&self, i: u64, j: u64, pixel_width: u64, pixel_height: u64) -> Vector3 {
        let w = self.width;
        let h = self.height;
        let v = pixel_width as f32;
        let c = pixel_height as f32;
        let i = i as f32;
        let j = j as f32;
        let f = self.focal_length;

        let x = (w * (-v + 1.0 + 2.0*i)) / (2.0 * v);
        let y = (h * (c - 1.0 - 2.0*j)) / (2.0 * c);
        let z = -f;
        return Vector3::new(x, y, z);
    }
}