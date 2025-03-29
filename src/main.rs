use std::fs;
use std::io::Write;

use hit_record::{Hit, HittableList, Sphere};

use crate::vec3d::Vec3d;

pub mod vec3d;
pub mod hit_record;

const IMAGE_WIDTH: u16 = 800;

#[derive(Debug, Default)]
struct Point3d(Vec3d);

impl Point3d {
    fn new(x: f32, y: f32, z: f32) -> Point3d {
        Point3d(Vec3d{x, y, z})
    }

    fn origin() -> Point3d {
        Point3d::new(0.0, 0.0, 0.0)
    }

    fn clone(&self) -> Point3d {
        Point3d(Vec3d{x: self.0.x, y: self.0.y, z: self.0.z})
    }

    fn as_vec3d(&self) -> Vec3d {
        Vec3d::new(self.0.x, self.0.y, self.0.z)
    }

    //fn as_vec3d(p: &Point3d) -> Vec3d {
    //    Vec3d{x: p.0.x, y: p.0.y, z: p.0.z}
    //}
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

    write!(f, "{ir} {ig} {ib}\n").expect("Cannot write to file");
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    match world.hit(r, 0.0, f32::INFINITY) {
        Some(hr) => {
            let cv = Vec3d::mul(&Vec3d::add(&hr.normal, &Vec3d::new(1.0, 1.0, 1.0)), 0.5);
            return Color{r: cv.x, g: cv.y, b: cv.z};
        },
        None => ()
    }

    let unit_direction = Vec3d::unit(&r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    let cv = Vec3d::add(&Vec3d::mul(&Vec3d::new(1.0, 1.0, 1.0), 1.0 - a), &Vec3d::mul(&Vec3d::new(0.5, 0.7, 1.0), a));

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
  
    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5)));

    world.add(Box::new(Sphere::new(Point3d::new(-1.0, 0.6, -2.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3d::new(5.0, 0.6, -5.0), 1.0)));
    
    world.add(Box::new(Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * f32::from(image_width) / f32::from(image_height);
    //let camera_center = Point3d::origin();
    let camera_center = Point3d::new(0.0, 0.0, 0.0);

    // viewport vectors
    let viewport_u = &Vec3d::new(viewport_width, 0.0, 0.0);
    let viewport_v = &Vec3d::new(0.0, -viewport_height, 0.0);

    // pixel delta vectors
    let pixel_delta_u = &Vec3d::mul(viewport_u, 1.0 / f32::from(image_width));
    let pixel_delta_v = &Vec3d::mul(viewport_v, 1.0 / f32::from(image_height));

    // Calculate the location of the upper left pixel
    let viewport_center_camera = &Vec3d::sub(&camera_center.as_vec3d(), &Vec3d::new(0.0, 0.0, focal_length));
    let viewport_center_viewport = &Vec3d::add(&Vec3d::mul(viewport_u, 0.5), &Vec3d::mul(viewport_v, 0.5));
    let viewport_upper_left = &Vec3d::sub(viewport_center_camera, viewport_center_viewport);

    let pixel00_loc = &Vec3d::add(viewport_upper_left, &Vec3d::mul(&Vec3d::add(pixel_delta_u, pixel_delta_v), 0.5));

    // Render
    use std::time::Instant;
    let now = Instant::now();

    let mut log_threshold = 0;

    let mut color_vec: Vec<Color> = vec![];

    for j in 0 .. image_height {
        let processed = (100.0 * f32::from(j+1) / f32::from(image_height)) as i32;
        if processed >= log_threshold {
            println!("Scanlines Processed {} ({}%)...", j, processed);
            log_threshold += 10;
        }
        
        for i in 0 .. image_width {
            let pixel_shift = &Vec3d::add(&Vec3d::mul(pixel_delta_u, f32::from(i)), &Vec3d::mul(pixel_delta_v, f32::from(j)));
            let ray_direction = Vec3d::add(pixel00_loc, pixel_shift);

            let r = Ray::new(camera_center.clone(), ray_direction);

            color_vec.push(ray_color(&r, &world));
        }
    }

    let mut elapsed = now.elapsed();
    println!("Calculated in: {:.2?}", elapsed);
    println!("Saving image to file...");

    let mut f = fs::File::create("rendered.ppm").expect("Cannot create rendered image file");
    write!(f, "P3\n{} {}\n255\n", image_width, image_height).expect("Cannot write to file");
    color_vec.iter().for_each(|c| write_color(&mut f, c));

    elapsed = now.elapsed();
    println!("Total elapsed: {:.2?}", elapsed);
}


