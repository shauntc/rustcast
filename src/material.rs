use crate::color::Color;

#[derive(Debug, Copy)]
pub struct Material {
    pub color: Color,
    pub albedo: f64
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Material {color: self.color, albedo: self.albedo}
    }
}