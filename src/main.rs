use std::fs;
use std::io::Write;

use crate::vec3d::Vec3d;

pub mod vec3d;

const IMAGE_WIDTH: u16 = 800;
//const IMAGE_HEIGHT: u16 = 256;

#[derive(Debug)]
struct Point3d(Vec3d);

impl Point3d {
    fn clone(&self) -> Point3d {
        Point3d(Vec3d{x: self.0.x, y: self.0.y, z: self.0.z})
    }

    fn as_vec3d(p: &Point3d) -> Vec3d {
        Vec3d{x: p.0.x, y: p.0.y, z: p.0.z}
    }
}

#[derive(Debug)]
struct Ray {
    origin: Point3d,
    direction: Vec3d,
}

impl Ray {
    fn new(origin: Point3d, direction: Vec3d) -> Ray {
        return Ray{origin, direction} 
    }
    
    fn at(&self, t: f32) -> Point3d {
        let p = Vec3d::add(&self.origin.0, &Vec3d::mul(&self.direction, t));
        Point3d(p)
    }
}

#[derive(Debug)]
struct Color{r: f32, g: f32, b: f32}

/*impl Color {
    fn as_vec3d(c: &Color) -> Vec3d {
        Vec3d{x: c.r, y: c.g, z: c.b}
    }
}*/

fn write_color(f: &mut fs::File, c: &Color) {

    let ir = (255.999 * c.r) as i32;
    let ig = (255.999 * c.g) as i32;
    let ib = (255.999 * c.b) as i32;

    write!(f, "{} {} {}\n", ir, ig, ib).expect("Cannot write to file");
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3d::unit(&r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    let cv = Vec3d::add(&Vec3d::mul(&Vec3d{x: 1.0, y: 1.0, z: 1.0}, 1.0 - a), &Vec3d::mul(&Vec3d{x: 0.5, y: 0.7, z: 1.0}, a));

    Color{r: cv.x, g: cv.y, b: cv.z}
}

fn get_image_height(w: u16, a: f32) -> u16 {
    let hf: f32 = f32::from(w) / a;
    return if hf < 1.0 { 1 } else { hf as u16 }
}

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u16 = IMAGE_WIDTH;

    // Calculate the image height and ensure it is at least 1
    let image_height = get_image_height(image_width, aspect_ratio);
   
    // Camera
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * f32::from(image_width) / f32::from(image_height);
    let camera_center = Point3d(Vec3d{x:0.0, y: 0.0, z: 0.0});

    // viewport vectors
    let viewport_u = Vec3d{x: viewport_width, y: 0.0, z: 0.0};
    let viewport_v = Vec3d{x: 0.0, y: -viewport_height, z: 0.0};

    // pixel delta vectors
    let pixel_delta_u = Vec3d::mul(&viewport_u, 1.0 / f32::from(image_width));
    let pixel_delta_v = Vec3d::mul(&viewport_v, 1.0 / f32::from(image_height));

    // Calculate the location of the upper left pixel
    let viewport_center_camera = Vec3d::sub(&Point3d::as_vec3d(&camera_center), &Vec3d{x: 0.0, y: 0.0, z: focal_length});
    let viewport_center_viewport = Vec3d::add(&Vec3d::mul(&viewport_u, 0.5), &Vec3d::mul(&viewport_v, 0.5));
    let viewport_upper_left = Vec3d::sub(&viewport_center_camera, &viewport_center_viewport);

    let pixel00_loc = Vec3d::add(&viewport_upper_left, &Vec3d::mul(&Vec3d::add(&pixel_delta_u, &pixel_delta_v), 0.5));

    // Render

    let mut f = fs::File::create("rendered.ppm").expect("Cannot create rendered image file");

    write!(f, "P3\n{} {}\n255\n", image_width, image_height).expect("Cannot write to file");
    
    for j in 0 .. image_height {
        let processed = (100.0 * f32::from(j) / f32::from(image_height)) as i32;
        if processed % 10 == 0 {
            println!("Scanlines Processed {} ({}%)...", j, processed);
        }
        
        for i in 0 .. image_width {
            let pixel_shift = Vec3d::add(&Vec3d::mul(&pixel_delta_u, f32::from(i)), &Vec3d::mul(&pixel_delta_v, f32::from(j)));
            let ray_direction = Vec3d::add(&pixel00_loc, &pixel_shift);

            let r = Ray::new(camera_center.clone(), ray_direction);

            let c = ray_color(&r);           
            write_color(&mut f, &c);
        }
    }
}


