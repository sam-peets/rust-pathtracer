use crate::{
    math::{RayIntersects, aabb::Aabb, ray::Ray, triangle::Triangle, vec4::Vec4},
    obj::{Material, Obj},
};

const MAX_DEPTH: usize = 16;
const MAX_TRIANGLES: usize = 32;

pub struct Octree {
    root: OctreeNode,
    pub materials: Vec<Material>,
}

impl Octree {
    pub fn from_obj(obj: Obj) -> Self {
        let root = OctreeNode::build(obj.triangles, 0);
        let materials = obj.materials;
        Self { root, materials }
    }

    pub fn intersects(&self, ray: Ray) -> Option<(f32, &Triangle)> {
        self.root.intersects(ray)
    }
}

pub struct OctreeNode {
    pub aabb: Aabb,
    data: OctreeData,
}

enum OctreeData {
    Internal {
        nnn: Box<OctreeNode>,
        nnp: Box<OctreeNode>,
        npn: Box<OctreeNode>,
        npp: Box<OctreeNode>,
        pnn: Box<OctreeNode>,
        pnp: Box<OctreeNode>,
        ppn: Box<OctreeNode>,
        ppp: Box<OctreeNode>,
    },
    Leaf {
        triangles: Vec<Triangle>,
    },
    Empty,
}

impl OctreeNode {
    pub fn intersects(&self, ray: Ray) -> Option<(f32, &Triangle)> {
        if self.aabb.intersects(ray).is_none() {
            return None;
        }

        match &self.data {
            OctreeData::Internal {
                nnn,
                nnp,
                npn,
                npp,
                pnn,
                pnp,
                ppn,
                ppp,
            } => [nnn, nnp, npn, npp, pnn, pnp, ppn, ppp]
                .iter()
                .flat_map(|node| node.intersects(ray))
                .min_by(|(t1, _), (t2, _)| t1.total_cmp(t2)),
            OctreeData::Leaf { triangles } => triangles
                .iter()
                .flat_map(|tri| tri.intersects(ray).map(|s| (s, tri)))
                .min_by(|(t1, _), (t2, _)| t1.total_cmp(t2)),
            OctreeData::Empty => None,
        }
    }

    pub fn from_obj(obj: Obj) -> Self {
        let triangles = obj.triangles;
        OctreeNode::build(triangles, 0)
    }

    pub fn build(triangles: Vec<Triangle>, depth: usize) -> OctreeNode {
        let aabb = triangles
            .iter()
            .map(|tri| tri.aabb())
            .reduce(Aabb::union)
            .unwrap_or(Aabb::new(
                Vec4::new(0.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 0.0),
            ));

        if depth >= MAX_DEPTH || triangles.len() <= MAX_TRIANGLES {
            return OctreeNode {
                aabb,
                data: OctreeData::Leaf { triangles },
            };
        }

        let center = aabb.centroid();

        let mut nnn = vec![];
        let mut nnp = vec![];
        let mut npn = vec![];
        let mut npp = vec![];
        let mut pnn = vec![];
        let mut pnp = vec![];
        let mut ppn = vec![];
        let mut ppp = vec![];

        for tri in triangles {
            let c = tri.centroid();
            if c.x() < center.x() {
                if c.y() < center.y() {
                    if c.z() < center.z() {
                        nnn.push(tri.clone());
                    } else {
                        nnp.push(tri.clone());
                    }
                } else {
                    if c.z() < center.z() {
                        npn.push(tri.clone());
                    } else {
                        npp.push(tri.clone());
                    }
                }
            } else {
                if c.y() < center.y() {
                    if c.z() < center.z() {
                        pnn.push(tri.clone());
                    } else {
                        pnp.push(tri.clone());
                    }
                } else {
                    if c.z() < center.z() {
                        ppn.push(tri.clone());
                    } else {
                        ppp.push(tri.clone());
                    }
                }
            }
        }

        let nnn = Box::new(OctreeNode::build(nnn, depth + 1));
        let nnp = Box::new(OctreeNode::build(nnp, depth + 1));
        let npn = Box::new(OctreeNode::build(npn, depth + 1));
        let npp = Box::new(OctreeNode::build(npp, depth + 1));
        let pnn = Box::new(OctreeNode::build(pnn, depth + 1));
        let pnp = Box::new(OctreeNode::build(pnp, depth + 1));
        let ppn = Box::new(OctreeNode::build(ppn, depth + 1));
        let ppp = Box::new(OctreeNode::build(ppp, depth + 1));

        OctreeNode {
            aabb,
            data: OctreeData::Internal {
                nnn,
                nnp,
                npn,
                npp,
                pnn,
                pnp,
                ppn,
                ppp,
            },
        }
    }
}
