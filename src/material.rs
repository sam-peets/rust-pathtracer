use crate::vec4::Vec4;

// a subset of the .obj/.mtl with pbr extension
// essentially just whatever blender exports

#[derive(Copy, Clone)]
pub struct Material {
    pub ns: f64,      // specular exponent
    pub ka: Vec4,     // ambient colour
    pub kd: Vec4,     // diffuse colour
    pub ks: Vec4,     // specular colour
    pub ke: Vec4,     // emmisive colour, TODO with pathtracing
    pub ni: f64,      // optical density, not implemented
    pub d: f64,       // dissolve/transparency, not implemented
    pub illum: usize, // illumination model
}

impl Material {
    pub fn default() -> Material {
        Material {
            ns: 40.,
            ka: Vec4::new(0.01, 0.01, 0.01, 1.),
            kd: Vec4::new(0.8, 0.8, 0.8, 1.),
            ks: Vec4::new(0.5, 0.5, 0.5, 1.),
            ke: Vec4::new(0., 0., 0., 0.),
            ni: 0.,
            d: 0.,
            illum: 2,
        }
    }
}
