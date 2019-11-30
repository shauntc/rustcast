use image::{DynamicImage, GenericImageView};

use crate::scene::{Scene, Sphere, Color};
use crate::point::{Point};

pub fn render(scene: &Scene) -> DynamicImage {
    DynamicImage::new_rgb8(scene.width, scene.height)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_render_scene() {
        let scene = Scene {
            width: 800,
            height: 600,
            fov: 90.0,
            sphere: Sphere {
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
            },
        };

        let img: DynamicImage = render(&scene);
        assert_eq!(scene.width, img.width());
        assert_eq!(scene.height, img.height());
    }
}