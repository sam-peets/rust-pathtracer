mod triangle;
mod vec3;

use crate::vec3::Vec3;
use crate::triangle::Triangle;

struct Ray {
    origin: Vec3,
    dir: Vec3,
}

fn main() {
    let mut triangles = Vec::new();
    
    let p1: Vec3 = Vec3{x: 0., y: 0., z: -5.};
    let p2: Vec3 = Vec3{x: 1., y: 0., z: -5.};
    let p3: Vec3 = Vec3{x: 0., y: 1., z: -5.};

    triangles.push(Triangle{p1: p1, p2: p2, p3: p3});

    println!("{} {} {}", triangles[0].p1, triangles[0].p2, triangles[0].p3);
}
