use crate::math::ray::Ray;
use crate::math::vec4::Vec4;

pub struct Camera {
    pos: Ray,
    focal_length: f32,
}

impl Camera {
    pub fn new(pos: Ray, focal_length: f32) -> Self {
        Self { pos, focal_length }
    }

    pub fn gen_rays(&self, width: usize, height: usize) -> Vec<Ray> {
        let forward = self.pos.direction.normalize();
        let right = Vec4::from([0.0, 1.0, 0.0, 0.0]).cross(forward).normalize();
        let up = right.cross(forward);

        let aspect = width as f32 / height as f32;

        let mut rays = vec![];

        for y in 0..height {
            let y = y as f32;
            let height = height as f32;

            for x in 0..width {
                let x = x as f32;
                let width = width as f32;

                let u = -((2.0 * (x + 0.5) / width - 1.0) * aspect);
                let v = -(1.0 - 2.0 * (y + 0.5) / height);

                let dir = right * u + up * v + forward * self.focal_length;

                rays.push(Ray::new(self.pos.origin, dir));
            }
        }

        rays
    }
}
