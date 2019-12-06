#[derive(Debug)]
pub struct Sensor {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub shadow_bias: f64,
}