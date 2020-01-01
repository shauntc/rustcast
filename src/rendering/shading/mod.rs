mod diffuse;
mod reflective;

use reflective::shade_reflective;
use diffuse::shade_diffuse;

use crate::scene::{SurfaceType, Color, Intersection, Scene};
use crate::raycast::{Ray, Intersectable};

pub fn shade(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);
    let element = intersection.element;

    match intersection.element.material().surface {
        SurfaceType::Diffuse => shade_diffuse(scene, element, hit_point, surface_normal),
        SurfaceType::Reflective { reflectivity } => shade_reflective(scene, ray, element, hit_point, surface_normal, reflectivity, depth),
    }
}
