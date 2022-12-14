use std::{ops, fmt::Display};

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color {
            r, g, b
        }
    }

    pub fn add(c1: &Color, c2: &Color) -> Self {
        Color {
            r: (c1.r + c2.r),
            g: (c1.g + c2.g),
            b: (c1.b + c2.b)
        }
    }

    pub fn scale(c: &Color, s: f32) -> Self {
        Color {
            r: c.r * s,
            g: c.g * s,
            b: c.b * s
        }
    }

    pub fn copy(&self) -> Self {
        Color {
            r: self.r,
            g: self.g,
            b: self.b
        }
    }

    pub fn clamp(&mut self) {
        self.r = self.r.max(0.0).min(0.999);
        self.g = self.g.max(0.0).min(0.999);
        self.b = self.b.max(0.0).min(0.999);
    }

    pub fn to_uint8_str(&self) -> String {
        let r = (self.r.sqrt() * 255.999) as u8;
        let g = (self.g.sqrt() * 255.999) as u8;
        let b = (self.b.sqrt() * 255.999) as u8;
        return format!("{} {} {}", r, g, b);
    }

    pub fn to_ycbcr(&self) -> Vec<u8> {
        let mut y  = self.r *  76.245   + self.g * 149.685  +  self.b * 29.07;
        let mut cb = self.r * -43.02768 - self.g * 84.47232 +  self.b * 127.5    + 128.0;
        let mut cr = self.r *  127.5    - self.g * 106.76544 - self.b * 20.73456 + 128.0;
        
        y  =  y.round().min(0.0).max(255.0);
        cb = cb.round().min(0.0).max(255.0);
        cr = cr.round().min(0.0).max(255.0);

        vec![y as u8, cb as u8, cr as u8]
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(mut self, rhs: Color) -> Self::Output {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
        self
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(mut self, rhs: Color) -> Self::Output {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
        self
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.r = rhs * self.r;
        self.g = rhs * self.g;
        self.b = rhs * self.b;
        self
    }
}

impl ops::Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {} {}]", self.r, self.g, self.b)
    }
}