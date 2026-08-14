use crate::math::ray::Ray;

pub mod aabb;
pub mod mat4;
pub mod ray;
pub mod triangle;
pub mod vec4;

pub trait RayIntersects {
    fn intersects(&self, ray: Ray) -> Option<f32>;
}
