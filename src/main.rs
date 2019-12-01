mod scene;
mod rendering;
mod point;
mod vector;
mod ray;
mod color;

use rendering::{render};
use point::Point;
use scene::{Sphere, Scene};
use std::path::Path;
use color::Color;

fn main() {
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

    let img = render(&scene);
    let path = Path::new("./test.png"); // TODO: Change .gitignore when this is removed
    if img.save(path).is_ok() {
        println!("Image Saved to {:?}", path);
    } else {
        println!("Failed to save image");
    }
}
