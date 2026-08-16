pub mod cook_torrance;
pub mod lambertian;

use crate::math::vec4::Vec4;

pub trait Bsdf {
    fn eval(&self, incoming: Vec4, outgoing: Vec4, normal: Vec4) -> Vec4;
    fn sample(&self, incoming: Vec4, normal: Vec4, rng: &mut fastrand::Rng) -> Vec4;
    fn pdf(&self, incoming: Vec4, outgoing: Vec4, normal: Vec4) -> f32;
}
