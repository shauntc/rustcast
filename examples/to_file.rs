use rustcast::render_scene;
use rustcast::math::*;
use rustcast::scene::*;
use rustcast::raycast::*;
use std::path::Path;

fn main() {
    let tex_path = Path::new("./assets/textures/grid.png");
    let texture_res = image::open(tex_path);

    if let Ok(texture) = texture_res {
        let scene = Scene {
            sensor: Sensor {
                width: 800,
                height: 600,
                fov: 90.0,
                shadow_bias: 1e-12,
            },
            elements: vec![
                Element::Plane(Plane {
                    origin: Point {x: 0.0, y:-2.0, z: 0.0},
                    normal: Vector3 {x: 0.0, y: -1.0, z: 0.0},
                    material: Material {
                        color: Coloration::Color(Color {red: 0.2, green: 0.01, blue: 0.8}),
                        albedo: 0.12
                    }
                }), 
                Element::Sphere(Sphere {
                    center: Point {x: -1.0, y: 1.0, z: -3.0},
                    radius: 1.0,
                    material: Material {
                        color: Coloration::Color(Color {red: 1.0, green: 0.01, blue: 0.01}),
                        albedo: 0.04
                    }
                }),
                Element::Sphere(Sphere {
                    center: Point {x: 1.0, y: -1.5, z: -5.0},
                    radius: 1.5,
                    material: Material {
                        color: Coloration::Texture(texture),
                        // color: Coloration::Color(Color {red: 0.01, green: 1.0, blue: 0.01}),
                        albedo: 0.05
                    }
                }),
                Element::Sphere(Sphere {
                    center: Point {x: 0.0, y: 2.0, z: -5.0},
                    radius: 1.0,
                    material: Material {
                        // color: Coloration::Texture(texture),
                        color: Coloration::Color(Color {red: 0.01, green: 0.01, blue: 1.0}),
                        albedo: 0.20
                    }
                }),
            ],
            clear_color: Color {red: 0.0, green: 0.0, blue: 0.0},
            lights: vec![
                Light::Directional(DirectionalLight {
                    direction: Vector3 {x: -1.0, y:-1.0, z: -1.5},
                    color: Color {red: 1.0, green: 1.0, blue: 1.0},
                    intensity: 5.0
                }),
                Light::Spherical(SphericalLight {
                    position: Point {x: 0.0, y:0.0, z: -3.0},
                    color: Color {red: 1.0, green: 1.0, blue: 1.0},
                    intensity: 1000.0
                })
            ]
        };
    
        let img = render_scene(&scene);
        let path = Path::new("./test.png"); // TODO: Change .gitignore when this is removed
        if img.save(path).is_ok() {
            println!("Image Saved to {:?}", path);
        } else {
            println!("Failed to save image");
        }

    } else {
        println!("Failed to load texture");
    }
    

}
