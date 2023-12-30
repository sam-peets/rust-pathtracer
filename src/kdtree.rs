use crate::aabb::AABB;
use crate::tracer::Ray;
use crate::triangle::Triangle;
use crate::vec4::Vec4;
use std::sync::{Arc, Mutex};

const MAX_DEPTH: usize = 8;

pub struct KDNode {
    pub aabb: AABB,
    pub lt: Option<Arc<KDNode>>,
    pub gt: Option<Arc<KDNode>>,
    pub triangles: Option<Vec<Triangle>>,
}

impl KDNode {
    pub fn new(triangles: &Vec<Triangle>, depth: usize, aabb: &AABB) -> KDNode {
        if triangles.len() == 0 {
            return KDNode {
                aabb: *aabb,
                lt: None,
                gt: None,
                triangles: None,
            };
        }
        if (depth == MAX_DEPTH) {
            let mut nt: Vec<Triangle> = Vec::new();
            for t in triangles {
                nt.push(*t);
            }
            return KDNode {
                aabb: *aabb,
                lt: None,
                gt: None,
                triangles: Some(nt),
            };
        }
        let axis = depth % 3;
        let mut midpoints: Vec<f64> = Vec::new();
        for t in triangles {
            midpoints.push(t.midpoint().elem(axis));
        }
        midpoints.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median: f64 = midpoints[midpoints.len() / 2]; // TODO: use linear time median

        let mut lt_v: Vec4 = aabb.max;
        lt_v.set_elem(axis, median);
        let mut gt_v: Vec4 = aabb.min;
        gt_v.set_elem(axis, median);
        let epsilon = Vec4::new(0.001,0.001,0.001,0.);

        let bb_lt = AABB {
            min: aabb.min - epsilon,
            max: lt_v + epsilon,
        };

        let bb_gt = AABB {
            min: gt_v - epsilon,
            max: aabb.max + epsilon,
        };

        let mut tlt: Vec<Triangle> = Vec::new();
        let mut tgt: Vec<Triangle> = Vec::new();

        for t in triangles {
            if bb_lt.intersect_triangle(&t) {
                tlt.push(*t);
            }
            if bb_gt.intersect_triangle(&t) {
                tgt.push(*t);
            }
        }

        println!("median {}: {} on {}", depth, median, axis);
        println!("tlt: {}, tgt: {}", tlt.len(), tgt.len());
        return KDNode {
            aabb: *aabb,
            lt: Some(Arc::new(Self::new(&tlt, depth + 1, &bb_lt))),
            gt: Some(Arc::new(Self::new(&tgt, depth + 1, &bb_gt))),
            triangles: None,
        };
    }

    pub fn ray_leaf(&self, r: &Ray) -> Vec<Triangle> {
        let mut leaf_triangles: Vec<Triangle> = Vec::new();
        if self.triangles.is_none() {
            if self.lt.as_ref().is_some() {
                let alt = Arc::clone(&(self.lt.as_ref().unwrap()));
                if alt.aabb.intersect_ray(r) {
                    leaf_triangles.extend(alt.ray_leaf(r));
                }
            }

            if self.lt.as_ref().is_some() {
                let agt = Arc::clone(&(self.gt.as_ref().unwrap()));
                if agt.aabb.intersect_ray(r) {
                    leaf_triangles.extend(agt.ray_leaf(r));
                }
            }
        } else if self.triangles.is_some() {
            leaf_triangles.extend(self.triangles.as_ref().unwrap());
        }

        if (leaf_triangles.len() > 500) {
            //println!("returning lots! {}", leaf_triangles.len());
        }
        return leaf_triangles;
    }
}
