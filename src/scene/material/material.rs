use std::fmt::Debug;

use super::{SurfaceNormal};

use crate::scene::{Coloration, Texture};
use crate::math::*;

#[derive(Debug)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f64 }
}

#[derive(Debug)]
pub struct Material {
    pub color: Coloration,
    pub albedo: f64,
    pub surface: SurfaceType,
    pub normal: SurfaceNormal
}

impl Material {
    pub fn surface_normal(&self, normal: Vector3, tex_coords: Vector2) -> Vector3 {
        match &self.normal {
            SurfaceNormal::Geometry => normal,
            SurfaceNormal::Texture(ref t) => {
                let color = t.point_color(tex_coords);
                let normal = (Vector3 {x: color.red as f64, y: color.green as f64, z: color.blue as f64}).normalize();

                return normal;
            }
        }
    }
}