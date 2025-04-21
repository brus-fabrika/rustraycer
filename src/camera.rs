use crate::{hit_record::{Hit, HittableList}, interval::Interval, vec3d::Vec3d, Color, Point3d};
use rand::Rng;


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
    samples_per_pixel: u8,          // count of random samples per pixel (antialiasing)
    max_depth: u8,                  // max number of ray bounces into scene (reflaction)
    center: Point3d,
    pixel00_loc: Point3d,
    pixel_delta_u: Vec3d,
    pixel_delta_v: Vec3d,
    pub(super) pixels: Vec<Color>
}

impl Camera {
    pub fn initialize(aspect_ratio: f32, image_width: u16, spp: u8) -> Camera {

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
            samples_per_pixel: spp,
            max_depth: 10,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixels: vec![],
        }
    }

    fn sample_square(&self) -> Vec3d {
        let rnd_x = rand::rng().random_range(0.0..1.0);
        let rnd_y = rand::rng().random_range(0.0..1.0);
        Vec3d::new(rnd_x - 0.5, rnd_y - 0.5, 0.0)
    }

    fn get_ray(&self, i: u16, j: u16) -> Ray {
        // construct a camera ray originating from the origin and directed at randomly
        // sampled point around the pixel location i, j
        let offset = self.sample_square();
        
        let pixel_shift = &Vec3d::add(&Vec3d::mul(&self.pixel_delta_u, f32::from(i) + offset.x), &Vec3d::mul(&self.pixel_delta_v, f32::from(j) + offset.y));
        let pixel_sample = Vec3d::add(&self.pixel00_loc.as_vec3d(), pixel_shift);

        let ray_origin = self.center.clone();
        let ray_direction = Vec3d::sub(&pixel_sample, &ray_origin.as_vec3d());

        Ray::new(ray_origin, ray_direction)
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

                let mut pixel_color = Vec3d::new(0.0, 0.0, 0.0);

                for _ in 0 .. self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    let pc = self.ray_color(&r, self.max_depth, &world);
                    pixel_color = Vec3d::add(&pixel_color, &Vec3d::new(pc.r, pc.g, pc.b));
                }
                
                pixel_color = Vec3d::mul(&pixel_color, 1.0 / f32::from(self.samples_per_pixel));

                self.pixels.push(Color { r: pixel_color.x, g: pixel_color.y, b: pixel_color.z });
            }
        }
    }

    fn ray_color(&self, r: &Ray, depth: u8, world: &HittableList) -> Color {
        if depth == 0 {
            return Color {r: 0.0, g: 0.0, b: 0.0};
        }

        if let Some(hr) = world.hit(r, &Interval::new(0.001, f32::INFINITY)) {
            //let direction = Vec3d::random_on_hemisphere(&hr.normal);
            let direction = hr.normal + Vec3d::random_unit();
            let rc = self.ray_color(&Ray{origin: hr.point, direction}, depth - 1, world);
            return Color{r: rc.r * 0.5, g: rc.g * 0.5, b: rc.b * 0.5};

            //let cv = Vec3d::mul(&Vec3d::add(&hr.normal, &Vec3d::new(1.0, 1.0, 1.0)), 0.5);
            //return Color{r: cv.x, g: cv.y, b: cv.z};
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
