use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct NVec4 {
    pub v: Vec4,
    pub n: Vec4,
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    pub fn normalize(&self) -> Vec4 {
        (*self) * (1. / self.length())
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    #[inline]
    pub fn cross(&self, other: Self) -> Vec4 {
        // 3d cross product with w=0
        let a2b1: f64 = self.y * other.x;
        let a3b1: f64 = self.z * other.x;

        let a1b2: f64 = self.x * other.y;
        let a3b2: f64 = self.z * other.y;

        let a1b3: f64 = self.x * other.z;
        let a2b3: f64 = self.y * other.z;

        return Vec4 {
            x: a2b3 - a3b2,
            y: a3b1 - a1b3,
            z: a1b2 - a2b1,
            w: 0.,
        };
    }

    pub fn pow(&self, other: f64) -> Vec4 {
        return Vec4 {
            x: self.x.powf(other),
            y: self.y.powf(other),
            z: self.z.powf(other),
            w: self.w.powf(other),
        };
    }

    pub fn as_rgb(&self) -> Vec4 {
        // clamp to [0,1], apply gamma correction, multiply by 255, round down
        let r: f64 = (self.x.clamp(0.,1.).powf(1./2.2) * 255.0).floor();
        let g: f64 = (self.y.clamp(0.,1.).powf(1./2.2) * 255.0).floor();
        let b: f64 = (self.z.clamp(0.,1.).powf(1./2.2) * 255.0).floor();
        let a: f64 = (self.w.clamp(0.,1.).powf(1./2.2) * 255.0).floor();
        return Vec4 {
            x: r,
            y: g,
            z: b,
            w: a,
        };
    }

    pub fn elem(&self, c: usize) -> f64 {
        match c {
            0 => return self.x,
            1 => return self.y,
            2 => return self.z,
            3 => return self.w,
            _ => panic!("not a vector component"),
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec4{{{}, {}, {}, {}}}", self.x, self.y, self.z, self.w)
    }
}

impl ops::Add for Vec4 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Vec4 {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        *self = (*self) + other;
    }
}

impl ops::Sub for Vec4 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Vec4 {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        *self = (*self) - other;
    }
}

impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl ops::Mul<Vec4> for Vec4 {
    type Output = Vec4;

    // term-by-term multiplication (like glsl)

    fn mul(self, other: Vec4) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl ops::MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, other: f64) {
        *self = (*self) * other;
    }
}

impl ops::Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl ops::DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, other: f64) {
        *self = (*self) / other;
    }
}
