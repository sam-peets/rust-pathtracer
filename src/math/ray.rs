use crate::math::vec4::Vec4;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec4,
    pub direction: Vec4,
}

impl Ray {
    pub fn new(origin: Vec4, direction: Vec4) -> Self {
        Self { origin, direction }
    }
}
