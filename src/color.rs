use image::Rgba;

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
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba ([
            self.red.to_u8(), 
            self.blue.to_u8(),
            self.green.to_u8(),
            0
        ])
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color { red: self.red, green: self.green, blue: self.blue }
    }
}