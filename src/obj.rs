use crate::aabb::AABB;
use crate::kdtree::KDNode;
use crate::mat4::Mat4;
use crate::triangle::Triangle;
use crate::vec4::{NVec4, Vec4};

use std::cmp;
use std::fs;
use std::sync::{Arc, Mutex};

pub struct Obj {
    pub vertices: Vec<NVec4>,
    //pub triangles: Vec<Triangle>,
    pub head: KDNode,
    pub aabb: AABB,
}

impl Obj {
    pub fn from_file(path: &str, m: &Mat4) -> Obj {
        let contents: String = fs::read_to_string(path).expect("couldn't open file");

        let mut vertices: Vec<NVec4> = Vec::new();
        let mut itriangles: Vec<(usize, usize, usize)> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();

        let mut minV = Vec4::new(0., 0., 0., 0.);
        let mut maxV = Vec4::new(0., 0., 0., 0.);

        for line in contents.split("\n") {
            if line == "" {
                continue;
            }
            let mut sl = line.split_whitespace();

            let fsl = sl.next();
            if fsl.is_none() {
                continue;
            }

            let first = fsl.unwrap();

            match first {
                "v" => {
                    // vertex
                    let x: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                    let y: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                    let z: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                    let mut nv: Vec4 = Vec4::new(x, y, z, 1.);
                    println!("{nv}");
                    nv = (*m) * nv;

                    println!("{nv}");
                    let nnv: NVec4 = NVec4 {
                        v: nv,
                        n: Vec4::new(0., 0., 0., 0.),
                    };

                    vertices.push(nnv);

                    minV.x = f64::min(minV.x, nv.x);
                    minV.y = f64::min(minV.y, nv.y);
                    minV.z = f64::min(minV.z, nv.z);

                    maxV.x = f64::max(maxV.x, nv.x);
                    maxV.y = f64::max(maxV.y, nv.y);
                    maxV.z = f64::max(maxV.z, nv.z);
                }
                "f" => {
                    // face
                    let mut v: Vec<usize> = Vec::new();
                    for s in sl {
                        let mut vspl = s.split("/");
                        v.push(vspl.next().unwrap().parse::<usize>().unwrap());
                        // TODO handle textures and maybe normals
                    }

                    //let mut i0: usize = sl.next().unwrap().parse::<usize>().unwrap();
                    //let mut i1: usize = sl.next().unwrap().parse::<usize>().unwrap();
                    //let mut i2: usize = sl.next().unwrap().parse::<usize>().unwrap();
                    let mut i0: usize = v[0];
                    let mut i1: usize = v[1];
                    let mut i2: usize = v[2];

                    // obj use 1-based indexing, have to modify
                    // TODO: support negative indices
                    i0 -= 1;
                    i1 -= 1;
                    i2 -= 1;

                    let p0: NVec4 = vertices[i0];
                    let p1: NVec4 = vertices[i1];
                    let p2: NVec4 = vertices[i2];

                    let nt: Triangle = Triangle {
                        p0: p0,
                        p1: p1,
                        p2: p2,
                    };

                    let n: Vec4 = nt.normal();

                    vertices[i0].n += n;
                    vertices[i1].n += n;
                    vertices[i2].n += n;

                    itriangles.push((i0, i1, i2));
                    //println!("added triangle: {} {} {}", p0, p1, p2);
                }
                // TODO: vertex normals, materials, textures...
                _ => {
                    println!("read unknown start token {}", first);
                }
            }
        }

        for v in &mut vertices {
            v.n = v.n.normalize();
        }
        for i in &itriangles {
            let t = Triangle {
                p0: vertices[(*i).0],
                p1: vertices[(*i).1],
                p2: vertices[(*i).2],
            };
            triangles.push(t);
        }

        let aabb = AABB {
            min: minV,
            max: maxV,
        };
        println!("read: {} verts, {} triangles", vertices.len(), triangles.len());

        return Obj {
            vertices: vertices,
            //triangles: triangles,
            head: KDNode::new(&triangles, 0, &aabb),
            aabb: aabb,
        };
    }
}
