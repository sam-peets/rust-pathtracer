use core::f32;
use std::fs::File;

use rayon::iter::ParallelIterator;

use crate::{
    acceleration_structure::{AccelerationStructure, bvh::Bvh, octree::Octree},
    bsdf::{Bsdf, cook_torrance::CookTorrance, lambertian::Lambertian},
    camera::Camera,
    math::{mat4::Mat4, ray::Ray, triangle::Triangle, vec4::Vec4},
    obj::{Material, Obj},
    ppm::{Ppm, Rgb8},
};

mod acceleration_structure;
mod bsdf;
mod camera;
mod math;
mod obj;
mod ppm;

const MAX_DEPTH: usize = 8;
const SKY_COLOR: Vec4 = Vec4::new(0.25, 0.56, 1.0, 1.0);

fn tonemap(color: Vec4) -> Rgb8 {
    let x = color.x().max(0.0);
    let y = color.y().max(0.0);
    let z = color.z().max(0.0);
    let r = (x / (x + 1.0)).powf(1.0 / 2.2);
    let g = (y / (y + 1.0)).powf(1.0 / 2.2);
    let b = (z / (z + 1.0)).powf(1.0 / 2.2);

    Rgb8::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn trace(
    ray: Ray,
    bvh: &impl AccelerationStructure,
    materials: &[Material],
    rng: &mut fastrand::Rng,
) -> Vec4 {
    let mut throughput: Vec4 = Vec4::from([1.0, 1.0, 1.0, 1.0]);
    let mut radiance = Vec4::new(0.0, 0.0, 0.0, 0.0);
    let mut ray = ray;

    for depth in 0..MAX_DEPTH {
        if let Some((t, triangle)) = bvh.intersects(ray) {
            let hit = ray.at(t);
            let material = if let Some(material) = triangle.mtl_id.and_then(|id| materials.get(id))
            {
                material
            } else {
                &Material::default()
            };

            // let bsdf = Lambertian {
            //     albedo: material.kd,
            // };

            let bsdf = CookTorrance {
                albedo: material.kd,
                roughness: material.pr,
                ior: material.ni,
                metallic: material.pm,
            };

            radiance = radiance + material.ke * throughput;

            // let (tangent, bitangent) = triangle.tangent_bitangent();
            let normal = triangle.normal(hit);

            let incoming = ray.direction * -1.0;
            let outgoing = bsdf.sample(incoming, normal, rng);
            let pdf = bsdf.pdf(incoming, outgoing, normal);
            if pdf <= 0.0 {
                break;
            }
            let f = bsdf.eval(incoming, outgoing, normal);
            throughput = throughput * (f / pdf);

            if depth >= 3 {
                let termination_chance = throughput
                    .x()
                    .max(throughput.y())
                    .max(throughput.z())
                    .min(0.95);
                if termination_chance <= 0.0 || rng.f32() > termination_chance {
                    break;
                }
                throughput = throughput / termination_chance;
            }

            ray = Ray::new(hit + normal * 1e-4, outgoing);
        } else {
            radiance = radiance + SKY_COLOR * throughput;
            break;
        }
    }

    radiance
}

fn main() {
    env_logger::init();

    let argv = std::env::args().collect::<Vec<String>>();
    let path = argv.get(1).expect("missing path to obj file");

    let obj = Obj::open(path).unwrap();
    let scaling = Mat4::scaling(Vec4::from([1.0, 1.0, 1.0, 0.0]));
    let rotation = Mat4::rotation(Vec4::from([0.0, 1.0, 0.0, 0.0]), 3.0 * f32::consts::PI);
    dbg!(obj.triangles.len());

    let centroid = obj.centroid();
    dbg!(centroid);

    let translate = Mat4::translation(centroid * -1.0);
    let obj = obj.apply(translate).apply(rotation).apply(scaling);
    let bvh = Bvh::build(obj.triangles, obj.materials);
    let materials = bvh.materials.clone();
    // let octree = Octree::build(obj.triangles, obj.materials);
    // let materials = octree.materials.clone();

    eprintln!("built octree");

    let camera = Camera::new(
        Ray::new(
            Vec4::from([0.0, 0.0, -15.0, 1.0]),
            Vec4::from([0.0, 0.0, 1.0, 0.0]),
        ),
        1.5,
    );

    let width = 32 * 2 * 2 * 2 * 2;
    let height = 32 * 2 * 2 * 2 * 2;

    let mut ppm = Ppm::new(width, height);

    let count = std::sync::atomic::AtomicUsize::new(0);

    let spp = 64 * 16 * 2;
    // let spp = 2 * 2 * 2 * 2;
    let cols: Vec<Rgb8> = camera
        .gen_rays_par_iter(width, height)
        // .gen_rays_iter(width, height)
        .map(|ray| {
            let mut col = Vec4::from([0.0, 0.0, 0.0, 0.0]);
            let mut rng = fastrand::Rng::new();
            for _ in 0..spp {
                let ray = Ray::new(
                    ray.origin,
                    (ray.direction + Vec4::from([rng.f32(), rng.f32(), rng.f32(), 0.0]) * 1e-3)
                        .normalize(),
                );
                col = col + trace(ray, &bvh, &materials, &mut rng);
            }
            let c = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if c.is_multiple_of(1000) {
                eprintln!("{} / {}", c, width * height);
            }
            col / (spp as f32)
        })
        .map(tonemap)
        .collect();

    ppm.buf = cols;

    // println!("{}", ppm.emit())
    let mut outfile = File::create("out.ppm").unwrap();
    let mut w = std::io::BufWriter::new(&mut outfile);
    ppm.write_to(&mut w);
    // outfile.write_all(&ppm.emit().into_bytes()).unwrap();
}
