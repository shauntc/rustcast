use image::{DynamicImage, GenericImageView, ColorType};
use std::fmt;
use std::fmt::Debug;

use crate::math::Vector2;

use super::color::*;

pub trait Texture {
    fn point_color(&self, coordinates: Vector2) -> Color;
}

impl Texture for DynamicImage {
    fn point_color(&self, coordinates: Vector2) -> Color {
        let x = wrap(coordinates.x as f32, self.width());
        let y = wrap(coordinates.y as f32, self.height());
        
        assert!(self.color().channel_count() >= 2, "Too few color channels");
        return Color::from_rgba(self.get_pixel(x, y))
    }
}

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
    pub fn color(&self, texture_coords: Vector2) -> Color {
        match self {
            Coloration::Color(c) => *c,
            Coloration::Texture(t) => t.point_color(texture_coords)
        }
    }
}