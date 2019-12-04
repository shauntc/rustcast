use crate::material::Material;
use crate::point::Point;
use crate::vector::Vector3;
use crate::ray::{Traceable, Ray};

#[derive(Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub material: Material
}

impl Traceable for Plane {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }

        return None;
    }

    fn surface_normal(&self, _hit_point: &Point) -> Vector3 {
        return -self.normal;
    }
}