mod obj;
mod tracer;
mod triangle;
mod vec4;
mod mat4;

use crate::{obj::Obj, tracer::Light, vec4::Vec4, mat4::Mat4};

use std::{fs::File, io::BufWriter, io::Write};

fn write_screen(path: &'static str, screen: Vec<Vec4>, res_x: usize, res_y: usize) {
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

    let r: f64 = -3.1415/8.;

    let mut maz = [Vec4::new(0.,0.,0.,0.); 4];
    maz[0] = Vec4::new(r.cos(), -(r.sin()), 0., 0.,);
    maz[1] = Vec4::new(r.sin(), r.cos(), 0., 0.,);
    maz[2] = Vec4::new(0.,0.,1.,0.);
    maz[3] = Vec4::new(0.,0.,0.,1.);
    println!("{}", maz[0]);
    
    let mut max = [Vec4::new(0.,0.,0.,0.); 4];
    max[0] = Vec4::new(1.,0.,0.,0.);
    max[1] = Vec4::new(0.,r.cos(),-r.sin(),0.);
    max[2] = Vec4::new(0.,r.sin(),r.cos(),0.);
    max[3] = Vec4::new(0.,0.,0.,1.);

    let ry: f64 = 3.1415/8.;
    let mut may = [Vec4::new(0.,0.,0.,0.); 4];
    may[0] = Vec4::new(ry.cos(),0.,-ry.sin(),0.);
    may[1] = Vec4::new(0.,1.,0.,0.);
    may[2] = Vec4::new(ry.sin(),0.,ry.cos(),0.);
    may[3] = Vec4::new(0.,0.,0.,1.);

    let mut mas = [Vec4::new(0.,0.,0.,0.); 4];
    mas[0] = Vec4::new(20.,0.,0.,0.);
    mas[1] = Vec4::new(0.,20.,0.,0.);
    mas[2] = Vec4::new(0.,0.,20.,0.);
    mas[3] = Vec4::new(0.,0.,0.,1.);

    let mz: Mat4 = Mat4{m: maz};
    let mx: Mat4 = Mat4{m: max};
    let ms: Mat4 = Mat4{m: mas};
    let my: Mat4 = Mat4{m: may};

    let object: Obj = Obj::from_file("bunny.obj", &(ms*my));
    println!(
        "{} vertices, {} triangles",
        object.vertices.len(),
        object.triangles.len()
    );

    let triangles = object.triangles;
    let mut lights: Vec<Light> = Vec::new();

    /*lights.push(Light{
        pos: Vec4::new(-30.,30.,10.,1.),
        col: Vec4::new(0.2,0.8,0.2,1.),
    });*/

    lights.push(Light {
        pos: Vec4::new(-5., 5., 10., 1.),
        col: Vec4::new(0.8, 0.2, 0.2, 1.),
    });

    lights.push(Light{
        pos: Vec4::new(5.,5.,10.,1.),
        col: Vec4::new(0.2,0.2,0.8,1.),
    });

    let mut screen = vec![Vec4::new(0., 0., 0., 1.); RES_X * RES_Y];
    let cam: Vec4 = Vec4::new(-0.75, 2., 10., 1.);

    tracer::raytrace(&mut screen, &triangles, &cam, &lights, RES_X, RES_Y, &mx);

    write_screen("out.ppm", screen, RES_X, RES_Y);
}
