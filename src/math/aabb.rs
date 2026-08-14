use crate::math::{RayIntersects, vec4::Vec4};

pub struct Aabb {
    min: Vec4,
    max: Vec4,
}

impl Aabb {
    pub const fn new(min: Vec4, max: Vec4) -> Self {
        Self { min, max }
    }

    pub fn centroid(&self) -> Vec4 {
        (self.min + self.max)/2.0
    }

    pub fn min(&self) -> Vec4 {
        self.min
    }

    pub fn max(&self) -> Vec4 {
        self.max
    }

    pub fn union(self, other: Self) -> Self {
        let min = Vec4::new(
            self.min.x().min(other.min.x()),
            self.min.y().min(other.min.y()),
            self.min.z().min(other.min.z()),
            self.min.w().min(other.min.w()),
        );
        let max = Vec4::new(
            self.max.x().max(other.max.x()),
            self.max.y().max(other.max.y()),
            self.max.z().max(other.max.z()),
            self.max.w().max(other.max.w()),
        );
        Self { min, max }
    }

    pub fn expand(self, point: Vec4) -> Self {
        let min = Vec4::new(
            self.min.x().min(point.x()),
            self.min.y().min(point.y()),
            self.min.z().min(point.z()),
            self.min.w().min(point.w()),
        );
        let max = Vec4::new(
            self.max.x().max(point.x()),
            self.max.y().max(point.y()),
            self.max.z().max(point.z()),
            self.max.w().max(point.w()),
        );
        Self { min, max }
    }

    pub fn contains(&self, point: Vec4) -> bool {
        point.x() >= self.min.x()
            && point.x() <= self.max.x()
            && point.y() >= self.min.y()
            && point.y() <= self.max.y()
            && point.z() >= self.min.z()
            && point.z() <= self.max.z()
            && point.w() >= self.min.w()
            && point.w() <= self.max.w()
    }
}

impl RayIntersects for Aabb {
    fn intersects(&self, ray: super::ray::Ray) -> Option<f32> {
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;

        let axes = [
            (self.min.x(), self.max.x(), ray.origin.x(), ray.direction.x()),
            (self.min.y(), self.max.y(), ray.origin.y(), ray.direction.y()),
            (self.min.z(), self.max.z(), ray.origin.z(), ray.direction.z()),
        ];

        for (lo, hi, o, d) in axes {
            if d.abs() < f32::EPSILON {
                if o < lo || o > hi {
                    return None;
                }
                continue;
            }

            let t1 = (lo - o) / d;
            let t2 = (hi - o) / d;
            let (near, far) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };

            if near > tmin {
                tmin = near;
            }
            if far < tmax {
                tmax = far;
            }

            if tmin > tmax {
                return None;
            }
        }

        if tmin > f32::EPSILON {
            Some(tmin)
        } else if tmax > f32::EPSILON {
            Some(tmax)
        } else {
            None
        }
    }
}
