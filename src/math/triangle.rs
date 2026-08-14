use crate::math::{mat4::Mat4, ray::Ray, vec4::Vec4};

#[derive(Debug, Clone)]
pub struct Triangle {
    p1: Vec4,
    p2: Vec4,
    p3: Vec4,
}

impl Triangle {
    pub fn new(p1: Vec4, p2: Vec4, p3: Vec4) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn centroid(&self) -> Vec4 {
        (self.p1 + self.p2 + self.p3)/3.0
    }

    pub fn intersects(&self, ray: Ray) -> Option<f32> {
        let e1 = self.p2 - self.p1;
        let e2 = self.p3 - self.p1;

        let ray_cross_e2 = ray.direction.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.p1;
        let u = inv_det * s.dot(ray_cross_e2);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * ray.direction.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * e2.dot(s_cross_e1);
        if t > f32::EPSILON { Some(t) } else { None }
    }

    pub fn apply(self, matrix: Mat4) -> Self {
        let p1 = matrix * self.p1;
        let p2 = matrix * self.p2;
        let p3 = matrix * self.p3;

        return Self {
            p1, p2, p3
        }
    }
}
