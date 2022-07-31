use std::ops;

use super::vector3::Vector3;

pub struct Matrix3 {
    matrix: Vec<f32>
}

impl Matrix3 {
    pub fn identity() -> Self {
        let m = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        Matrix3 {
            matrix: m
        }
    }

    pub fn rotation_matrix(x: f32, y: f32, z: f32) -> Self {
        let x = x.to_radians();
        let y = y.to_radians();
        let z = z.to_radians();

        let sinx = x.sin();
        let siny = y.sin();
        let sinz = z.sin();

        let cosx = x.cos();
        let cosy = y.cos();
        let cosz = z.cos();

        let a11 = cosy*cosz;
        let a12 = sinx*siny*cosz - cosx*sinz;
        let a13 = cosx*siny*cosz + sinx*sinz;
        
        let a21 = cosy*sinz;
        let a22 = sinx*siny*sinz + cosx*cosz;
        let a23 = cosx*siny*sinz - sinx*cosz;
        
        let a31 = -siny;
        let a32 = sinx*cosy;
        let a33 = cosx*cosy;

        let m = vec![a11, a12, a13, a21, a22, a23, a31, a32, a33];
        return Matrix3 {
            matrix: m
        };
    }
}

impl ops::Mul<&Vector3> for &Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        let m = &self.matrix;

        let x = m[0]*rhs.x + m[1]*rhs.y + m[2]*rhs.z;
        let y = m[3]*rhs.x + m[4]*rhs.y + m[5]*rhs.z;
        let z = m[6]*rhs.x + m[7]*rhs.y + m[8]*rhs.z;

        Vector3 {
            x, y, z
        }
    }
}