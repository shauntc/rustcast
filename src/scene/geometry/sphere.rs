use crate::math::{Vector3, Point, Vector2};
use crate::raycast::{Intersectable, Ray};
use crate::scene::Material;

use std::f64::consts::PI;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let l: Vector3 = self.center - ray.origin;
        let r_sq = self.radius * self.radius;
        let adj = l.dot(ray.direction);
        let d_sq = l.dot(l) - (adj * adj);
        if d_sq > r_sq {
            // If the ray doesn't pass through the sphere we return nothing
            return None;
        }
        let depth = (r_sq - d_sq).sqrt();
        let i_one = adj - depth;
        let i_two = adj + depth;
        if i_one < 0.0 && i_two < 0.0 {
            return None;
        }
        let distance = if i_one < i_two { i_one } else { i_two };

        return Some(distance);
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        return (*hit_point - self.center).normalize();
    }

    fn texture_coords(&self, hit_point: &Point) -> Vector2 {
        let hit_vec = *hit_point - self.center;
        return Vector2 {
            x: (1.0 + hit_vec.z.atan2(hit_vec.x) / PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() / PI
        }
    }
}
