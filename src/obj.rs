use std::{collections::HashMap, fs::File, hash::Hash, io::Read, path::Path};

use anyhow::anyhow;

use crate::math::{aabb::Aabb, mat4::Mat4, triangle::Triangle, vec4::Vec4};

fn read_vec4<'a>(spl: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Vec4> {
    let x: f32 = spl.next().ok_or(anyhow!("missing component 1"))?.parse()?;
    let y: f32 = spl.next().ok_or(anyhow!("missing component 2"))?.parse()?;
    let z: f32 = spl.next().ok_or(anyhow!("missing component 3"))?.parse()?;

    Ok(Vec4::new(x, y, z, 1.0))
}

// placeholder type -- this will probably never be used
// this should theoretically be an enum
#[derive(Debug, Clone)]
pub struct IlluminationModel(usize);

#[derive(Debug, Clone)]
pub struct Material {
    pub kd: Vec4,
    pub ks: Vec4,
    pub ke: Vec4,
    pub ni: f32,
    pub d: f32,
    pub illum: IlluminationModel,
    pub pr: f32,
    pub pm: f32,
    pub ps: f32,
    pub pc: f32,
    pub pcr: f32,
    pub aniso: f32,
    pub anisor: f32,
}

impl Default for Material {
    fn default() -> Self {
        // matches the default blender material
        Self {
            kd: Vec4::new(0.8, 0.8, 0.8, 1.0),
            ks: Vec4::new(0.5, 0.5, 0.5, 1.0),
            ke: Vec4::new(0.0, 0.0, 0.0, 1.0),
            ni: 1.5,
            d: 1.0,
            illum: IlluminationModel(2),
            pr: 0.5,
            pm: 0.0,
            ps: 0.0,
            pc: 0.0,
            pcr: 0.03,
            aniso: 0.0,
            anisor: 0.0,
        }
    }
}

struct Mtl {
    materials: HashMap<String, Material>,
}

impl Mtl {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let f = {
            let mut f = File::open(path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            s
        };

        let mut materials = HashMap::new();

        let mut current_material = None;

        for line in f.lines() {
            let mut spl = line.split_ascii_whitespace();
            match spl.next() {
                Some("newmtl") => {
                    if let Some(name) = spl.next() {
                        current_material = Some(name.to_string());
                        materials.insert(name.to_string(), Material::default());
                    } else {
                        return Err(anyhow!("missing material name"));
                    }
                }
                Some("Kd") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.kd = read_vec4(&mut spl)?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("Ks") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.ks = read_vec4(&mut spl)?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("Ke") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.ke = read_vec4(&mut spl)?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("Ni") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.ni = spl.next().ok_or(anyhow!("missing Ni"))?.parse()?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("d") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.d = spl.next().ok_or(anyhow!("missing d"))?.parse()?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("illum") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.illum =
                            IlluminationModel(spl.next().ok_or(anyhow!("missing illum"))?.parse()?);
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("Pr") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.pr = spl.next().ok_or(anyhow!("missing Pr"))?.parse()?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("Pm") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.pm = spl.next().ok_or(anyhow!("missing Pm"))?.parse()?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("Pc") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.pc = spl.next().ok_or(anyhow!("missing Pc"))?.parse()?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("Pcr") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.pcr = spl.next().ok_or(anyhow!("missing Pcr"))?.parse()?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("aniso") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.aniso = spl.next().ok_or(anyhow!("missing aniso"))?.parse()?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some("anisor") => {
                    if let Some(name) = &current_material
                        && let Some(mat) = materials.get_mut(name)
                    {
                        mat.anisor = spl.next().ok_or(anyhow!("missing anisor"))?.parse()?;
                    } else {
                        return Err(anyhow!("bad mtl"));
                    }
                }
                Some(op) => {
                    log::warn!("ignoring mtl op: {}", op);
                }
                None => {}
            }
        }

        Ok(Self { materials })
    }
}

pub struct Obj {
    pub triangles: Vec<Triangle>,
    pub materials: Vec<Material>,
}

impl Obj {
    pub fn centroid(&self) -> Vec4 {
        self.aabb().centroid()
    }

    pub fn aabb(&self) -> Aabb {
        self.triangles
            .iter()
            .map(|tri| tri.aabb())
            .reduce(Aabb::union)
            .unwrap_or(Aabb::new(
                Vec4::new(0.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 0.0),
            ))
    }

    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let f = {
            let mut f = File::open(&path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            s
        };

        let mut mtl = None;

        let mut mtl_ids: HashMap<String, usize> = HashMap::new();
        let mut materials: Vec<Material> = vec![];

        let mut cur_material = None;

        let mut verts = vec![Vec4::new(0.0, 0.0, 0.0, 0.0)];
        let mut triangles = vec![];
        let mut normals = vec![None];

        for line in f.lines() {
            let mut spl = line.split_ascii_whitespace();
            match spl.next() {
                Some("v") => {
                    let v = read_vec4(&mut spl)?;
                    verts.push(v);
                    normals.push(None);
                }
                Some("f") => {
                    let i1: usize = spl.next().ok_or(anyhow!("missing index 1"))?.parse()?;
                    let i2: usize = spl.next().ok_or(anyhow!("missing index 2"))?.parse()?;
                    let i3: usize = spl.next().ok_or(anyhow!("missing index 3"))?.parse()?;

                    let e1 = verts[i2] - verts[i1];
                    let e2 = verts[i3] - verts[i1];
                    let tn = e1.cross(e2).normalize();

                    triangles.push(((i1, i2, i3), cur_material));

                    for i in [i1, i2, i3] {
                        if let Some(n) = normals[i] {
                            normals[i] = Some(n + tn);
                        } else {
                            normals[i] = Some(tn);
                        }
                    }
                }
                Some("mtllib") => {
                    if let Some(mtl_path) = spl.next() {
                        let path = Path::new(path.as_ref()).parent().unwrap().join(mtl_path);
                        mtl = Some(Mtl::open(path)?);
                    } else {
                        return Err(anyhow!("missing mtl path"));
                    }
                }
                Some("usemtl") => {
                    if let Some(name) = spl.next()
                        && let Some(mtl) = &mtl
                        && let Some(mat) = mtl.materials.get(name)
                    {
                        log::info!("using material: {}", name);
                        log::info!("  kd: {:?}", mat.kd);
                        log::info!("  ks: {:?}", mat.ks);
                        log::info!("  ke: {:?}", mat.ke);
                        if let Some(id) = mtl_ids.get(name) {
                            cur_material = Some(*id);
                        } else {
                            let id = materials.len();
                            materials.push(mat.clone());
                            mtl_ids.insert(name.to_string(), id);
                            cur_material = Some(id);
                        }
                        println!("materials: {:?}", materials);
                    } else {
                        return Err(anyhow!("missing material name or mtl"));
                    }
                }
                Some(_) => {}
                None => {}
            }
        }

        let triangles = triangles
            .into_iter()
            .map(|((i1, i2, i3), cur_material)| {
                // don't accumulate normals, just use the face normal for now
                // let n1 = normals[i1].unwrap().normalize();
                // let n2 = normals[i2].unwrap().normalize();
                // let n3 = normals[i3].unwrap().normalize();

                let mut t = Triangle::new(verts[i1], verts[i2], verts[i3]);
                t.mtl_id = cur_material;
                t
            })
            .collect();

        Ok(Self {
            triangles,
            materials,
        })
    }

    pub fn apply(self, matrix: Mat4) -> Self {
        let applied_tris = self
            .triangles
            .into_iter()
            .map(|t| t.apply(matrix))
            .collect();

        Self {
            triangles: applied_tris,
            materials: self.materials,
        }
    }
}
