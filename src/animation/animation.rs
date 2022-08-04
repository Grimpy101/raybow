use crate::math::{vector2::Vector2, vector3::Vector3};

pub enum Interpolation {
    Constant,
    Linear,
    Bezier(Vec<Vector2>)
}

pub struct AnimationKey {
    point: Vector2,
    interpolation: Interpolation
}

impl AnimationKey {
    pub fn new(x: f32, y: f32) -> Self {
        AnimationKey {
            point: Vector2 {x, y},
            interpolation: Interpolation::Linear
        }
    }

    pub fn change_interpolation(&mut self, interp: Interpolation) {
        self.interpolation = interp;
    }
}

pub struct AnimationChannel {
    keys: Vec<AnimationKey>
}

impl AnimationChannel {
    pub fn new() -> Self {
        AnimationChannel {
            keys: Vec::new()
        }
    }

    pub fn add_key(&mut self, k: AnimationKey) {
        self.keys.push(k);
    }

    pub fn get_value_at_frame(&self, f: f32) -> Option<f32> {
        if self.keys.is_empty() {
            return None;
        }

        let mut first = self.keys.first().unwrap();
        let mut last = self.keys.last().unwrap();

        for i in &self.keys {
            if i.point.x < f {
                first = &i;
            }
            else if i.point.x == f {
                return Some(i.point.y);
            }
            else {
                last = &i;
                break;
            }
        }

        if last.point.x == first.point.x {
            return Some(first.point.y);
        }

        let t = (f - first.point.x) / (last.point.x - first.point.x);

        match &first.interpolation {
            Interpolation::Constant => {
                return Some(first.point.y)
            },
            Interpolation::Linear => {
                let y = AnimationChannel::linear_interpolate(&first.point, &last.point, t);
                return Some(y.y);
            },
            Interpolation::Bezier(control_points_first) => {
                let p0 = &first.point;
                let p1 = &control_points_first[1];
                // TODO: Fix this later!
                let p2 = match &last.interpolation {
                    Interpolation::Bezier(control_points_last) => &control_points_last[0],
                    _ => &Vector2 {x: 0.0, y: 0.0}
                };
                let p3 = &last.point;

                let a = AnimationChannel::linear_interpolate(&p0, &p1, t);
                let b = AnimationChannel::linear_interpolate(&p1, &p2, t);
                let c = AnimationChannel::linear_interpolate(&p2, &p3, t);
                
                let d = AnimationChannel::linear_interpolate(&a, &b, t);
                let e = AnimationChannel::linear_interpolate(&b, &c, t);

                let f = AnimationChannel::linear_interpolate(&d, &e, t);
                return Some(f.y);
            }
        }
    }

    pub fn get_vector_by_frame(c1: Option<&AnimationChannel>, c2: Option<&AnimationChannel>,
            c3: Option<&AnimationChannel>, default: &Vector3, f: f32) -> Vector3 {
        let mut x = default.x;
        let mut y = default.y;
        let mut z = default.z;

        if c1.is_some() {
            let res = c1.unwrap().get_value_at_frame(f);
            if res.is_some() {
                x = res.unwrap();
            }
        }

        if c2.is_some() {
            let res = c2.unwrap().get_value_at_frame(f);
            if res.is_some() {
                y = res.unwrap();
            }
        }

        if c3.is_some() {
            let res = c3.unwrap().get_value_at_frame(f);
            if res.is_some() {
                z = res.unwrap();
            }
        }

        return Vector3::new(x, y, z);
    }

    fn linear_interpolate(a: &Vector2, b: &Vector2, t: f32) -> Vector2 {
        Vector2 {
            x: a.x - ((a.x - b.x) * t),
            y: a.y - ((a.y - b.y) * t)
        }
    }
}