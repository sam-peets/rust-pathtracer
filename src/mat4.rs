use crate::vec4::Vec4;
use std::ops;

#[derive(Copy, Clone)]
pub struct Mat4 {
    // row major
    pub m: [Vec4; 4],
}

impl Mat4 {
    pub fn identity() -> Mat4 {
        let mut m = [Vec4::new(0., 0., 0., 0.); 4];
        m[0] = Vec4::new(1., 0., 0., 0.);
        m[1] = Vec4::new(0., 1., 0., 0.);
        m[2] = Vec4::new(0., 0., 1., 0.);
        m[3] = Vec4::new(0., 0., 0., 1.);
        return Mat4 { m: m };
    }

    pub fn column(&self, c: usize) -> Vec4 {
        let x = self.m[0].elem(c);
        let y = self.m[1].elem(c);
        let z = self.m[2].elem(c);
        let w = self.m[3].elem(c);
        return Vec4::new(x, y, z, w);
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Self::Output {
        let x = self.m[0].dot(other);
        let y = self.m[1].dot(other);
        let z = self.m[2].dot(other);
        let w = self.m[3].dot(other);

        return Vec4::new(x, y, z, w);
    }
}

impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Self::Output {
        let mut m = [Vec4::new(0., 0., 0., 0.); 4];

        for i in 0..4 {
            m[i].x = self.m[i].dot(other.column(0));
            m[i].y = self.m[i].dot(other.column(1));
            m[i].z = self.m[i].dot(other.column(2));
            m[i].w = self.m[i].dot(other.column(3));
        }

        return Mat4 { m: m };
    }
}
