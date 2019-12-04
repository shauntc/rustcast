use crate::color::Color;
use crate::point::Point;
use crate::vector::Vector3;
use crate::ray::{Intersect, Ray};

#[derive(Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color
}

impl Intersect for Plane {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        if denom < -1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }

        return None;
    }
}