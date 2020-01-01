use image::{DynamicImage, GenericImage};

use crate::scene::{Scene, Color};
use crate::raycast::{Ray};
use super::shading::{shade};

pub fn render_scene(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.sensor.width, scene.sensor.height);
    for x in 0..scene.sensor.width {
        for y in 0..scene.sensor.height {
            let ray = Ray::create_prime(x, y, &scene.sensor);

            let color = cast_ray(scene, &ray, 0);
            image.put_pixel(x, y, color.clamp().to_rgba());
        }
    }

    return image;
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth >= scene.sensor.max_recursion_depth {
        return scene.clear_color;
    }

    let intersection = scene.trace(&ray);
    if let Some(intersection) = intersection {
        return shade(scene, &ray, &intersection, depth);
    } else {
        return scene.clear_color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::tests::test_scene;
    use image::GenericImageView;

    #[test]
    fn test_can_render_scene() {
        let scene = test_scene();
        let img: DynamicImage = render_scene(&scene);
        assert_eq!(scene.sensor.width, img.width());
        assert_eq!(scene.sensor.height, img.height());
    }
}