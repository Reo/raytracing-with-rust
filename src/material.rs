use crate::vec::*;

// currently only diffuse materials with base colour base_col
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub base_col: RGBcol,
}

impl Material {
    pub fn create_empty() -> Self { Material {
        base_col: Vec3d::zero()
    } }
}

