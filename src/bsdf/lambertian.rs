use crate::{bsdf::Bsdf, math::vec4::Vec4};

pub struct Lambertian {
    pub albedo: Vec4,
}

impl Bsdf for Lambertian {
    fn eval(&self, incoming: Vec4, outgoing: Vec4, normal: Vec4) -> Vec4 {
        self.albedo / std::f32::consts::PI
    }

    fn sample(&self, incoming: Vec4, normal: Vec4, rng: &mut fastrand::Rng) -> Vec4 {
        // cosine-weighted hemisphere
        let e0 = rng.f32();
        let e1 = rng.f32();

        let theta = e0.sqrt().acos();
        let phi = 2.0 * std::f32::consts::PI * e1;

        let x = theta.sin() * phi.cos();
        let y = theta.sin() * phi.sin();
        let z = theta.cos();

        // Transform the sampled direction to world space using the normal
        let tangent = if normal.x().abs() > 0.1 {
            Vec4::from([0.0, 1.0, 0.0, 0.0]).cross(normal).normalize()
        } else {
            Vec4::from([1.0, 0.0, 0.0, 0.0]).cross(normal).normalize()
        };
        let bitangent = normal.cross(tangent);

        // Transform the local direction to world space
        let direction = tangent * x + bitangent * y + normal * z;
        direction
    }

    fn pdf(&self, incoming: Vec4, outgoing: Vec4, normal: Vec4) -> f32 {
        let cos_theta = normal.dot(outgoing).max(0.0);
        cos_theta / std::f32::consts::PI
    }
}
