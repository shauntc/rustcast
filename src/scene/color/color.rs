use image::Rgba;
use std::ops::{Mul, Add, AddAssign};

type ColorChannel = f32;

trait ChannelFormat {
    fn to_u8(self) -> u8;
}

impl ChannelFormat for ColorChannel {
    fn to_u8(self) -> u8 {
        (self * 255.0) as u8
    }
}

#[derive(Debug, Copy)]
pub struct Color {
    pub red: ColorChannel,
    pub green: ColorChannel,
    pub blue: ColorChannel
}

impl Color {
    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        return Rgba ([
            self.red.to_u8(), 
            self.green.to_u8(),
            self.blue.to_u8(),
            0
        ]);
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: rgba[0] as f32 / 255.0,
            green: rgba[1] as f32 / 255.0,
            blue: rgba[2] as f32 / 255.0,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            red: self.red * other as f32,
            blue: self.blue * other as f32,
            green: self.green * other as f32,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Color {
        return Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue
        };
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue
        };
    }
}


impl Clone for Color {
    fn clone(&self) -> Self {
        Color { red: self.red, green: self.green, blue: self.blue }
    }
}