use crate::color::Color;
use crate::vector::Vector3;
use crate::point::Point;

#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f64
}

#[derive(Debug)]
pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f64
}

#[derive(Debug)]
pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight)
}

impl Light {
    pub fn direction(&self, hit_point: &Point) -> Vector3 {
        match self {
            Light::Directional(ref d) => -d.direction.normalize(),
            Light::Spherical(ref s) => (s.position - *hit_point).normalize()
        }
    }
    pub fn distance(&self, hit_point: &Point) -> f64 {
        match *self {
            Light::Directional(_) => std::f64::INFINITY,
            Light::Spherical(ref s) => (s.position - *hit_point).length(),
        }
    }
    pub fn intensity(&self, hit_point: &Point) -> f64 {
        match self {
            Light::Directional(ref d) => d.intensity,
            Light::Spherical(ref s) => s.intensity / (4.0 * std::f64::consts::PI * (s.position - *hit_point).norm())
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Light::Directional(ref d) => d.color,
            Light::Spherical(ref s) => s.color
        }
    }
}