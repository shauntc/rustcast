use crate::color::{Color};
use crate::ray::{Ray, Traceable};
use crate::vector::Vector3;
use crate::point::Point;
use crate::material::Material;

pub mod sphere;
pub mod plane;
pub mod light;

use sphere::Sphere;
use plane::Plane;
use light::Light;

use std::vec::Vec;

#[derive(Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane)
}

impl Element {
    pub fn material(&self) -> Material {
        match self {
            Element::Sphere(ref s) => s.material,
            Element::Plane(ref p) => p.material
        }
    }

}

impl Traceable for Element {
    fn intersects(&self, ray: &Ray) ->  Option<f64> {
        match self {
            Element::Sphere(ref s) => s.intersects(ray),
            Element::Plane(ref p) => p.intersects(ray),
        }
    }
    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point)
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub clear_color: Color,
    pub shadow_bias: f64,
    pub lights: Vec<Light>
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::light::DirectionalLight;

    pub fn test_scene() -> Scene {
        return Scene {
            width: 800,
            height: 600,
            fov: 90.0,
            elements: vec![
                Element::Sphere (Sphere {
                    center: Point { x: 0.0, y: 0.0, z: -5.0},
                    radius: 1.0,
                    material: Material {
                        color: Color {red: 0.4, green: 1.0, blue: 0.4},
                        albedo: 0.18
                    }
                })
            ],
            clear_color: Color {red: 0.0, green: 0.0, blue: 0.0},
            shadow_bias: 1e-12,
            lights: vec![
                Light::Directional( DirectionalLight {
                    direction: Vector3 {x: -1.0, y:-1.0, z: 0.0},
                    color: Color {red: 1.0, green: 1.0, blue: 1.0},
                    intensity: 0.4
                })
            ]
        };
    }
}