use crate::point::{Point};
use crate::color::{Color};
use crate::ray::{Ray, Intersect};
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

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Sphere
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements.iter().filter_map(|element|
            element.intersects(ray).map(|distance|
                Intersection {distance, element}
            )
        ).min_by(|i1, i2|
            i1.distance.partial_cmp(&i2.distance).unwrap()
        )
    }
}