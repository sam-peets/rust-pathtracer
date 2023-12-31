use crate::aabb::AABB;
use crate::kdtree::KDNode;
use crate::mat4::Mat4;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec4::{NVec4, Vec4};
use std::sync::{Arc, Mutex};

use std::cmp;
use std::collections::HashMap;
use std::fs;

pub struct Obj {
    pub vertices: Vec<NVec4>,
    pub head: KDNode,
    pub aabb: AABB,
}

impl Obj {
    pub fn from_file(objpath: &str, m: &Mat4) -> Obj {
        let (vertices, triangles, aabb) = read_obj(objpath, m);

        println!(
            "read: {} verts, {} triangles",
            vertices.len(),
            triangles.len()
        );

        let head = KDNode::new(&triangles, 0, &aabb);

        return Obj {
            vertices: vertices,
            head: head,
            aabb: aabb,
        };
    }
}

pub fn read_obj(objpath: &str, m: &Mat4) -> (Vec<NVec4>, Vec<Triangle>, AABB) {
    let mut materials: HashMap<String, Material> = HashMap::new();
    let mut cur_material: Option<Material> = Some(Material::default());
    let obj_contents: String = fs::read_to_string(objpath).expect("couldn't open obj");
    let mut vertices: Vec<NVec4> = Vec::new();
    let mut itriangles: Vec<(usize, usize, usize, Option<Material>)> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    let mut minV = Vec4::new(0., 0., 0., 0.);
    let mut maxV = Vec4::new(0., 0., 0., 0.);

    for line in obj_contents.split("\n") {
        if line == "" {
            continue;
        }
        let mut sl = line.split_whitespace();

        let fsl = sl.next();
        if fsl.is_none() {
            continue;
        }

        let first = fsl.unwrap();
        if first.chars().next().unwrap() == '#' {
            continue;
        }

        match first {
            "v" => {
                // vertex
                let x: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                let y: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                let z: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                let mut nv: Vec4 = Vec4::new(x, y, z, 1.);
                nv = (*m) * nv;
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
                    mat: None,
                };

                let n: Vec4 = nt.normal();

                vertices[i0].n += n;
                vertices[i1].n += n;
                vertices[i2].n += n;

                itriangles.push((i0, i1, i2, cur_material));
                //println!("added triangle: {} {} {}", p0, p1, p2);
            }
            "o" => {
                // mayb extend this in the future, this *should* be fine for now
                cur_material = Some(Material::default());
            }
            "mtllib" => {
                materials.extend(read_mtl(sl.next().unwrap()));
            }
            "usemtl" => match materials.get(sl.next().unwrap()) {
                Some(x) => {
                    cur_material = Some(*x);
                }
                None => {
                    panic!("couldnt find material");
                }
            },
            // TODO: vertex normals, materials, textures...
            _ => {
                println!("read unknown obj token: {}", first);
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
            mat: (*i).3,
        };
        triangles.push(t);
    }
    let aabb = AABB {
        min: minV,
        max: maxV,
    };

    return (vertices, triangles, aabb);
}

pub fn read_mtl(mtlpath: &str) -> HashMap<String, Material> {
    let mut material_stack: Vec<(String, Material)> = Vec::new();
    let mtl_contents: String = fs::read_to_string(mtlpath).expect("couldn't open mtl");

    for line in mtl_contents.split("\n") {
        if line == "" {
            continue;
        }
        let mut sl = line.split_whitespace();
        let fsl = sl.next();
        if fsl.is_none() {
            continue;
        }
        let first = fsl.unwrap();
        if first.chars().next().unwrap() == '#' {
            continue;
        }

        match first {
            "newmtl" => {
                material_stack.push((
                    sl.next().unwrap().to_string(),
                    (Material {
                        ns: 0.,
                        ka: Vec4::new(0., 0., 0., 0.),
                        kd: Vec4::new(0., 0., 0., 0.),
                        ks: Vec4::new(0., 0., 0., 0.),
                        ke: Vec4::new(0., 0., 0., 0.),
                        ni: 0.,
                        d: 0.,
                        illum: 2,
                    }),
                ));
                println!("created material {}", material_stack.last().unwrap().0);
            }
            "Ns" => {
                let ns: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                material_stack.last_mut().unwrap().1.ns = ns;
            }
            "Ka" => {
                let r = sl.next().unwrap().parse::<f64>().unwrap();
                let g = sl.next().unwrap().parse::<f64>().unwrap();
                let b = sl.next().unwrap().parse::<f64>().unwrap();
                material_stack.last_mut().unwrap().1.ka.x = r;
                material_stack.last_mut().unwrap().1.ka.y = g;
                material_stack.last_mut().unwrap().1.ka.z = b;
            }
            "Kd" => {
                let r = sl.next().unwrap().parse::<f64>().unwrap();
                let g = sl.next().unwrap().parse::<f64>().unwrap();
                let b = sl.next().unwrap().parse::<f64>().unwrap();
                material_stack.last_mut().unwrap().1.kd.x = r;
                material_stack.last_mut().unwrap().1.kd.y = g;
                material_stack.last_mut().unwrap().1.kd.z = b;
            }
            "Ks" => {
                let r = sl.next().unwrap().parse::<f64>().unwrap();
                let g = sl.next().unwrap().parse::<f64>().unwrap();
                let b = sl.next().unwrap().parse::<f64>().unwrap();
                material_stack.last_mut().unwrap().1.ks.x = r;
                material_stack.last_mut().unwrap().1.ks.y = g;
                material_stack.last_mut().unwrap().1.ks.z = b;
            }
            "Ke" => {
                let r = sl.next().unwrap().parse::<f64>().unwrap();
                let g = sl.next().unwrap().parse::<f64>().unwrap();
                let b = sl.next().unwrap().parse::<f64>().unwrap();
                material_stack.last_mut().unwrap().1.ke.x = r;
                material_stack.last_mut().unwrap().1.ke.y = g;
                material_stack.last_mut().unwrap().1.ke.z = b;
            }

            "Ni" => {
                let ni: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                material_stack.last_mut().unwrap().1.ni = ni;
            }
            "d" => {
                let d: f64 = sl.next().unwrap().parse::<f64>().unwrap();
                material_stack.last_mut().unwrap().1.d = d;
            }
            "illum" => {
                let illum: usize = sl.next().unwrap().parse::<usize>().unwrap();
                material_stack.last_mut().unwrap().1.illum = illum;
            }
            _ => {
                println!("read unknown mtl token: {}", first);
            }
        }
    }

    let mut materials: HashMap<String, Material> = HashMap::new();
    for (s, mat) in material_stack {
        println!("{}: {} {} {}", s, mat.ka, mat.kd, mat.ks);
        materials.insert(s.clone(), mat);
    }

    return materials;
}
