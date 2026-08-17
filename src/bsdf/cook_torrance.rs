use core::f32;

use crate::{bsdf::Bsdf, math::vec4::Vec4};

pub struct CookTorrance {
    pub albedo: Vec4,
    pub roughness: f32,
    pub ior: f32,
    pub metallic: f32,
}

impl CookTorrance {
    fn d_blinn_phong(&self, wh: Vec4, normal: Vec4) -> f32 {
        let alpha = self.roughness * self.roughness;
        let alpha2 = alpha * alpha;
        let h_dot_n = wh.dot(normal).max(0.0);

        1.0 / (f32::consts::PI * alpha2) * h_dot_n.powf(2.0 / alpha2 - 2.0)
    }

    fn d_ggx(&self, wh: Vec4, normal: Vec4) -> f32 {
        let roughness = self.roughness.max(0.003);
        let alpha = roughness * roughness;
        let alpha2 = alpha * alpha;

        let n_dot_m = normal.dot(wh);
        let tan2_theta_m = -(n_dot_m * n_dot_m - 1.0) / (n_dot_m * n_dot_m);
        let denom = n_dot_m * n_dot_m * (alpha2 + tan2_theta_m);

        if n_dot_m < 0.0 {
            0.0
        } else {
            alpha2 / (f32::consts::PI * denom * denom)
        }
    }
    fn g_ggx_g1(&self, wx: Vec4, wh: Vec4, normal: Vec4) -> f32 {
        let roughness = self.roughness.min(0.003);
        let alpha = roughness * roughness;
        let alpha2 = alpha * alpha;
        let x_dot_h = wx.dot(wh);
        let x_dot_n = wx.dot(normal);

        let mu2 = x_dot_n * x_dot_n;
        let tan2_theta_x = (1.0 - mu2) / mu2;

        let denom = 1.0 + (1.0 + alpha2 * tan2_theta_x).sqrt();
        if (x_dot_h / x_dot_n) < 0.0 {
            0.0
        } else {
            (2.0 / denom).max(0.0)
        }
    }
    fn g_ggx(&self, wi: Vec4, wo: Vec4, normal: Vec4) -> f32 {
        let half = (wi + wo).normalize();
        self.g_ggx_g1(wi, half, normal) * self.g_ggx_g1(wo, half, normal)
    }

    // height-correlated smith
    fn g_smith(&self, wi: Vec4, wo: Vec4, normal: Vec4, lambda: fn(Vec4) -> f32) -> f32 {
        let wh = (wi + wo).normalize();

        todo!()
    }

    fn f_schlick(&self, wi: Vec4, wo: Vec4) -> f32 {
        let wh = (wi + wo).normalize();
        let f0 = (self.ior - 1.0).powi(2) / (self.ior + 1.0).powi(2);
        let v_dot_h = wi.dot(wh).clamp(0.0, 1.0);

        f0 + (1.0 - f0) * (1.0 - v_dot_h).powi(5)
    }
}

impl Bsdf for CookTorrance {
    fn eval(&self, wi: Vec4, wo: Vec4, normal: Vec4) -> Vec4 {
        let wh = (wi + wo).normalize();
        let d = self.d_ggx(wh, normal);
        let g = self.g_ggx(wi, wo, normal);
        let f = self.f_schlick(wi, wo);

        let n_dot_l = normal.dot(wo);
        let n_dot_v = normal.dot(wi);
        if n_dot_l <= 0.0 || n_dot_v <= 0.0 {
            return Vec4::new(0.0, 0.0, 0.0, 0.0);
        }

        let ks = (f * (d * g)) / (4.0 * n_dot_l * n_dot_v);
        let kd = self.albedo / f32::consts::PI * (1.0 - f);

        (Vec4::new(ks, ks, ks, ks) + kd) * n_dot_l
    }

    fn sample(&self, wi: Vec4, normal: Vec4, rng: &mut fastrand::Rng) -> Vec4 {
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
        tangent * x + bitangent * y + normal * z
    }

    fn pdf(&self, wi: Vec4, wo: Vec4, normal: Vec4) -> f32 {
        let cos_theta = normal.dot(wo).max(0.0);
        cos_theta / std::f32::consts::PI
    }
}
