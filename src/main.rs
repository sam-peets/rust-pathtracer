mod aabb;
mod kdtree;
mod mat4;
mod obj;
mod tracer;
mod triangle;
mod vec4;

use crate::{aabb::AABB, mat4::Mat4, obj::Obj, tracer::Light, vec4::Vec4};

use std::{fs::File, io::BufWriter, io::Write};

use std::sync::{Arc, Mutex};
use std::thread;

use std::ops::Deref;

fn write_screen(path: &'static str, screen: &Vec<Vec4>, res_x: usize, res_y: usize) {
    let file = File::create(path).unwrap();
    let mut buf = BufWriter::new(file);

    writeln!(buf, "P3").unwrap();
    writeln!(buf, "{} {}", res_x, res_y).unwrap();
    writeln!(buf, "255").unwrap();

    for i in 0..(res_x * res_y) {
        let c: Vec4 = screen[i].as_rgb();
        writeln!(buf, "{} {} {}", c.x, c.y, c.z).unwrap();
    }
    let _ = buf.flush();
    println!("done writing: {}", path);
}

fn main() {
    const RES_X: usize = 1000;
    const RES_Y: usize = 1000;

    let rx: f64 = -3.1415/2.;
    let ry: f64 = 3.1415/4.;
    let rz: f64 = -(3.1415 / 8.);

    let mut maz = [Vec4::new(0., 0., 0., 0.); 4];
    maz[0] = Vec4::new(rz.cos(), -(rz.sin()), 0., 0.);
    maz[1] = Vec4::new(rz.sin(), rz.cos(), 0., 0.);
    maz[2] = Vec4::new(0., 0., 1., 0.);
    maz[3] = Vec4::new(0., 0., 0., 1.);
    println!("{}", maz[0]);

    let mut max = [Vec4::new(0., 0., 0., 0.); 4];
    max[0] = Vec4::new(1., 0., 0., 0.);
    max[1] = Vec4::new(0., rx.cos(), -(rx.sin()), 0.);
    max[2] = Vec4::new(0., rx.sin(), rx.cos(), 0.);
    max[3] = Vec4::new(0., 0., 0., 1.);

    let mut may = [Vec4::new(0., 0., 0., 0.); 4];
    may[0] = Vec4::new(ry.cos(), 0., -(ry.sin()), 0.);
    may[1] = Vec4::new(0., 1., 0., 0.);
    may[2] = Vec4::new(ry.sin(), 0., ry.cos(), 0.);
    may[3] = Vec4::new(0., 0., 0., 1.);

    let mut mas = [Vec4::new(0., 0., 0., 0.); 4];
    mas[0] = Vec4::new(20., 0., 0., 0.);
    mas[1] = Vec4::new(0., 20., 0., 0.);
    mas[2] = Vec4::new(0., 0., 20., 0.);
    mas[3] = Vec4::new(0., 0., 0., 1.);

    let mz: Mat4 = Mat4 { m: maz };
    let mx: Mat4 = Mat4 { m: max };
    let ms: Mat4 = Mat4 { m: mas };
    let my: Mat4 = Mat4 { m: may };

    let object: Obj = Obj::from_file("obj/cornell.obj", &(Mat4::identity()));
    /*println!(
        "{} vertices, {} triangles",
        object.vertices.len(),
        object.head.triangles.as_ref().unwrap().len()
    );*/

    //let triangles = object.triangles;
    let mut lights: Vec<Light> = Vec::new();

    /*lights.push(Light{
        pos: Vec4::new(-30.,30.,10.,1.),
        col: Vec4::new(0.2,0.8,0.2,1.),
    });

    lights.push(Light {
        pos: Vec4::new(-3000., 1500., 2500., 1.),
        col: Vec4::new(0.5, 0.1, 0.1, 1.),
    });

    lights.push(Light {
        pos: Vec4::new(3000., 1500., 2500., 1.),
        col: Vec4::new(0.1, 0.1, 0.5, 1.),
    });*/

    lights.push(Light {
        pos: Vec4::new(0., 4., 5., 1.),
        col: Vec4::new(1.,1.,1.,1.),
    });

    let mut screen = vec![Vec4::new(0., 0., 0., 1.); RES_X * RES_Y];
    let cam: Vec4 = Vec4::new(0., 0., 20., 1.);
    let screen_mutex = Arc::new(Mutex::new(screen));

    println!("min: {}, max: {}", object.aabb.min, object.aabb.max);

    tracer::raytrace(
        &screen_mutex,
        Box::leak(Box::new(object)), // has to be a better way to make these 'static
        Box::leak(Box::new(cam)),
        Box::leak(Box::new(lights)),
        RES_X,
        RES_Y,
        Box::leak(Box::new(Mat4::identity())),
    );

    write_screen(
        "out.ppm",
        Arc::clone(&screen_mutex).lock().unwrap().deref(),
        RES_X,
        RES_Y,
    );
}
