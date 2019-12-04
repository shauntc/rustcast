use crate::color::{Color, RgbaColor};
use crate::ray::{Ray, Intersect};

pub mod sphere;
pub mod plane;

use image::Rgba;
use sphere::Sphere;
use plane::Plane;

use std::vec::Vec;

#[derive(Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane)
}

impl Intersect for Element {
    fn intersects(&self, ray: &Ray) ->  Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersects(ray),
            Element::Plane(ref p) => p.intersects(ray),
        }
    }
}

impl RgbaColor for Element {
    fn to_rgba(&self) -> Rgba<u8> {
        match self {
            Element::Sphere(ref s) => s.color.to_rgba(),
            Element::Plane(ref p) => p.color.to_rgba()
        }
    }

}

#[derive(Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub clear_color: Color
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element
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