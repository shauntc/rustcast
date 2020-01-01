use crate::math::{Vector3,Point};
use crate::raycast::Sensor;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3
}

/// Image Sensor Width
static SENSOR_WIDTH: f64 = 2.0;

impl Ray {
    pub fn create_prime(x: u32, y:u32, sensor: &Sensor) -> Ray {
        assert!(sensor.width > sensor.height);
        let aspect_ratio = sensor.width as f64 / sensor.height as f64;
        let fov_adjustment = (sensor.fov.to_radians() / 2.0).tan();
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
        let sensor_x = ((((x as f64 + 0.5) / sensor.width as f64) - 0.5) * SENSOR_WIDTH) * aspect_ratio;
        let sensor_y = (0.5 - ((y as f64 + 0.5) / sensor.height as f64)) * SENSOR_WIDTH;

        return Ray {
            origin: Point::zero(),
            direction: (Vector3 {
                x: sensor_x * fov_adjustment, y: sensor_y * fov_adjustment, z: -1.0
            }).normalize()
        }
    }

    pub fn create_reflected(normal: Vector3, incident: Vector3, intersection: Point, sensor: &Sensor) -> Ray {
        return Ray {
            /* origin is adjusted away from the sphere by the shadow bias distance */
            origin: intersection + (sensor.shadow_bias * normal),
            // TODO: Explain
            direction: (incident - (2.0 * incident.dot(normal) * normal)).normalize()
        }
    }
}