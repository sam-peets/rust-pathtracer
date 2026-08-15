use core::f32;
use std::{fs::File, io::Write};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    camera::Camera,
    math::{mat4::Mat4, ray::Ray, triangle::Triangle, vec4::Vec4},
    obj::Obj,
    octree::OctreeNode,
    ppm::{Ppm, Rgb8},
};

pub mod camera;
mod math;
mod obj;
mod octree;
mod ppm;

pub fn shade(intersection: Vec4, triangle: Triangle) -> Vec4 {
    todo!()
}

fn main() {
    let obj = Obj::open("models/cornell.obj").unwrap();
    // let scaling = Mat4::scaling(Vec4::from([1000.0, 1000.0, 1000.0, 1.0]));
    let rotation = Mat4::rotation(Vec4::from([0.0, 1.0, 0.0, 0.0]), f32::consts::PI);
    dbg!(obj.triangles.len());

    let centroid = obj.centroid();
    dbg!(centroid);

    let translate = Mat4::translation(centroid * -1.0);
    let obj = obj.apply(translate).apply(rotation);
    let octree = OctreeNode::from_obj(obj);

    eprintln!("built octree");

    let camera = Camera::new(
        Ray::new(
            Vec4::from([0.0, 0.0, -10.0, 1.0]),
            Vec4::from([0.0, 0.0, 1.0, 0.0]),
        ),
        1.0,
    );

    let width = 1000;
    let height = 1000;

    let mut ppm = Ppm::new(width, height);

    let rays = camera.gen_rays(width, height);

    let cols: Vec<Rgb8> = rays
        .par_iter()
        .map(|ray| {
            if let Some((t, tri)) = octree.intersects(*ray) {
                let norm = tri.normal() * 0.5 + Vec4::from([0.5; 4]);
                let r = (norm.x() * 255.0) as u8;
                let g = (norm.y() * 255.0) as u8;
                let b = (norm.z() * 255.0) as u8;

                Rgb8::new(r, g, b)
            } else {
                Rgb8::new(130, 180, 180)
            }
        })
        .collect();

    ppm.buf = cols;

    // println!("{}", ppm.emit())
    let mut outfile = File::create("out.ppm").unwrap();
    outfile.write_all(&ppm.emit().into_bytes()).unwrap();
}
