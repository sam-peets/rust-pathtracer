use crate::vec4::{NVec4, Vec4};

#[derive(Copy, Clone)]
pub struct Triangle {
    pub p0: NVec4,
    pub p1: NVec4,
    pub p2: NVec4,
}

impl Triangle {
    pub fn normal(&self) -> Vec4 {
        let a: Vec4 = self.p1.v - self.p0.v;
        let b: Vec4 = self.p2.v - self.p0.v;

        return a.cross(b).normalize();
    }

    pub fn normal_interp(&self, p: &Vec4) -> Vec4 {
        let v0 = self.p1.v - self.p0.v;
        let v1 = self.p2.v - self.p0.v;
        let v2 = *p - self.p0.v;

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);

        let denom = d00 * d11 - d01 * d01;

        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1. - v - w;

        return (self.p0.n * u + self.p1.n * v + self.p2.n * w).normalize();
    }

    pub fn normal3p(&self) -> Vec4 {
        return (self.p0.n + self.p1.n + self.p2.n).normalize();
    }
}
