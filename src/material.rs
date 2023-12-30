use crate::vec4::Vec4;

// a subset of the .obj/.mtl with pbr extension
// essentially just whatever blender exports

pub struct Material {
    pub name: String, // material name
    pub ns: f64, // specular exponent
    pub ka: Vec4, // ambient colour
    pub kd: Vec4, // diffuse colour
    pub ks: Vec4, // specular colour
    pub ke: Vec4, // emmisive colour, TODO with pathtracing
    pub Ni: f64, // optical density, not implemented
    pub d: f64, // dissolve/transparency, not implemented
    pub illum: usize, // illumination model
}
