use crate::triangle::Triangle;
use crate::vec3::Vec3;

use std::fs;

pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
}

impl Obj {
    pub fn from_file(path: &str) -> Obj {
        let contents: String = fs::read_to_string(path).expect("couldn't open file");

        let mut vertices: Vec<Vec3> = Vec::new();
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
                    let nv: Vec3 = Vec3::new(x, y, z);
                    vertices.push(nv);

                    //println!("added vertice: {}", nv);
                }
                "f" => {
                    // face
                    let mut i0: usize = sl.next().unwrap().parse::<usize>().unwrap();
                    let mut i1: usize = sl.next().unwrap().parse::<usize>().unwrap();
                    let mut i2: usize = sl.next().unwrap().parse::<usize>().unwrap();

                    // obj use weird numbering, have to modify
                    // TODO: support negative indices
                    i0 -= 1;
                    i1 -= 1;
                    i2 -= 1;

                    let p0: Vec3 = vertices[i0];
                    let p1: Vec3 = vertices[i1];
                    let p2: Vec3 = vertices[i2];

                    let nt: Triangle = Triangle {
                        p0: p0,
                        p1: p1,
                        p2: p2,
                    };

                    triangles.push(nt);

                    //println!("added triangle: {} {} {}", p0, p1, p2);
                }
                // TODO: vertex normals, materials, textures...
                _ => {
                    println!("read unknown start token {}", first);
                }
            }
        }

        return Obj {
            vertices: vertices,
            triangles: triangles,
        };
    }
}
