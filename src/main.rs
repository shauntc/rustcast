mod scene;
mod rendering;
mod point;
mod vector;
mod ray;
mod color;

use rendering::{render};
use point::Point;
use scene::{Scene, Element};
use scene::sphere::Sphere;
use vector::Vector3;
use scene::plane::Plane;
use std::path::Path;
use color::Color;

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        elements: vec![
            Element::Plane(Plane {
                origin: Point {x: 0.0, y:-1.0, z: 0.0},
                normal: Vector3 {x: 0.0, y: 1.0, z: 0.0},
                color: Color {red: 0.1, green: 0.0, blue: 0.5}
            }), 
            Element::Sphere(Sphere {
                center: Point {x: -1.0, y: 1.0, z: -5.0},
                radius: 1.0,
                color: Color {red: 1.0, green: 0.0, blue: 0.0},
            }),
            Element::Sphere(Sphere {
                center: Point {x: 1.0, y: 1.0, z: -5.0},
                radius: 1.0,
                color: Color {red: 0.0, green: 1.0, blue: 0.0},
            }),
            Element::Sphere(Sphere {
                center: Point {x: -1.0, y: -1.0, z: -5.0},
                radius: 1.0,
                color: Color {red: 0.0, green: 0.0, blue: 1.0},
            }),
            Element::Sphere(Sphere {
                center: Point {x: 1.0, y: -1.0, z: -5.0},
                radius: 1.0,
                color: Color {red: 1.0, green: 1.0, blue: 1.0},
            }),
            Element::Sphere(Sphere {
                center: Point {x: 1.0, y: 3.0, z: -5.0},
                radius: 0.5,
                color: Color {red: 0.5, green: 0.0, blue: 0.4}
            }),
            Element::Sphere(Sphere {
                center: Point {x: -2.0, y: -1.0, z: -5.0},
                radius: 2.0,
                color: Color {red: 0.0, green: 0.0, blue: 0.4}
            }),
            Element::Sphere(Sphere {
                center: Point {x: 0.0, y: 0.0, z: -3.0},
                radius: 0.1,
                color: Color {red: 1.0, green: 0.0, blue: 0.0}
            })
        ],
        clear_color: Color {red: 0.0, green: 0.0, blue: 0.0},
    };

    let img = render(&scene);
    let path = Path::new("./test.png"); // TODO: Change .gitignore when this is removed
    if img.save(path).is_ok() {
        println!("Image Saved to {:?}", path);
    } else {
        println!("Failed to save image");
    }
}
