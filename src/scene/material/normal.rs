use image::DynamicImage;
use std::fmt::Debug;
use std::fmt;

pub enum SurfaceNormal {
    Geometry,
    Texture(DynamicImage)
}

impl Debug for SurfaceNormal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SurfaceNormal::Geometry => write!(f, "Geometry"),
            SurfaceNormal::Texture(ref t) => write!(f, "Texture {{ Binary color_type:{:?} }}", t.color())
        }
    }
}
