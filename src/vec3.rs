use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Vec3 {
        (*self) * (1. / self.length())
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: Self) -> Vec3 {
        let a2b1: f64 = self.y * other.x;
        let a3b1: f64 = self.z * other.x;

        let a1b2: f64 = self.x * other.y;
        let a3b2: f64 = self.z * other.y;

        let a1b3: f64 = self.x * other.z;
        let a2b3: f64 = self.y * other.z;

        return Vec3 {
            x: a2b3 - a3b2,
            y: a3b1 - a1b3,
            z: a1b2 - a2b1,
        };
    }

    pub fn pow(&self, other: f64) -> Vec3 {
        return Vec3 {
            x: self.x.powf(other),
            y: self.y.powf(other),
            z: self.z.powf(other),
        }
    }


    pub fn as_rgb(&self) -> Vec3 {
        let r: f64 = (self.x * 255.).floor().clamp(0., 255.);
        let g: f64 = (self.y * 255.).floor().clamp(0., 255.);
        let b: f64 = (self.z * 255.).floor().clamp(0., 255.);
        return Vec3 { x: r, y: g, z: b };
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Vec3 {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = (*self) + other;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Vec3 {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = (*self) - other;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    // term-by-term multiplication (like glsl)

    fn mul(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = (*self) * other;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = (*self) / other;
    }
}
