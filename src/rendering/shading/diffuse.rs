use crate::scene::{Scene, Color, Element};
use crate::raycast::{Ray, Intersectable};
use crate::math::{Point, Vector3};

pub fn shade_diffuse(scene: &Scene, element: &Element, hit_point: Point, surface_normal: Vector3) -> Color {
    let material = element.material();
    let mut color = Color {red: 0.0, green: 0.0, blue: 0.0};

    for light in &scene.lights {
        let direction_to_light = light.direction(&hit_point);

        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.sensor.shadow_bias),
            direction: direction_to_light
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none()
            || shadow_intersection.unwrap().distance > light.distance(&hit_point);

        let light_intensity = if in_light { light.intensity(&hit_point) } else { 0.0 };
        let light_power = (surface_normal.dot(direction_to_light)).max(0.0) * light_intensity;

        let light_reflected = material.albedo / std::f64::consts::PI;
        let light_color = light.color() * light_power as f32 * light_reflected  as f32;

        let texture_coords = element.texture_coords(&hit_point);

        color += light_color * material.color.color(texture_coords);
    }
    
    return color;
}