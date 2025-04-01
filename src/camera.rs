use crate::{hit_record::{Hit, HittableList}, interval::Interval, vec3d::Vec3d, Color, Point3d};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3d,
    pub direction: Vec3d,
}

impl Ray {
    fn new(origin: Point3d, direction: Vec3d) -> Ray {
        return Ray{origin, direction} 
    }
    
    pub fn at(&self, t: f32) -> Point3d {
        let p = Vec3d::add(&self.origin.0, &Vec3d::mul(&self.direction, t));
        Point3d(p)
    }
}

#[derive(Default)]
pub struct Camera {
    aspect_ratio: f32,
    pub(super) image_width: u16,
    pub(super) image_height: u16,
    center: Point3d,
    pixel00_loc: Point3d,
    pixel_delta_u: Vec3d,
    pixel_delta_v: Vec3d,
    pub(super) pixels: Vec<Color>
}

impl Camera {
    pub fn initialize(aspect_ratio: f32, image_width: u16) -> Camera {

         // Calculate the image height and ensure it is at least 1
        let image_height = get_image_height(image_width, aspect_ratio);
      
        // Camera
        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width = viewport_height * f32::from(image_width) / f32::from(image_height);
        //let camera_center = Point3d::origin();
        let center = Point3d::new(0.0, 0.0, 0.0);

        // viewport vectors
        let viewport_u = &Vec3d::new(viewport_width, 0.0, 0.0);
        let viewport_v = &Vec3d::new(0.0, -viewport_height, 0.0);

        // pixel delta vectors
        let pixel_delta_u = Vec3d::mul(viewport_u, 1.0 / f32::from(image_width));
        let pixel_delta_v = Vec3d::mul(viewport_v, 1.0 / f32::from(image_height));

        // Calculate the location of the upper left pixel
        let viewport_center_camera = &Vec3d::sub(&center.as_vec3d(), &Vec3d::new(0.0, 0.0, focal_length));
        let viewport_center_viewport = &Vec3d::add(&Vec3d::mul(viewport_u, 0.5), &Vec3d::mul(viewport_v, 0.5));
        let viewport_upper_left = &Vec3d::sub(viewport_center_camera, viewport_center_viewport);

        let pixel00_loc = Point3d::from_vec3d(Vec3d::add(viewport_upper_left, &Vec3d::mul(&Vec3d::add(&pixel_delta_u, &pixel_delta_v), 0.5)));

   
        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixels: vec![],
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        let mut log_threshold = 0;

        for j in 0 .. self.image_height {
            let processed = (100.0 * f32::from(j+1) / f32::from(self.image_height)) as i32;
            if processed >= log_threshold {
                println!("Scanlines Processed {} ({}%)...", j, processed);
                log_threshold += 10;
            }
            
            for i in 0 .. self.image_width {
                let pixel_shift = &Vec3d::add(&Vec3d::mul(&self.pixel_delta_u, f32::from(i)), &Vec3d::mul(&self.pixel_delta_v, f32::from(j)));
                let ray_direction = Vec3d::add(&self.pixel00_loc.as_vec3d(), pixel_shift);

                let r = Ray::new(self.center.clone(), ray_direction);

                self.pixels.push(self.ray_color(&r, &world));
            }
        }
    }

    fn ray_color(&self, r: &Ray, world: &HittableList) -> Color {
        if let Some(hr) = world.hit(r, &Interval::new(0.0, f32::INFINITY)) {
            let cv = Vec3d::mul(&Vec3d::add(&hr.normal, &Vec3d::new(1.0, 1.0, 1.0)), 0.5);
            return Color{r: cv.x, g: cv.y, b: cv.z};
        }

        let unit_direction = Vec3d::unit(&r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);

        let cv = Vec3d::add(&Vec3d::mul(&Vec3d::new(1.0, 1.0, 1.0), 1.0 - a), &Vec3d::mul(&Vec3d::new(0.5, 0.7, 1.0), a));

        Color{r: cv.x, g: cv.y, b: cv.z}
    }
}

fn get_image_height(w: u16, a: f32) -> u16 {
    let hf: f32 = f32::from(w) / a;
    return if hf < 1.0 { 1 } else { hf as u16 }
}
