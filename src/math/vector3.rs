use std::fmt::Display;
use rand::{self, Rng};
use std::ops;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn dot(x: &Vector3, y: &Vector3) -> f32 {
        x.x * y.x + x.y * y.y + x.z * y.z
    }

    pub fn cross(v1: &Vector3, v2: &Vector3) -> Self {
        let x = v1.y * v2.z - v1.z * v2.y;
        let y = v1.z * v2.x - v1.x * v2.z;
        let z = v1.x * v2.y - v1.y * v2.x;
        Vector3 {
            x, y, z
        }
    }

    pub fn sum(v1: &Vector3, v2: &Vector3) -> Self {
        Vector3 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z
        }
    }

    pub fn diff(v1: &Vector3, v2: &Vector3) -> Self {
        Vector3 {
            x: v1.x - v2.x,
            y: v1.y - v2.y,
            z: v1.z - v2.z
        }
    }

    pub fn scale(v: &Vector3, s: f32) -> Self {
        Vector3 {
            x: v.x * s,
            y: v.y * s,
            z: v.z * s
        }
    }

    pub fn distance(v1: &Vector3, v2: &Vector3) -> f32 {
        let x = v1.x - v2.x;
        let y = v1.y - v2.y;
        let z = v1.z - v2.z;

        (x*x + y*y + z*z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        Vector3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn reflection(d: &Vector3, n: &Vector3) -> Vector3 {
        let dn = d * n;
        let s = Vector3::scale(n, 2.0 * dn);
        
        Vector3::diff(d, &s)
    }

    pub fn orth_proj(b: &Vector3, a: &Vector3) -> Self {
        let top = a.x * b.x + a.y * b.y + a.z * b.z;
        let btm = b.x * b.x + b.y * b.y + b.z * b.z;
        let s = top / btm;
        Vector3 {
            x: b.x * s,
            y: b.y * s,
            z: b.z * s
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn copy(&self) -> Self {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        let a1: f32 = rand::prelude::thread_rng().sample(rand_distr::StandardNormal);
        let a2: f32 = rand::prelude::thread_rng().sample(rand_distr::StandardNormal);
        let a3: f32 = rand::prelude::thread_rng().sample(rand_distr::StandardNormal);

        let norm = (a1*a1 + a2*a2 + a3*a3).sqrt();

        Vector3 {
            x: a1 / norm,
            y: a2 / norm,
            z: a3 / norm
        }
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {} {}]", self.x, self.y, self.z)
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(mut self, rhs: Vector3) -> Self::Output {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self
    }
}

impl ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = f32;

    fn mul(self, rhs: Vector3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Mul<&Vector3> for &Vector3 {
    type Output = f32;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x = rhs * self.x;
        self.y = rhs * self.y;
        self.z = rhs * self.z;
        self
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(mut self, rhs: Vector3) -> Self::Output {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self
    }
}

impl ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}