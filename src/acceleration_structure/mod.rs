pub mod bvh;
pub mod octree;

use crate::{
    math::{ray::Ray, triangle::Triangle},
    obj::Material,
};

pub trait AccelerationStructure {
    fn intersects(&self, ray: Ray) -> Option<(f32, &Triangle)>;
    fn build(triangles: Vec<Triangle>, materials: Vec<Material>) -> Self;
}
