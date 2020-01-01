use crate::scene::Color;
use crate::raycast::TextureCoords;
use image::{DynamicImage, GenericImageView};
use std::fmt::Debug;
use std::fmt;

pub enum Coloration {
    Color(Color),
    Texture(DynamicImage)
}

fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

impl Debug for Coloration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Coloration::Color(ref c) => c.fmt(f),
            Coloration::Texture(ref t) => write!(f, "Texture {{ Binary color_type:{:?} }}", t.color())
        }
    }
}

impl Coloration {
    pub fn color(&self, texture_coords: &TextureCoords) -> Color {
        match self {
            Coloration::Color(c) => *c,
            Coloration::Texture(t) => {
                let x = wrap(texture_coords.x as f32, t.width());
                let y = wrap(texture_coords.y as f32, t.height());
                
                return Color::from_rgba(t.get_pixel(x, y))
            }
        }
    }
}

#[derive(Debug)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f64 }
}

#[derive(Debug)]
pub struct Material {
    pub color: Coloration,
    pub albedo: f64,
    pub surface: SurfaceType
}