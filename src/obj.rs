use crate::triangle::Triangle;
use crate::vec4::{NVec4, Vec4};

use std::fs;

pub struct Obj {
    pub vertices: Vec<NVec4>,
    pub triangles: Vec<Triangle>,
}

impl Obj {
    pub fn from_file(path: &str) -> Obj {
        let contents: String = fs::read_to_string(path).expect("couldn't open file");

        let mut vertices: Vec<NVec4> = Vec::new();
        let mut itriangles: Vec<(usize, usize, usize)> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();

        for line in contents.split("\n") {
            if line == "" {
                continue;
            }
            let mut sl = line.split_whitespace();
            let first: &str = sl.next().unwrap();

            match first {
                "v" => {
                    // vertex
                    let x: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                    let y: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                    let z: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                    let nv: Vec4 = Vec4::new(x, y, z, 1.);
                    let nnv: NVec4 = NVec4 {
                        v: nv,
                        n: Vec4::new(0., 0., 0., 0.),
                    };
                    vertices.push(nnv);

                    //println!("added vertice: {}", nv);
                }
                "f" => {
                    // face
                    let mut i0: usize = sl.next().unwrap().parse::<usize>().unwrap();
                    let mut i1: usize = sl.next().unwrap().parse::<usize>().unwrap();
                    let mut i2: usize = sl.next().unwrap().parse::<usize>().unwrap();

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

        return Obj {
            vertices: vertices,
            triangles: triangles,
        };
    }
}
