use std::{fs::File, io::Write};

use crate::{
    camera::Camera,
    math::{
        RayIntersects,
        mat4::Mat4,
        ray::{self, Ray},
        vec4::Vec4,
    },
    obj::Obj,
    octree::OctreeNode,
    ppm::{Ppm, Rgb8},
};

pub mod camera;
mod math;
mod obj;
mod octree;
mod ppm;

fn main() {
    let obj = Obj::open("xyzrgb_dragon.obj").unwrap();

    let centroid = obj.centroid();
    dbg!(centroid);

    let translate = Mat4::translation(centroid * -1.0);
    let obj = obj.apply(translate);
    let octree = OctreeNode::from_obj(obj);
    eprintln!("built octree");

    let camera = Camera::new(
        Ray::new(
            Vec4::from([0.0, 0.0, -150.0, 1.0]),
            Vec4::from([0.0, 0.0, 1.0, 0.0]),
        ),
        1.0,
    );

    let width = 3000;
    let height = 3000;

    let mut ppm = Ppm::new(width, height);

    let rays = camera.gen_rays(width, height);

    for (i, ray) in rays.into_iter().enumerate() {
        let x = width - i % width;
        let y = height - i / width;

        let mut min_intersection = None;
        let candidates = octree.find_candidates(ray);
        for tri in candidates {
            if let Some(t) = tri.intersects(ray) {
                if let Some((min_t, _)) = min_intersection {
                    if t < min_t {
                        min_intersection = Some((t, tri.clone()))
                    }
                } else {
                    min_intersection = Some((t, tri.clone()))
                }
            }
        }

        if let Some((t, tri)) = min_intersection {
            let norm = tri.normal() * 0.5 + Vec4::from([0.5; 4]);
            let r = (norm.x() * 255.0) as u8;
            let g = (norm.y() * 255.0) as u8;
            let b = (norm.z() * 255.0) as u8;

            ppm.write(x, y, Rgb8::new(r, g, b));
        }
    }

    // println!("{}", ppm.emit())
    let mut outfile = File::create("out.ppm").unwrap();
    outfile.write_all(&ppm.emit().into_bytes()).unwrap();
}
