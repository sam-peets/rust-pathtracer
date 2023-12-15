mod obj;
mod tracer;
mod triangle;
mod vec3;

use crate::{obj::Obj, vec3::Vec3, tracer::Light};

use std::{fs::File, io::BufWriter, io::Write};

fn write_screen(path: &'static str, screen: Vec<Vec3>, res_x: usize, res_y: usize) {
    let file = File::create(path).unwrap();
    let mut buf = BufWriter::new(file);

    writeln!(buf, "P3").unwrap();
    writeln!(buf, "{} {}", res_x, res_y).unwrap();
    writeln!(buf, "255").unwrap();

    for i in 0..(res_x * res_y) {
        let c: Vec3 = screen[i].as_rgb();
        writeln!(buf, "{} {} {}", c.x, c.y, c.z).unwrap();
    }
    let _ = buf.flush();
    println!("done writing: {}", path);
}

fn main() {
    const RES_X: usize = 500;
    const RES_Y: usize = 500;

    let object: Obj = Obj::from_file("teapot.obj");

    let triangles = object.triangles;
    let mut lights: Vec<Light> = Vec::new();

    lights.push(Light{
        pos: Vec3::new(-5.,0.,5.),
        col: Vec3::new(0.2,0.8,0.2),
    });
    
    lights.push(Light{
        pos: Vec3::new(5.,0.,5.),
        col: Vec3::new(0.8,0.2,0.2),
    });

    lights.push(Light{
        pos: Vec3::new(0.,3.,5.),
        col: Vec3::new(0.2,0.2,0.8),
    });

    let mut screen = vec![Vec3::new(0., 0., 0.); RES_X * RES_Y];
    let cam: Vec3 = Vec3::new(0., 1.5, 5.);

    tracer::raytrace(&mut screen, &triangles, &cam, &lights, RES_X, RES_Y);

    write_screen("out.ppm", screen, RES_X, RES_Y);
}
