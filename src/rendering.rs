use image::{DynamicImage, GenericImage};

use crate::scene::Scene;
use crate::ray::{Ray, Traceable};
use crate::color::{Color};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            let trace_result = scene.trace(&ray);
            if let Some(intersection) = trace_result {
                let hit_point = ray.origin + (ray.direction * intersection.distance);
                let surface_normal = intersection.element.surface_normal(&hit_point);
                let element = intersection.element;
                let material = element.material();

                let mut color = Color {red: 0.0, green: 0.0, blue: 0.0};

                for light in &scene.lights {
                    let direction_to_light = light.direction(&hit_point);
    
                    let shadow_ray = Ray {
                        origin: hit_point + (surface_normal * scene.shadow_bias),
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

                    color += light_color * material.color.color(&texture_coords);
                }

                image.put_pixel(x, y, color.clamp().to_rgba());
            } else {
                image.put_pixel(x, y, scene.clear_color.to_rgba());
            }
        }
    }

    return image;
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::tests::test_scene;
    use image::GenericImageView;

    #[test]
    fn test_can_render_scene() {
        let scene = test_scene();
        let img: DynamicImage = render(&scene);
        assert_eq!(scene.width, img.width());
        assert_eq!(scene.height, img.height());
    }
}