use crate::vec4::Vec4;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub p0: Vec4,
    pub p1: Vec4,
    pub p2: Vec4,
}

impl Triangle {
    pub fn normal(&self) -> Vec4 {
        let a: Vec4 = self.p1 - self.p0;
        let b: Vec4 = self.p2 - self.p0;

        return b.cross(a).normalize();
    }
}
