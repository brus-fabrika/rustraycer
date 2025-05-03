use core::f32;

use crate::{hit_record::{Hit, HittableList}, interval::Interval, vec3d::Vec3d, Color, Point3d};
use rand::Rng;


#[derive(Debug)]
pub struct Ray {
    pub origin: Point3d,
    pub direction: Vec3d,
}

impl Ray {
    pub fn new(origin: Point3d, direction: Vec3d) -> Ray {
        Ray{origin, direction} 
    }
    
    pub fn at(&self, t: f32) -> Point3d {
        let p = Vec3d::add(&self.origin.0, &Vec3d::mul(&self.direction, t));
        Point3d(p)
    }
}

pub struct CameraView {
    pub vfov: f32,              // vertical view angle (field of view)
    pub lookfrom: Point3d,      // point camera is looking from
    pub lookat: Point3d,        // point camera is looking at
    pub vup: Vec3d,             // camera-relative up direction
    pub defocus_angle: f32,     // variation angle of rays through each pixel
    pub focus_dist: f32,        // distance from camera lookfrom point to plane of perfect focus
}

#[derive(Default)]
pub struct Camera {
    //aspect_ratio: f32,
    pub(super) image_width: u16,
    pub(super) image_height: u16,
    samples_per_pixel: u16,          // count of random samples per pixel (antialiasing)
    max_depth: u8,                  // max number of ray bounces into scene (reflaction)
    center: Point3d,
    pixel00_loc: Point3d,
    pixel_delta_u: Vec3d,
    pixel_delta_v: Vec3d,
    pub(super) pixels: Vec<Color>,

    defocus_angle: f32,

    defocus_disk_u: Vec3d,
    defocus_disk_v: Vec3d,

    //pub(super) vfov: f32, // vertical view angle (field of view)
    //pub(super) lookfrom: Point3d, // point camera is looking from
    //pub(super) lookat: Point3d, // point camera is looking at
    //pub(super) vup: Vec3d, // camera-relative up direction

    //u: Vec3d, // camera frame basis vectors
    //v: Vec3d,
    //w: Vec3d,
}

const BLACK_COLOR: Color = Color{r: 0.0, g: 0.0, b: 0.0};
const BLACK_VEC:Vec3d = Vec3d{x: 0.0, y: 0.0, z: 0.0};
const SOMECOLOR_VEC:Vec3d = Vec3d{x: 0.5, y: 0.7, z: 1.0};

impl Camera {
    pub fn initialize(aspect_ratio: f32, image_width: u16, max_depth: u8, spp: u16, cv: CameraView) -> Camera {

         // Calculate the image height and ensure it is at least 1
        let image_height = get_image_height(image_width, aspect_ratio);
        
        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        let w = Vec3d::unit(&(cv.lookfrom.as_vec3d() - cv.lookat.as_vec3d()));
        let u = Vec3d::unit(&Vec3d::cross(&cv.vup, &w));
        let v = Vec3d::cross(&w, &u);

        // Camera
        // Determine viewport dimensions
        //let focal_length: f32 = Vec3d::sub(&cv.lookfrom.as_vec3d(), &cv.lookat.as_vec3d()).length();

        let theta = cv.vfov * 2.0 * f32::consts::PI / 360.0;
        let h = f32::tan(theta / 2.0);

        let viewport_height: f32 = 2.0 * h * cv.focus_dist;
        let viewport_width = viewport_height * f32::from(image_width) / f32::from(image_height);

        let center = cv.lookfrom.clone();

        // viewport vectors
        let viewport_u = &Vec3d::mul(&u, viewport_width);
        let viewport_v = &Vec3d::mul(&v, -viewport_height);

        // pixel delta vectors
        let pixel_delta_u = Vec3d::mul(viewport_u, 1.0 / f32::from(image_width));
        let pixel_delta_v = Vec3d::mul(viewport_v, 1.0 / f32::from(image_height));

        // Calculate the location of the upper left pixel
        let viewport_center_camera = &Vec3d::sub(&center.as_vec3d(), &Vec3d::mul(&w, cv.focus_dist));
        let viewport_center_viewport = &Vec3d::add(&Vec3d::mul(viewport_u, 0.5), &Vec3d::mul(viewport_v, 0.5));
        let viewport_upper_left = &Vec3d::sub(viewport_center_camera, viewport_center_viewport);

        let pixel00_loc = Point3d::from_vec3d(Vec3d::add(viewport_upper_left, &Vec3d::mul(&Vec3d::add(&pixel_delta_u, &pixel_delta_v), 0.5)));

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = cv.focus_dist * f32::tan((cv.defocus_angle / 2.0) * 2.0 * f32::consts::PI / 360.0);
        let defocus_disk_u = Vec3d::mul(&u, defocus_radius); 
        let defocus_disk_v = Vec3d::mul(&v, defocus_radius); 

        Camera {
            //aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel: spp,
            max_depth,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixels: vec![],
            defocus_angle: cv.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            //vfov: cv.vfov,
            //lookfrom: cv.lookfrom,
            //lookat: cv.lookat,
            //vup: cv.vup,
            //u,
            //v,
            //w,
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
        
        let pixel_shift = Vec3d::add(&Vec3d::mul(&self.pixel_delta_u, f32::from(i) + offset.x), &Vec3d::mul(&self.pixel_delta_v, f32::from(j) + offset.y));
        let pixel_sample = self.pixel00_loc.as_vec3d() + pixel_shift;
        let ray_origin = if self.defocus_angle > 0.0 { self.defocus_disk_sample() } else { self.center.clone() };
        let ray_direction = pixel_sample - ray_origin.as_vec3d();
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3d {
        let p = Vec3d::random_in_unit_disk();
        Point3d::new(
            self.center.0.x + p.x * self.defocus_disk_u.x + p.y * self.defocus_disk_v.x,
            self.center.0.y + p.x * self.defocus_disk_u.y + p.y * self.defocus_disk_v.y,
            self.center.0.z + p.x * self.defocus_disk_u.z + p.y * self.defocus_disk_v.z
        )
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

                let mut pixel_color = BLACK_VEC; 

                for _ in 0 .. self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    let pc = self.ray_color(r, self.max_depth, world);
                    pixel_color = pixel_color + Vec3d::new(pc.r, pc.g, pc.b);
                }
                
                pixel_color = pixel_color / f32::from(self.samples_per_pixel);

                self.pixels.push(Color { r: pixel_color.x, g: pixel_color.y, b: pixel_color.z });
            }
        }
    }

    fn ray_color(&self, r: Ray, depth: u8, world: &HittableList) -> Color {
        if depth == 0 {
            return BLACK_COLOR; 
        }

        if let Some((hr, hit_mat)) = world.hit(&r, Interval{min: 0.001, max: f32::INFINITY}) {
            // TODO: refactor this!!!
            let (scat_ray, scat_color, scattered) = hit_mat.scatter(&r, &hr);
            return if scattered {
                let rc = self.ray_color(scat_ray, depth - 1, world);
                Color{r: rc.r * scat_color.r, g: rc.g * scat_color.g, b: rc.b * scat_color.b}
            } else {
                BLACK_COLOR
            }; 
        }

        let unit_direction = Vec3d::unit(&r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);

        let cv = BLACK_VEC * (1.0 - a) + SOMECOLOR_VEC * a;

        Color{r: cv.x, g: cv.y, b: cv.z}
    }
}

fn get_image_height(w: u16, a: f32) -> u16 {
    let hf: f32 = f32::from(w) / a;
    if hf < 1.0 { 1 } else { hf as u16 }
}
