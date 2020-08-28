use rustcast::render_scene;
use rustcast::math::*;
use rustcast::scene::*;
use rustcast::raycast::*;
use std::path::Path;
fn main() {
    let scene = Scene {
        sensor: Sensor {
            width: 1600,
            height: 1200,
            fov: 90.0,
            shadow_bias: 1e-12,
            max_recursion_depth: 4
        },
        elements: vec![
            Element::Plane(Plane {
                origin: Point {x: 0.0, y:-2.0, z: 0.0},
                normal: Vector3 {x: 0.0, y: -1.0, z: 0.0},
                material: Material {
                    color: Coloration::Color(Color {red: 0.2, green: 0.01, blue: 0.8}),
                    albedo: 0.12,
                    surface: SurfaceType::Diffuse,
                    normal: SurfaceNormal::Geometry
                }
            }), 
            Element::Sphere(Sphere {
                center: Point {x: -1.5, y: 1.0, z: -3.0},
                radius: 1.0,
                material: Material {
                    color: Coloration::Texture(load_texture("assets/materials/planks/tbykeacs_2K_Albedo.jpg")),
                    albedo: 0.04,
                    surface: SurfaceType::Diffuse,
                    normal: SurfaceNormal::Geometry
                }
            }),
            Element::Sphere(Sphere {
                center: Point {x: 2.0, y: -1.5, z: -5.0},
                radius: 1.5,
                material: Material {
                    color: Coloration::Texture(load_texture("assets/materials/rock/smokagcp_2K_Albedo.jpg")),
                    albedo: 0.05,
                    surface: SurfaceType::Diffuse,
                    normal: SurfaceNormal::Geometry
                }
            }),
            Element::Sphere(Sphere {
                center: Point {x: 0.0, y: 2.0, z: -5.0},
                radius: 1.0,
                material: Material {
                    color: Coloration::Texture(load_texture("assets/materials/metal/se2abbvc_2K_Albedo.jpg")),
                    albedo: 0.20,
                    surface: SurfaceType::Reflective { reflectivity: 0.75 },
                    normal: SurfaceNormal::Geometry
                }
            }),
            Element::Sphere(Sphere {
                center: Point {x: -1.5, y: -1.0, z: -7.0},
                radius: 2.0,
                material: Material {
                    color: Coloration::Color(Color {red: 1.0, green: 1.0, blue: 1.0}),
                    albedo: 0.20,
                    surface: SurfaceType::Reflective { reflectivity: 1.0 },
                    normal: SurfaceNormal::Geometry
                }
            }),
        ],
        clear_color: Color {red: 0.0, green: 0.0, blue: 0.0},
        lights: vec![
            Light::Directional(DirectionalLight {
                direction: Vector3 {x: -1.0, y:-1.0, z: -1.5},
                color: Color {red: 1.0, green: 1.0, blue: 1.0},
                intensity: 10.0
            }),
            Light::Spherical(SphericalLight {
                position: Point {x: 0.0, y:0.0, z: -3.0},
                color: Color {red: 1.0, green: 1.0, blue: 1.0},
                intensity: 1000.0
            })
        ]
    };

    let img = render_scene(&scene);
    let path = Path::new("./example_output.png");
    if img.save(path).is_ok() {
        println!("Image Saved to {:?}", path);
    } else {
        println!("Failed to save image");
    }
}


fn load_texture(path_str: &str) -> image::DynamicImage {
    let tex_path = Path::new(path_str);
    let texture_res = image::open(tex_path);

    return texture_res.unwrap();
}