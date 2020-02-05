use crate::raycast::Ray;
use crate::math::{Point, Vector3, Vector2};

pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn texture_coords(&self, hit_point: &Point) -> Vector2;
}