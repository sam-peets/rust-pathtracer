use crate::mat4::Mat4;
use crate::obj::Obj;
use crate::triangle::Triangle;
use crate::vec4::Vec4;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, Thread};

use std::cmp;

const EPSILON: f64 = 0.0000001;
const NUM_THREADS: usize = 16;
const MAX_DEPTH: usize = 4;

const AMBIENT_COLOR: Vec4 = Vec4 {
    x: 1.,
    y: 1.,
    z: 1.,
    w: 1.,
};

pub struct Ray {
    pub origin: Vec4,
    pub dir: Vec4,
}

struct Intersection {
    p: Vec4,
    t: f64,
    triangle: Triangle,
}

pub struct Light {
    pub pos: Vec4,
    pub col: Vec4,
}

fn intersects(r: &Ray, t: &Triangle) -> Option<Intersection> {
    // moller-trumbore intersection test
    // adapted from https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html

    let p0p1: Vec4 = t.p1.v - t.p0.v;
    let p0p2: Vec4 = t.p2.v - t.p0.v;
    let pvec: Vec4 = (r.dir).cross(p0p2);
    let det: f64 = p0p1.dot(pvec);

    if det.abs() < EPSILON {
        return None;
    }

    let inv_det: f64 = 1. / det;
    let tvec: Vec4 = r.origin - t.p0.v;
    let u: f64 = tvec.dot(pvec) * inv_det;

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let qvec: Vec4 = tvec.cross(p0p1);
    let v: f64 = (r.dir).dot(qvec) * inv_det;

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let it: f64 = p0p2.dot(qvec) * inv_det;
    if it <= 0. {
        return None;
    }

    return Some(Intersection {
        p: r.origin + r.dir * it,
        t: it,
        triangle: *t,
    });
}

fn brdf(p: &Vec4, t: &Triangle, cam: &Vec4, lights: &Vec<Light>, object: &Obj) -> Vec4 {
    // blinn-phong brdf

    let ns = t.mat.unwrap().ns;
    let ka = t.mat.unwrap().ka;
    let kd = t.mat.unwrap().kd;
    let ks = t.mat.unwrap().ks;
    let ambient: Vec4 = AMBIENT_COLOR * ka;

    //let N: Vec4 = t.normal3p();
    //let mut N: Vec4 = t.normal();

    let mut N: Vec4 = t.normal_interp(&p);
    if N.x.is_nan() {
        N = t.normal(); // fallback to surface normal if interpolation fails
    }

    let V: Vec4 = (*cam - *p).normalize();

    let mut col: Vec4 = ambient;

    for light in lights {
        let mut lN: Vec4 = N;
        let L: Vec4 = (light.pos - *p).normalize();

        let mut lambertian = N.dot(L);

        if lambertian <= 0. {
            lN = N * -1.;
            lambertian = lN.dot(L);
        }

        let r: Ray = Ray {
            origin: ((*p) + lN * 0.0001),
            dir: L,
        };

        let mut fail = false;

        for t in object.head.ray_leaf(&r) {
            // shadow rays
            let result: Option<Intersection> = intersects(&r, &t);

            if result.is_some() {
                let res: Intersection = result.unwrap();
                if (res.p - *p).length() < (light.pos - *p).length() {
                    fail = true;
                    break;
                }
            }
        }
        if fail == true {
            continue;
        }

        let diffuse = light.col * kd * lambertian;

        let H: Vec4 = (L + V).normalize();

        let spec: f64 = lN.dot(H);

        if spec <= 0. {
            col += diffuse;
            continue;
        }

        let specular: Vec4 = light.col * (ks * spec.powf(ns));

        col += diffuse + specular;
    }
    if (col.x.is_nan() || col.y.is_nan() || col.z.is_nan()) {
        println!("NaN value fixme");
        return Vec4::new(0., 0., 0., 0.); // hack to cover weird case idk why this happens
    }
    return col;
}

pub fn raytrace(
    screen: &Arc<Mutex<Vec<Vec4>>>,
    object: &'static Obj,
    cam: &'static Vec4,
    lights: &'static Vec<Light>,
    res_x: usize,
    res_y: usize,
    m: &'static Mat4,
) {
    let mut threads: Vec<JoinHandle<_>> = Vec::new();

    let lines_per_thread = res_y / NUM_THREADS;

    for c in 0..(NUM_THREADS) {
        let screen_mutex_clone = Arc::clone(screen);
        println!("starting thread {c}");

        threads.push(thread::spawn(move || {
            for i in (c * lines_per_thread)..(cmp::min((c + 1) * lines_per_thread, res_y)) {
                if i % 15 == 0 {
                    println!(
                        "thread {}: working on {}/{}",
                        c,
                        i - (c * lines_per_thread),
                        cmp::min((c + 1) * lines_per_thread, res_y) - c * lines_per_thread
                    );
                }
                for j in 0..res_x {
                    let mut res: Vec4 = Vec4::new(0., 0., 0., 0.);
                    let ux = ((j as f64 / res_x as f64) * 2. - 1.) * (res_x as f64 / res_y as f64);
                    let uy = -((i as f64 / res_y as f64) * 2. - 1.);

                    let d = Vec4::new(ux, uy, -2., 0.).normalize();

                    let r: Ray = Ray {
                        origin: (*m) * (*cam),
                        dir: (*m) * d,
                    };

                    let mut intersections: Vec<Intersection> = Vec::new();
                    for t in object.head.ray_leaf(&r) {
                        let result: Option<Intersection> = intersects(&r, &t);
                        if result.is_none() {
                            continue;
                        }

                        intersections.push(result.unwrap());
                    }

                    if intersections.len() == 0 {
                        res = Vec4::new(0.52, 0.80, 0.92, 1.0);
                    } else {
                        let mut min_inter: &Intersection = &intersections[0];

                        for inter in &intersections {
                            if inter.t < min_inter.t {
                                min_inter = &inter;
                            }
                        }
                        res = brdf(&min_inter.p, &min_inter.triangle, &cam, &lights, &object);
                    }

                    {
                        let mut data = screen_mutex_clone.lock().unwrap();
                        data[i * res_x + j] = res;
                    }
                }
            }
            println!("{c} finished");
        }));
    }
    let mut i = 0;
    for t in threads {
        let t_ = t.join();
        i += 1;
    }
}
