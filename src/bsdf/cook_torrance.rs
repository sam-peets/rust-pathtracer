use std::ops::DivAssign;

use crate::{bsdf::Bsdf, math::vec4::Vec4};

/// Clamped away from zero so that a perfect mirror doesn't send `ggx` to 0/0.
const MIN_ROUGHNESS: f32 = 0.03;

pub struct CookTorrance {
    pub albedo: Vec4,
    pub roughness: f32,
    pub ior: f32,
    pub metallic: f32,
}

impl CookTorrance {
    fn alpha(&self) -> f32 {
        let roughness = self.roughness.clamp(MIN_ROUGHNESS, 1.0);
        roughness * roughness
    }

    fn ggx(&self, half: Vec4, normal: Vec4) -> f32 {
        let alpha = self.alpha();
        let alpha2 = alpha * alpha;

        let cos_theta = normal.dot(half).max(0.0);
        let cos_theta2 = cos_theta * cos_theta;

        let denom = cos_theta2 * (alpha2 - 1.0) + 1.0;
        alpha2 / (std::f32::consts::PI * denom * denom)
    }

    /// Height-correlated Smith masking-shadowing, with the 1/(4 cos_i cos_o)
    /// factor of the microfacet denominator folded in so grazing angles can't
    /// divide by zero.
    fn smith_v(&self, incoming: Vec4, outgoing: Vec4, normal: Vec4) -> f32 {
        let alpha = self.alpha();
        let alpha2 = alpha * alpha;

        let cos_i = normal.dot(incoming).max(0.0);
        let cos_o = normal.dot(outgoing).max(0.0);

        let lambda_i = (alpha2 + (1.0 - alpha2) * cos_i * cos_i).sqrt();
        let lambda_o = (alpha2 + (1.0 - alpha2) * cos_o * cos_o).sqrt();

        let denom = cos_o * lambda_i + cos_i * lambda_o;
        if denom <= 0.0 { 0.0 } else { 0.5 / denom }
    }

    fn schlick(&self, half: Vec4, outgoing: Vec4) -> Vec4 {
        let dielectric = (self.ior - 1.0).powi(2) / (self.ior + 1.0).powi(2);
        let f0 = Vec4::new(dielectric, dielectric, dielectric, dielectric);
        let f0 = f0 + (self.albedo - f0) * self.metallic;
        let cos_theta = half.dot(outgoing).max(0.0);
        f0 + (Vec4::new(1.0, 1.0, 1.0, 1.0) - f0) * (1.0 - cos_theta).powi(5)
    }
}

impl Bsdf for CookTorrance {
    fn eval(&self, incoming: Vec4, outgoing: Vec4, normal: Vec4) -> Vec4 {
        if normal.dot(incoming) <= 0.0 || normal.dot(outgoing) <= 0.0 {
            return Vec4::new(0.0, 0.0, 0.0, 0.0);
        }

        let half = (incoming + outgoing).normalize();
        let d = self.ggx(half, normal);
        let v = self.smith_v(incoming, outgoing, normal);
        let f = self.schlick(half, outgoing);

        let specular = f * (d * v);
        let diffuse = self.albedo * ((Vec4::new(1.0, 1.0, 1.0, 1.0) - f) / std::f32::consts::PI);

        diffuse + specular
    }

    fn sample(&self, incoming: Vec4, normal: Vec4, rng: &mut fastrand::Rng) -> Vec4 {
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
        // let half = (incoming + outgoing).normalize();
        // self.ggx(half, normal)

        let cos_theta = normal.dot(outgoing).max(0.0);
        cos_theta / std::f32::consts::PI
    }
}
