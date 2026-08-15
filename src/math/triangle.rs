use crate::math::{RayIntersects, aabb::Aabb, mat4::Mat4, ray::Ray, vec4::Vec4};

#[derive(Debug, Clone)]
pub struct Triangle {
    p1: Vec4,
    p2: Vec4,
    p3: Vec4,

    // internal
    n1: Vec4,
    n2: Vec4,
    n3: Vec4,
}

impl Triangle {
    pub fn new(p1: Vec4, p2: Vec4, p3: Vec4) -> Self {
        let e1 = p3 - p1;
        let e2 = p3 - p2;
        let normal = e1.cross(e2).normalize();
        Self {
            p1,
            p2,
            p3,
            n1: normal,
            n2: normal,
            n3: normal,
        }
    }

    pub fn new_with_normals(p1: Vec4, p2: Vec4, p3: Vec4, n1: Vec4, n2: Vec4, n3: Vec4) -> Self {
        Self {
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
        }
    }

    pub fn normal(&self, point: Vec4) -> Vec4 {
        let v0 = self.p2 - self.p1;
        let v1 = self.p3 - self.p1;
        let v2 = point - self.p1;
        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);

        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        (self.n1 * u + self.n2 * v + self.n3 * w).normalize()
    }

    pub fn centroid(&self) -> Vec4 {
        (self.p1 + self.p2 + self.p3) / 3.0
    }

    pub fn apply(self, matrix: Mat4) -> Self {
        let p1 = matrix * self.p1;
        let p2 = matrix * self.p2;
        let p3 = matrix * self.p3;

        let inv_t = matrix.inverse().transpose();
        let n1 = (inv_t * self.n1).normalize();
        let n2 = (inv_t * self.n2).normalize();
        let n3 = (inv_t * self.n3).normalize();

        Self::new_with_normals(p1, p2, p3, n1, n2, n3)
    }

    pub fn aabb(&self) -> Aabb {
        let min = Vec4::new(
            self.p1.x().min(self.p2.x()).min(self.p3.x()),
            self.p1.y().min(self.p2.y()).min(self.p3.y()),
            self.p1.z().min(self.p2.z()).min(self.p3.z()),
            self.p1.w().min(self.p2.w()).min(self.p3.w()),
        );
        let max = Vec4::new(
            self.p1.x().max(self.p2.x()).max(self.p3.x()),
            self.p1.y().max(self.p2.y()).max(self.p3.y()),
            self.p1.z().max(self.p2.z()).max(self.p3.z()),
            self.p1.w().max(self.p2.w()).max(self.p3.w()),
        );
        Aabb::new(min, max)
    }
}

impl RayIntersects for Triangle {
    fn intersects(&self, ray: Ray) -> Option<f32> {
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
}
