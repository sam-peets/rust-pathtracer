use crate::{
    acceleration_structure::AccelerationStructure,
    math::{RayIntersects, aabb::Aabb, ray::Ray, triangle::Triangle, vec4::Vec4},
    obj::Material,
};

pub struct Bvh {
    pub root: BvhNode,
    pub materials: Vec<Material>,
}

impl AccelerationStructure for Bvh {
    fn intersects(&self, ray: Ray) -> Option<(f32, &Triangle)> {
        self.root.intersects(ray)
    }

    fn build(triangles: Vec<Triangle>, materials: Vec<Material>) -> Self {
        let root = BvhNode::build(triangles, 0);
        Self { root, materials }
    }
}

enum BvhData {
    Internal {
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
    Leaf {
        triangles: Vec<Triangle>,
    },
}

const MAX_DEPTH: usize = 32;
const MAX_TRIANGLES: usize = 8;

struct BvhNode {
    pub aabb: Aabb,
    data: BvhData,
}

impl BvhNode {
    pub fn intersects(&self, ray: Ray) -> Option<(f32, &Triangle)> {
        let mut best = (f32::INFINITY, None);
        self.walk(ray, &mut best);
        if let (t, Some(tri)) = best {
            Some((t, tri))
        } else {
            None
        }
    }

    fn walk<'a>(&'a self, ray: Ray, best: &mut (f32, Option<&'a Triangle>)) {
        match &self.data {
            BvhData::Internal { left, right } => {
                let t_left = left.aabb.intersects(ray);
                let t_right = right.aabb.intersects(ray);

                match (t_left, t_right) {
                    (None, None) => (),
                    (None, Some(t)) => {
                        if t < best.0 {
                            right.walk(ray, best);
                        }
                    }
                    (Some(t), None) => {
                        if t < best.0 {
                            left.walk(ray, best);
                        }
                    }
                    (Some(tl), Some(tr)) => {
                        if tl < tr {
                            left.walk(ray, best);
                            right.walk(ray, best);
                        } else {
                            right.walk(ray, best);
                            left.walk(ray, best);
                        }
                    }
                }
            }
            BvhData::Leaf { triangles } => {
                for tri in triangles {
                    if let Some(t) = tri.intersects(ray)
                        && t < best.0
                    {
                        *best = (t, Some(tri))
                    }
                }
            }
        }
    }

    pub fn build(triangles: Vec<Triangle>, depth: usize) -> Self {
        let aabb = triangles
            .iter()
            .map(|tri| tri.aabb())
            .reduce(Aabb::union)
            .unwrap_or(Aabb::new(
                Vec4::new(0.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 0.0),
            ));

        if depth >= MAX_DEPTH || triangles.len() <= MAX_TRIANGLES {
            return BvhNode {
                aabb,
                data: BvhData::Leaf { triangles },
            };
        }

        let len_x = (aabb.max().x() - aabb.min().x()).abs();
        let len_y = (aabb.max().y() - aabb.min().y()).abs();
        let len_z = (aabb.max().z() - aabb.min().z()).abs();
        let max_axis = len_x.max(len_y).max(len_z);

        let num_triangles = triangles.len();

        let centroids = triangles.iter().map(|t| t.centroid());

        let (lhs, rhs): (Vec<Triangle>, Vec<Triangle>) = if len_x == max_axis {
            let mut x_centroid: Vec<f32> = centroids.map(|c| c.x()).collect();

            let (_, median, _) =
                x_centroid.select_nth_unstable_by(num_triangles / 2, |a, b| a.total_cmp(b));

            triangles
                .clone()
                .into_iter()
                .partition(|t| t.centroid().x() <= *median)
        } else if len_y == max_axis {
            let mut y_centroid: Vec<f32> = centroids.map(|c| c.y()).collect();

            let (_, median, _) =
                y_centroid.select_nth_unstable_by(num_triangles / 2, |a, b| a.total_cmp(b));

            triangles
                .clone()
                .into_iter()
                .partition(|t| t.centroid().y() <= *median)
        } else {
            let mut z_centroid: Vec<f32> = centroids.map(|c| c.z()).collect();

            let (_, median, _) =
                z_centroid.select_nth_unstable_by(num_triangles / 2, |a, b| a.total_cmp(b));

            triangles
                .clone()
                .into_iter()
                .partition(|t| t.centroid().z() <= *median)
        };

        let left = Box::new(Self::build(lhs, depth + 1));
        let right = Box::new(Self::build(rhs, depth + 1));

        Self {
            aabb,
            data: BvhData::Internal { left, right },
        }
    }
}
