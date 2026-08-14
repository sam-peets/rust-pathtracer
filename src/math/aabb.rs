use crate::math::{RayIntersects, vec4::Vec4};

pub struct Aabb {
    min: Vec4,
    max: Vec4,
}

impl RayIntersects for Aabb {
    fn intersects(&self, ray: super::ray::Ray) -> Option<f32> {
        todo!()
    }
}
