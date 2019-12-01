use std::ops::{Add,Mul,Sub,Neg};
use crate::vector::Vector3;

#[derive(Debug, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn zero() -> Point {
        Point{x: 0.0, y: 0.0, z: 0.0}
    }
}

impl Add<Vector3> for Point {
    type Output = Self;

    fn add(self, vector: Vector3) -> Self {
        Point { x: self.x + vector.x, y: self.y + vector.y, z: self.z + vector.z }
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, s: f64) -> Self {
        Point { x: self.x * s, y: self.y * s, z: self.z * s }
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        self * -1.0
    }
}

impl Sub<Vector3> for Point {
    type Output = Self;

    fn sub(self, vector: Vector3) -> Self {
        self + -vector
    }
}

impl Sub for Point {
    type Output = Vector3;

    fn sub(self, p: Point) -> Vector3 {
        Vector3 { x: self.x - p.x, y: self.y - p.y, z: self.z - p.z }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y, z: self.z }
    }
}