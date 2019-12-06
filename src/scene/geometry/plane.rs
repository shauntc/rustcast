use crate::math::{Vector3, Point};
use crate::raycast::{Intersectable, Ray, TextureCoords};
use crate::scene::{Material};

#[derive(Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub material: Material
}

impl Intersectable for Plane {
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

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3::zero());
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {x:0.0, y:1.0, z:0.0});
        }
        let y_axis = self.normal.cross(&x_axis);

        let hit_vec = *hit_point -  self.origin;
        
        return TextureCoords {
            x: hit_vec.dot(x_axis),
            y: hit_vec.dot(y_axis)
        }
    }
}