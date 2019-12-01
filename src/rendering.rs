use image::{DynamicImage, GenericImage};

use crate::scene::{Scene};
use crate::ray::{Ray, Intersect};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            let sphere = &scene.elements[0];
            if sphere.intersects(&ray) {
                image.put_pixel(x, y, sphere.color.to_rgba());
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
    use crate::scene::Sphere;
    use crate::color::Color;
    use crate::point::Point;
    use image::GenericImageView;

    #[test]
    fn test_can_render_scene() {
        let scene = Scene {
            width: 800,
            height: 600,
            fov: 90.0,
            elements: vec![
                Sphere {
                    center: Point {
                        x: 0.0,
                        y: 0.0,
                        z: -5.0,
                    },
                    radius: 1.0,
                    color: Color {
                        red: 0.4,
                        green: 1.0,
                        blue: 0.4,
                    },
                }
            ],
            clear_color: Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
        };

        let img: DynamicImage = render(&scene);
        assert_eq!(scene.width, img.width());
        assert_eq!(scene.height, img.height());
    }
}