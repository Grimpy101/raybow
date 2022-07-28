/*use std::fmt::{Display};

use super::{vector4::Vector4, PI_DIV_180};

pub struct Matrix4 {
    matrix: Vec<f32>
}

impl Matrix4 {
    pub fn new(matrix: Vec<f32>) -> Self {
        let a = match matrix.len() != 15 {
            true => {
                let mut b = vec![0.0; 16];
                for i in 0..matrix.len().min(16) {
                    b[i] = matrix[i];
                }
                b
            },
            false => {
                matrix
            }
        };

        Matrix4 {
            matrix: a
        }
    }

    pub fn mul(m1: &Matrix4, m2: &Matrix4) -> Self {
        let a = &m1.matrix;
        let b = &m2.matrix;

        let mut res = vec![0.0; 16];

        res[0] = a[0]*b[0] + a[1]*b[4] + a[2]*b[8]  + a[3]*b[12];
        res[1] = a[0]*b[1] + a[1]*b[5] + a[2]*b[9]  + a[3]*b[13];
        res[2] = a[0]*b[2] + a[1]*b[6] + a[2]*b[10] + a[3]*b[14];
        res[3] = a[0]*b[3] + a[1]*b[7] + a[2]*b[11] + a[3]*b[15];

        res[4] = a[4]*b[0] + a[5]*b[4] + a[6]*b[8]  + a[7]*b[12];
        res[5] = a[4]*b[1] + a[5]*b[5] + a[6]*b[9]  + a[7]*b[13];
        res[6] = a[4]*b[2] + a[5]*b[6] + a[6]*b[10] + a[7]*b[14];
        res[7] = a[4]*b[3] + a[5]*b[7] + a[6]*b[11] + a[7]*b[15];

        res[8]  = a[8]*b[0] + a[9]*b[4] + a[10]*b[8]  + a[11]*b[12];
        res[9]  = a[8]*b[1] + a[9]*b[5] + a[10]*b[9]  + a[11]*b[13];
        res[10] = a[8]*b[2] + a[9]*b[6] + a[10]*b[10] + a[11]*b[14];
        res[11] = a[8]*b[3] + a[9]*b[7] + a[10]*b[11] + a[11]*b[15];

        res[12] = a[12]*b[0] + a[13]*b[4] + a[14]*b[8]  + a[15]*b[12];
        res[13] = a[12]*b[1] + a[13]*b[5] + a[14]*b[9]  + a[15]*b[13];
        res[14] = a[12]*b[2] + a[13]*b[6] + a[14]*b[10] + a[15]*b[14];
        res[15] = a[12]*b[3] + a[13]*b[7] + a[14]*b[11] + a[15]*b[15];

        Matrix4 {
            matrix: res
        }
    }

    pub fn scale(m: &Matrix4, a: f32) -> Self {
        let mut mat = Matrix4::copy(m);
        for i in 0..16 {
            mat.matrix[i] = mat.matrix[i] * a;
        }
        return mat;
    }

    pub fn add(m1: &Matrix4, m2: &Matrix4) -> Self {
        let mut res = vec![0.0; 16];

        for i in 0..16 {
            res[i] = m1.matrix[i] + m2.matrix[i];
        }
        Matrix4 {
            matrix: res
        }
    }

    pub fn sub(m1: &Matrix4, m2: &Matrix4) -> Self {
        let mut res = vec![0.0; 16];

        for i in 0..16 {
            res[i] = m1.matrix[i] - m2.matrix[i];
        }
        Matrix4 {
            matrix: res
        }
    }

    pub fn mul_vector4(m: &Matrix4, v: &Vector4) -> Vector4 {
        let a = &m.matrix;
        
        let x =  a[0]*v.x +  a[1]*v.y +  a[2]*v.z +  a[3]*v.w;
        let y =  a[4]*v.x +  a[5]*v.y +  a[6]*v.z +  a[7]*v.w;
        let z =  a[8]*v.x +  a[9]*v.y + a[10]*v.z + a[11]*v.w;
        let w = a[12]*v.x + a[13]*v.y + a[14]*v.z + a[15]*v.w;

        Vector4::new(x, y, z, w)
    }

    pub fn from_euler_zyx(x: f32, y: f32, z: f32) -> Self {
        let mut a = vec![0.0; 16];
        let x = x * PI_DIV_180;
        let y = y * PI_DIV_180;
        let z = z * PI_DIV_180;
        
        let sinx = x.sin();
        let siny = y.sin();
        let sinz = z.sin();

        let cosx = x.cos();
        let cosy = y.cos();
        let cosz = z.cos();

        a[0] = cosy * cosz;
        a[1] = sinx*siny*cosz - cosx*sinz;
        a[2] = cosx*siny*cosz + sinx*sinz;
        
        a[4] = cosy*sinz;
        a[5] = sinx*siny*sinz + cosx*cosz;
        a[6] = cosx*siny*sinz - sinx*cosz;
        
        a[8] = -siny;
        a[9] = sinx*cosy;
        a[10] = cosx*siny;
        
        a[15] = 1.0;

        Matrix4 {
            matrix: a
        }
    }

    pub fn from_scale(x: f32, y: f32, z: f32) -> Self {
        let mut a = vec![0.0; 16];
        a[0]  = x;
        a[5]  = y;
        a[10] = z;
        a[15] = 1.0;
        Matrix4 {
            matrix: a
        }
    }

    pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
        let mut a = vec![0.0; 16];
        a[0]  = 1.0;
        a[3]  = x;
        a[5]  = 1.0;
        a[7]  = y;
        a[10] = 1.0;
        a[11] = z;
        a[15] = 1.0;

        Matrix4 {
            matrix: a
        }
    }

    pub fn from_srt(x: f32, y: f32, z: f32,
                    a: f32, b: f32, c: f32,
                    u: f32, v: f32, w: f32) -> Self {
        let mut res = vec![0.0; 16];
        
        let a = a * PI_DIV_180;
        let b = b * PI_DIV_180;
        let c = c * PI_DIV_180;
        
        let sina = a.sin();
        let sinb = b.sin();
        let sinc = c.sin();

        let cosa = a.cos();
        let cosb = b.cos();
        let cosc = c.cos();

        res[0] = cosb*cosc * u;
        res[1] = (sina*sinb*cosc - cosa*sinc) * u;
        res[2] = (cosa*sinb*cosc + sina*sinc) * u;
        res[3] = x;
        
        res[4] = cosb*sinc * v;
        res[5] = (sina*sinb*sinc + cosa*cosc) * v;
        res[6] = (cosa*sinb*sinc - sina*cosc) * v;
        res[7] = y;
        
        res[8] = -sinb * w;
        res[9] = sina*cosb * w;
        res[10] = cosa*sinb * w;
        res[11] = z;
        
        res[15] = 1.0;

        Matrix4 {
            matrix: res
        }
    }

    pub fn copy(&self) -> Self {
        Matrix4 {
            matrix: self.matrix.clone()
        }
    }

    pub fn identity() -> Self {
        let mut a = vec![0.0; 16];
        a[0] = 1.0;
        a[5] = 1.0;
        a[10] = 1.0;
        a[15] = 1.0;

        Matrix4 {
            matrix: a
        }
    }
}

impl Display for Matrix4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:>3} {:>3} {:>3} {:>3}]\n[{:>3} {:>3} {:>3} {:>3}]\n[{:>3} {:>3} {:>3} {:>3}]\n[{:>3} {:>3} {:>3} {:>3}]",
        self.matrix[0], self.matrix[1], self.matrix[2], self.matrix[3],
        self.matrix[4], self.matrix[5], self.matrix[6], self.matrix[7],
        self.matrix[8], self.matrix[9], self.matrix[10], self.matrix[11],
        self.matrix[12], self.matrix[13], self.matrix[14], self.matrix[15])
    }
}*/