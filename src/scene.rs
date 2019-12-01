use crate::point::{Point};
use crate::color::{Color};
use std::vec::Vec;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color
}

#[derive(Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Sphere>,
    pub clear_color: Color
}