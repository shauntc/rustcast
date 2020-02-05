use crate::raycast::{Ray, Intersectable};
use crate::math::{Vector3, Point, Vector2};
use crate::scene::{Material, Sphere, Plane};

#[derive(Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane)
}

impl Element {
    pub fn material(&self) -> &Material {
        match self {
            Element::Sphere(ref s) => &s.material,
            Element::Plane(ref p) => &p.material
        }
    }

}

impl Intersectable for Element {
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
    fn texture_coords(&self, hit_point: &Point) -> Vector2 {
        match self {
            Element::Sphere(ref s) => s.texture_coords(hit_point),
            Element::Plane(ref p) => p.texture_coords(hit_point)
        }
    }
}