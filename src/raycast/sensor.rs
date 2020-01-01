#[derive(Debug)]
pub struct Sensor {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub shadow_bias: f64,
    pub max_recursion_depth: u32
}