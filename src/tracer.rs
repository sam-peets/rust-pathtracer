use crate::mat4::Mat4;
use crate::triangle::Triangle;
use crate::vec4::Vec4;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, Thread};

const EPSILON: f64 = 0.0000001;

const NUM_THREADS: usize = 16;

const AMBIENT_COLOR: Vec4 = Vec4 {
    x: 1.,
    y: 1.,
    z: 1.,
    w: 1.,
};

struct Ray {
    origin: Vec4,
    dir: Vec4,
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

    return Some(Intersection {
        p: r.origin + r.dir * it,
        t: it,
        triangle: *t,
    });
}

fn brdf(
    p: &Vec4,
    t: &Triangle,
    cam: &Vec4,
    lights: &Vec<Light>,
    triangles: &Vec<Triangle>,
) -> Vec4 {
    // phong brdf

    let ka: f64 = 0.01;
    let kd: f64 = 0.8;
    let ks: f64 = 0.6;
    let ns: f64 = 40.;

    let ambient: Vec4 = AMBIENT_COLOR * ka;

    //let N: Vec4 = t.normal3p();
    //let N: Vec4 = t.normal();

    let N: Vec4 = t.normal_interp(&p);

    let V: Vec4 = (*cam - *p).normalize();

    let mut col: Vec4 = ambient;

    for light in lights {
        let L: Vec4 = (light.pos - *p).normalize();

        let r: Ray = Ray {
            origin: ((*p) + N * 0.001),
            dir: L,
        };

        let lambertian: f64 = N.dot(L);
        let mut fail = false;

        for t in triangles {
            let result: Option<Intersection> = intersects(&r, &t);

            if result.is_some() {
                if result.unwrap().t > 0. {
                    fail = true;
                    break;
                }
            }
        }
        if fail == true {
            continue;
        }

        if lambertian < 0. {
            continue;
        }

        let diffuse = light.col * Vec4::new(1., 1., 1., 1.) * (lambertian * kd);

        let H: Vec4 = (L + V).normalize();

        let spec: f64 = N.dot(H);

        if spec < 0. {
            col += diffuse;
            continue;
        }

        let specular: Vec4 = light.col * (spec.powf(ns) * ks);

        col += diffuse + specular;
    }

    return col;
}

pub fn raytrace(
    screen: &Arc<Mutex<Vec<Vec4>>>,
    triangles: &'static Vec<Triangle>,
    cam: &'static Vec4,
    lights: &'static Vec<Light>,
    res_x: usize,
    res_y: usize,
    m: &'static Mat4,
) {
    let mut threads: Vec<JoinHandle<_>> = Vec::new();
    for i in 0..NUM_THREADS {

    }
    for i in 0..res_x {
        let screen_mutex_clone = Arc::clone(screen);

        if i % 10 == 0 {
            println!("starting: {}/{}", i, res_x);
        }

        threads.push(thread::spawn(move || {
            for j in 0..res_y {
                let mut res: Vec4 = Vec4::new(0., 0., 0., 0.);
                let ux = -(((i * res_x + j) % res_y) as f64 / res_x as f64 * 2. - 1.);
                let uy = -(((i * res_x + j) / res_y) as f64 / res_y as f64 * 2. - 1.);

                let d = Vec4::new(ux, uy, -2., 0.).normalize();

                let r: Ray = Ray {
                    origin: (*m) * (*cam),
                    dir: (*m) * d,
                };

                let mut intersections: Vec<Intersection> = Vec::new();

                for t in triangles {
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
                    res = brdf(&min_inter.p, &min_inter.triangle, &cam, &lights, triangles);
                }

                {
                    let mut data = screen_mutex_clone.lock().unwrap();
                    data[i * res_x + j] = res;
                }
            }

            println!("done werkin {i}");
        }))
    }
    let mut i = 0;
    for t in threads {
        println!("{}: joining", i);
        let t_ = t.join();
        i += 1;
    }
    /*
    for i in 0..(res_x * res_y) {
        let ux = -((i % res_y) as f64 / res_x as f64 * 2. - 1.);
        let uy = -((i / res_y) as f64 / res_y as f64 * 2. - 1.);

        if i % 10000 == 0 {
            println!("line: {}/{}", i, res_x * res_y);
        }

        let d = Vec4::new(ux, uy, -2., 0.).normalize();

        let r: Ray = Ray {
            origin: (*m) * (*cam),
            dir: (*m) * d,
        };

        let mut col: Vec4 = Vec4::new(0., 0., 0., 1.);

        let mut intersections: Vec<Intersection> = Vec::new();

        {
            for t in triangles {
                let result: Option<Intersection> = intersects(&r, &t);
                if result.is_none() {
                    continue;
                }

                intersections.push(result.unwrap());
            }

            if intersections.len() == 0 {
                screen[i] = Vec4::new(0.52, 0.80, 0.92, 1.0);
                continue;
            }

            let mut min_inter: &Intersection = &intersections[0];

            for inter in &intersections {
                if inter.t < min_inter.t {
                    min_inter = &inter;
                }
            }

            screen[i] = brdf(&min_inter.p, &min_inter.triangle, &cam, &lights, triangles);
        }
    }*/
}
