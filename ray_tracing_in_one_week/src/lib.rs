use std::fmt;
use std::ops;
pub struct vec3 {
    e: [f64; 3],
}
impl Default for vec3 {
    fn default() -> Self {
        vec3 { e: [0.0, 0.0, 0.0] }
    }
}
impl ops::Sub for vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}
impl ops::Index<usize> for vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= 3 {
            panic!("the index is only allowed between [0, 2]")
        }
        &self.e[index]
    }
}
impl ops::IndexMut<usize> for vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        if index >= 3 {
            panic!("the index is only allowed between [0, 2]")
        }
        &mut self.e[index]
    }
}
impl ops::AddAssign for vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.e[i] += rhs.e[i];
        }
    }
}
impl ops::MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.e[i] *= rhs;
        }
    }
}

impl ops::DivAssign<f64> for vec3 {
    fn div_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            panic!("the divisor cannot be 0.0")
        }
        for i in 0..3 {
            self.e[i] /= rhs;
        }
    }
}

impl fmt::Display for vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl ops::Add for vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}
impl ops::Neg for vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
impl ops::Mul for vec3 {
    type Output = vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<vec3> for f64 {
    type Output = vec3;
    fn mul(self, rhs: vec3) -> Self::Output {
        vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

impl ops::Mul<f64> for vec3 {
    type Output = vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for vec3 {
    type Output = vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        vec3 { e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    fn length_squared(&self) -> f64 {
        let mut res = 0.0;
        for i in 0..3 {
            res += self.e[i] * self.e[i];
        }
        res
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(self) -> Self {
        let len = self.length();
        self / len
    }
}
pub fn dot_vec3(u: &vec3, v: &vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
pub fn cross_vec3(u: &vec3, v: &vec3) -> vec3 {
    vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub use vec3 as point3;
pub use vec3 as color;

pub fn write_color(pixel_color: color) {
    let multi = 255.999;
    print!(
        "{} {} {}\n",
        (multi * pixel_color.x()) as i32,
        (multi * pixel_color.y()) as i32,
        (multi * pixel_color.z()) as i32
    );
}
