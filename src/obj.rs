use std::{fs::File, io::Read, path::Path};

use anyhow::anyhow;

use crate::math::{aabb::Aabb, mat4::Mat4, triangle::Triangle, vec4::Vec4};

pub struct Obj {
    pub triangles: Vec<Triangle>,
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
            let mut f = File::open(path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            s
        };

        let mut verts = vec![Vec4::new(0.0, 0.0, 0.0, 0.0)];
        let mut triangles = vec![];

        for line in f.lines() {
            let mut spl = line.split_ascii_whitespace();
            match spl.next() {
                Some("v") => {
                    let x: f32 = spl.next().ok_or(anyhow!("missing component 1"))?.parse()?;
                    let y: f32 = spl.next().ok_or(anyhow!("missing component 2"))?.parse()?;
                    let z: f32 = spl.next().ok_or(anyhow!("missing component 3"))?.parse()?;

                    verts.push(Vec4::new(x, y, z, 1.0));
                }
                Some("f") => {
                    let i1: usize = spl.next().ok_or(anyhow!("missing index 1"))?.parse()?;
                    let i2: usize = spl.next().ok_or(anyhow!("missing index 2"))?.parse()?;
                    let i3: usize = spl.next().ok_or(anyhow!("missing index 3"))?.parse()?;

                    let tri = Triangle::new(verts[i1], verts[i2], verts[i3]);

                    triangles.push(tri);
                }
                Some(_) => {}
                None => {}
            }
        }

        Ok(Self { triangles })
    }

    pub fn apply(self, matrix: Mat4) -> Self {
        let applied_tris = self
            .triangles
            .into_iter()
            .map(|t| t.apply(matrix))
            .collect();

        Self {
            triangles: applied_tris,
        }
    }
}
