pub mod cook_torrance;
pub mod lambertian;

use crate::math::vec4::Vec4;

pub trait Bsdf {
    fn eval(&self, wi: Vec4, wo: Vec4, normal: Vec4) -> Vec4;
    fn sample(&self, wi: Vec4, normal: Vec4, rng: &mut fastrand::Rng) -> Vec4;
    fn pdf(&self, wi: Vec4, wo: Vec4, normal: Vec4) -> f32;
}
