use crate::{bsdf::Bsdf, math::vec4::Vec4};

pub struct Lambertian {
    albedo: Vec4,
}

impl Bsdf for Lambertian {
    fn eval(&self, incoming: Vec4, outgoing: Vec4, normal: Vec4) -> f32 {
        todo!()
    }

    fn sample(&self, incoming: Vec4, normal: Vec4) -> Vec4 {
        todo!()
    }

    fn pdf(&self, incoming: Vec4, outgoing: Vec4, normal: Vec4) -> f32 {
        todo!()
    }
}
