use crate::vec::RGBcol;

// currently only diffuse materials with base colour base_col
#[derive(Debug, PartialEq)]
pub struct Material {
    base_col: RGBcol;
}
