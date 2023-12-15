mod triangle;
mod vec3;

use crate::triangle::Triangle;
use crate::vec3::Vec3;

use std::{fs::File, io::BufWriter, io::Write};

const EPSILON: f64 = 0.00001;

struct Ray {
    origin: Vec3,
    dir: Vec3,
}

struct Intersection {
    p: Vec3,
}

fn intersects(r: &Ray, t: &Triangle) -> Option<Intersection> {
    // moller-trumbore intersection test
    // adapted from https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html

    let p0p1: Vec3 = t.p1 - t.p0;
    let p0p2: Vec3 = t.p2 - t.p0;
    let pvec: Vec3 = (r.dir).cross(p0p2);
    let det: f64 = p0p1.dot(pvec);

    if det.abs() < EPSILON {
        return None;
    }

    let inv_det: f64 = 1. / det;
    let tvec: Vec3 = r.origin - t.p0;
    let u: f64 = tvec.dot(pvec) * inv_det;

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let qvec: Vec3 = tvec.cross(p0p1);
    let v: f64 = (r.dir).dot(qvec) * inv_det;

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t: f64 = p0p2.dot(qvec) * inv_det;

    return Some(Intersection {
        p: r.origin + r.dir * t,
    });
}

fn main() {
    const RES_X: usize = 1000;
    const RES_Y: usize = 1000;

    let mut triangles = Vec::new();
    let mut screen = vec![Vec3::new(0., 0., 0.); RES_X * RES_Y];

    let p00: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: -5.,
    };
    let p01: Vec3 = Vec3 {
        x: 1.,
        y: 0.,
        z: -5.,
    };
    let p02: Vec3 = Vec3 {
        x: 0.,
        y: 1.,
        z: -5.,
    };

    let p10: Vec3 = Vec3::new(-1., -1., -5.);
    let p11: Vec3 = Vec3::new(-2., -1., -5.);
    let p12: Vec3 = Vec3::new(-1., -2., -5.);

    let cam: Vec3 = Vec3::new(0., 0., 0.);

    triangles.push(Triangle {
        p0: p00,
        p1: p01,
        p2: p02,
    });
    triangles.push(Triangle {
        p0: p10,
        p1: p11,
        p2: p12,
    });

    for i in 0..(RES_X * RES_Y) {
        let ux = -((i % RES_Y) as f64 / RES_X as f64 * 2. - 1.);
        let uy = -((i / RES_Y) as f64 / RES_Y as f64 * 2. - 1.);

        let d = Vec3::new(ux, uy, -1.).normalize();

        let r: Ray = Ray {
            origin: cam,
            dir: d,
        };
        let mut col: Vec3 = Vec3::new(0., 0., 0.);

        for t in &triangles {
            let result: Option<Intersection> = intersects(&r, &t);
            if result.is_none() {
                continue;
            }

            col = Vec3::new(1., 1., 1.);
        }
        screen[i] = col;
    }

    let mut file = File::create("out.ppm").unwrap();
    let mut buf = BufWriter::new(file);

    writeln!(buf, "P3").unwrap();
    writeln!(buf, "{} {}", RES_X, RES_Y).unwrap();
    writeln!(buf, "255").unwrap();

    for i in 0..(RES_X * RES_Y) {
        let c: Vec3 = screen[i].as_rgb();
        writeln!(buf, "{} {} {}", c.x, c.y, c.z).unwrap();
    }
}
