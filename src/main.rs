mod scene;
mod rendering;
mod point;
mod vector;
mod ray;
mod color;
mod material;

use rendering::{render};
use point::Point;
use scene::{Scene, Element};
use scene::sphere::Sphere;
use scene::light::Light;
use material::Material;
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
                origin: Point {x: 0.0, y:-2.0, z: 0.0},
                normal: Vector3 {x: 0.0, y: -1.0, z: 0.0},
                material: Material {
                    color: Color {red: 0.5, green: 0.01, blue: 0.8},
                    albedo: 0.02
                }
            }), 
            Element::Sphere(Sphere {
                center: Point {x: -1.0, y: 1.0, z: -2.0},
                radius: 1.0,
                material: Material {
                    color: Color {red: 1.0, green: 0.01, blue: 0.01},
                    albedo: 0.04
                }
            }),
            Element::Sphere(Sphere {
                center: Point {x: 1.0, y: -1.5, z: -5.0},
                radius: 1.5,
                material: Material {
                    color: Color {red: 0.01, green: 1.0, blue: 0.01},
                    albedo: 0.05
                }
            }),
            Element::Sphere(Sphere {
                center: Point {x: 0.0, y: 2.0, z: -5.0},
                radius: 1.0,
                material: Material {
                    color: Color {red: 0.01, green: 0.01, blue: 1.0},
                    albedo: 0.20
                }
            }),
        ],
        clear_color: Color {red: 0.0, green: 0.0, blue: 0.0},
        shadow_bias: 1e-12,
        light: Light {
            direction: Vector3 {x: -1.0, y:-1.0, z: -1.5},
            color: Color {red: 1.0, green: 1.0, blue: 1.0},
            intensity: 50.0
        }
    };

    let img = render(&scene);
    let path = Path::new("./test.png"); // TODO: Change .gitignore when this is removed
    if img.save(path).is_ok() {
        println!("Image Saved to {:?}", path);
    } else {
        println!("Failed to save image");
    }
}
