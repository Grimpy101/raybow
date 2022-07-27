use crate::math::vector3::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3
}

impl Ray {
    pub fn new(orig: Vector3, dir: Vector3) -> Self {
        Ray {
            origin: orig,
            direction: dir
        }
    }

    pub fn get_origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn at(&self, t: f32) -> Vector3 {
        let dt = Vector3::scale(&self.direction, t);
        let p = Vector3::sum(&self.origin, &dt);
        return p;
    }
}