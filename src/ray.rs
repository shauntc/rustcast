use crate::point::Point;
use crate::vector::Vector3;
use crate::scene::{Scene, Sphere};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3
}

pub trait Intersect {
    fn intersects(&self, ray: &Ray) -> bool;
}

impl Intersect for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector3 = self.center - ray.origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(l) - (adj2 * adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        return d2 < (self.radius * self.radius);
    }
}

/// Image Sensor Width
static SENSOR_WIDTH: f64 = 2.0;

impl Ray {
    pub fn create_prime(x: u32, y:u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let aspect_ratio = scene.width as f64 / scene.height as f64;
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        /*
        Steps (for x and y sensor position):
            + 0.5
                Get the center of the pixel, ie 0,1,2...800 becomes 0.5, 1.5, 2.5...799.5
            / scene.width (width resolution of the camera)
                Normalize range of values so they all fall within 0.0 to 1.0
            - 0.5
                Center on zero so the values fall within -0.5 to 0.5
            * SENSOR_WIDTH
                Spread the values to the full width of the sensor, ie with sensor width 2.0 values range -1.0 to 1.0
            * aspect_ratio
                increate the width of the sensor plane on the x axis to prevent image distortion
        */        
        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) - 0.5) * SENSOR_WIDTH) * aspect_ratio;
        let sensor_y = (0.5 - ((y as f64 + 0.5) / scene.height as f64)) * SENSOR_WIDTH;

        return Ray {
            origin: Point::zero(),
            direction: (Vector3 {
                x: sensor_x * fov_adjustment, y: sensor_y * fov_adjustment, z: -1.0
            }).normalize()
        }
    }
}