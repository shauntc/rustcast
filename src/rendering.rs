use image::{DynamicImage, GenericImage};

use crate::scene::Scene;
use crate::ray::{Ray, Traceable};
use crate::color::RgbaColor;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            let trace_result = scene.trace(&ray);
            if let Some(intersection) = trace_result {
                let hit_point = ray.origin + (ray.direction * intersection.distance);
                let direction_to_light = -scene.light.direction.normalize();
                let surface_normal = intersection.element.surface_normal(&hit_point);

                let shadow_ray = Ray {
                    origin: hit_point + (surface_normal * scene.shadow_bias),
                    direction: direction_to_light
                };
                let in_light = scene.trace(&shadow_ray).is_none();

                let light_intensity = if in_light { scene.light.intensity } else { 0.0 };

                let light_power = (surface_normal.dot(direction_to_light)).max(0.0) * light_intensity;
                let light_reflected = intersection.element.material().albedo / std::f64::consts::PI;
                let color = intersection.element.material().color
                    * scene.light.color * light_power as f32 * light_reflected  as f32;

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