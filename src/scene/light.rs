use crate::color::Color;
use crate::vector::Vector3;

#[derive(Debug)]
pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f64
}