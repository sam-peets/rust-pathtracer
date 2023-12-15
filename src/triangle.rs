use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
}

impl Triangle {
    pub fn normal(&self) -> Vec3 {
        let a: Vec3 = self.p1 - self.p0;
        let b: Vec3 = self.p2 - self.p0;

        return b.cross(a).normalize();
    }
}
