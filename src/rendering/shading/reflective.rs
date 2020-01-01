use crate::scene::{Scene, Color, Element};
use crate::raycast::{Ray};
use crate::math::{Point, Vector3};

use super::shade_diffuse;
use super::super::cast_ray;

pub fn shade_reflective(
    scene: &Scene, ray: &Ray, element: &Element,
    hit_point: Point, surface_normal: Vector3,
    reflectivity: f64, depth: u32
) -> Color {
    let mut color = shade_diffuse(scene, element, hit_point, surface_normal);
    let reflection_ray = Ray::create_reflected(surface_normal, ray.direction, hit_point, &scene.sensor);

    color = color * (1.0 - reflectivity) as f32;
    color = color + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity);
    
    return color;
}