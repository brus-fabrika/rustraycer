use std::fs;
use std::io::Write;

use crate::vec3d::Vec3d;

pub mod vec3d;

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

#[derive(Debug)]
struct Point3d(Vec3d);

#[derive(Debug)]
struct Ray {
    origin: Point3d,
    direction: Vec3d,
}

impl Ray {
    fn at(&self, t: f32) -> Point3d {
        let p = Vec3d::add(&self.origin.0, &Vec3d::mul(&self.direction, t));
        Point3d(p)
    }
}

#[derive(Debug)]
struct Color{r: f32, g: f32, b: f32}

fn write_color(f: &mut fs::File, c: &Color) {

    let ir = (255.999 * c.r) as i32;
    let ig = (255.999 * c.g) as i32;
    let ib = (255.999 * c.b) as i32;

    write!(f, "{} {} {}\n", ir, ig, ib).expect("Cannot write to file");
}

fn ray_color(r: &Ray) -> Color {
    Color{r: 0.0, g: 0.0, b: 0.0}
}

fn get_image_height(w: u16, a: f32) -> u16 {
    let hf: f32 = f32::from(w) / a;
    return if hf < 1.0 { 1 } else { hf as u16 }
}

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u16 = 400;

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
    //let pixel_delta_u = Vec3d::div_scalar(viewport_u, image_width);
    //let pixel_delta_v = Vec3d::div_scalar(viewport_v, image_height);

    // Calculate the location of the upper left pixel
// ...

    // Render


    let mut f = fs::File::create("rendered.ppm").expect("Cannot create rendered image file");
    // Renderer
    write!(f, "P3\n{} {}\n255\n", image_width, image_height).expect("Cannot write to file");
    for j in 0 .. image_height {
        println!("Scanlines remaining {}", image_height - j);
        for i in 0 .. image_width {
            let c = Color{
                r: f32::from(i) / f32::from(image_width - 1),
                g: f32::from(j) / f32::from(image_height - 1),
                b: 0.0};
            
            write_color(&mut f, &c);
        }
    }
}


