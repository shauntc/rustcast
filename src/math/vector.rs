use std::ops::{Add, Sub, Mul, Neg, Div};

#[derive(Debug, Default, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3{x:0f64, y:0f64, z:0f64}
    }

    pub fn norm(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn normalize(&self) -> Vector3 {
        *self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn dot(&self, v: Vector3) -> f64 {
        *self * v
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scale: f64) -> Self {
        Vector3 { x: self.x * scale, y: self.y * scale, z: self.z * scale }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        vector * self
    }
}

impl Mul for Vector3 {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, val: f64) -> Self {
        self * (1.0 / val)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.x + other.z
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1.0
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl Clone for Vector3 {
    fn clone(&self) -> Self {
        Vector3 {x: self.x, y: self.y, z: self.z}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_length_one() {
        let v = Vector3{x:1.7, y:10.3, z:15.5};
        let normalized = v.normalize();
        let length = normalized.length();
        assert!(length > 0.999);
        assert!(length < 1.001);
    }

}