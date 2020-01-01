use std::vec::Vec;
use crate::raycast::{Sensor, Ray, Intersectable};
use crate::scene::{Element, Color, Light};

#[derive(Debug)]
pub struct Scene {
    pub sensor: Sensor,
    pub elements: Vec<Element>,
    pub clear_color: Color,
    pub lights: Vec<Light>
}

#[derive(Debug)]
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
    use crate::scene::*;
    use crate::math::{Vector3, Point};

    pub fn test_scene() -> Scene {
        return Scene {
            sensor: Sensor {
                width: 800,
                height: 600,
                fov: 90.0,
                shadow_bias: 1e-12,
                max_recursion_depth: 4
            },
            elements: vec![
                Element::Sphere (Sphere {
                    center: Point { x: 0.0, y: 0.0, z: -5.0},
                    radius: 1.0,
                    material: Material {
                        color: Coloration::Color(Color {red: 0.4, green: 1.0, blue: 0.4}),
                        albedo: 0.18,
                        surface: SurfaceType::Diffuse
                    }
                })
            ],
            clear_color: Color {red: 0.0, green: 0.0, blue: 0.0},
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