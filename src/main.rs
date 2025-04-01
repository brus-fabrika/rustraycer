mod camera;
mod vec3d;
mod hit_record;
mod interval;

use std::fs;
use std::io::Write;

use camera::Camera;

use hit_record::{HittableList, Sphere};

use crate::vec3d::Vec3d;

const IMAGE_WIDTH: u16 = 800;

#[derive(Debug, Default)]
struct Point3d(Vec3d);

impl Point3d {
    fn new(x: f32, y: f32, z: f32) -> Point3d {
        Point3d(Vec3d{x, y, z})
    }

    fn from_vec3d(v: Vec3d) -> Point3d {
        Point3d(v)
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

fn main() {

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5)));

    world.add(Box::new(Sphere::new(Point3d::new(-1.0, 0.6, -2.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3d::new(5.0, 0.6, -5.0), 1.0)));
    
    world.add(Box::new(Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut camera = Camera::initialize(16.0 / 9.0, IMAGE_WIDTH);

    // Render
    use std::time::Instant;
    let now = Instant::now();

    camera.render(&world);

    let mut elapsed = now.elapsed();
    println!("Calculated in: {:.2?}", elapsed);
    println!("Saving image to file...");

    let mut f = fs::File::create("rendered.ppm").expect("Cannot create rendered image file");
    write!(f, "P3\n{} {}\n255\n", camera.image_width, camera.image_height).expect("Cannot write to file");
    camera.pixels.iter().for_each(|c| write_color(&mut f, c));

    elapsed = now.elapsed();
    println!("Total elapsed: {:.2?}", elapsed);
}


