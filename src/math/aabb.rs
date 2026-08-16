use crate::math::{RayIntersects, ray::Ray, vec4::Vec4};

pub struct Aabb {
    min: Vec4,
    max: Vec4,
}

impl Aabb {
    pub const fn new(min: Vec4, max: Vec4) -> Self {
        Self { min, max }
    }

    pub fn centroid(&self) -> Vec4 {
        (self.min + self.max) / 2.0
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
    fn intersects(&self, ray: Ray) -> Option<f32> {
        let ray_inv = 1.0 / ray.direction;
        let t1 = (self.min - ray.origin) * ray_inv;
        let t2 = (self.max - ray.origin) * ray_inv;

        let tmin = t1
            .x()
            .min(t2.x())
            .max(t1.y().min(t2.y()))
            .max(t1.z().min(t2.z()))
            .max(0.0);
        let tmax = t1
            .x()
            .max(t2.x())
            .min(t1.y().max(t2.y()))
            .min(t1.z().max(t2.z()));

        (tmax >= tmin).then_some(tmin)
    }
}
