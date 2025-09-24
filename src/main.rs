#![allow(dead_code, unused_variables, unused_imports)]
mod config;
mod camera;
mod vec3d;
mod hit_record;
mod material;
mod interval;
mod aabb;
mod bhv;
mod scene;

use std::{ops::Index, sync::Arc};
use std::fs;
use std::io::Write;

use camera::{Camera, CameraView};

use config::Settings;
use hit_record::{HittableList, Sphere};
use interval::Interval;
use material::{Dielectric, Lambertian, Metal};
use rand::Rng;

use crate::aabb::Aabb;
use crate::bhv::BvhNode;
use crate::{hit_record::Hittable, material::MaterialEnum, vec3d::Vec3d};

#[derive(Debug, Default, Clone)]
struct Point3d(Vec3d);

impl Index<usize> for Point3d {
    type Output = f32;

    fn index(&self, i:usize) -> &f32 {
        match i {
            0 => &self.0.x,
            1 => &self.0.y,
            2 => &self.0.z,
            _ => panic!("Index out of bound for Point3d, should be in range 0..2")
        }
    }
}

impl Point3d {
    fn new(x: f32, y: f32, z: f32) -> Point3d {
        Point3d(Vec3d{x, y, z})
    }

    fn from_vec3d(v: Vec3d) -> Point3d {
        Point3d(v)
    }

    fn _origin() -> Point3d {
        Point3d::new(0.0, 0.0, 0.0)
    }

    fn clone(&self) -> Point3d {
        Point3d(Vec3d{x: self.0.x, y: self.0.y, z: self.0.z})
    }

    fn as_vec3d(&self) -> Vec3d {
        Vec3d::new(self.0.x, self.0.y, self.0.z)
    }

    fn sub(&self, p: Point3d) -> Point3d {
        Point3d::new(
            self.0.x - p.0.x,
            self.0.y - p.0.y,
            self.0.z - p.0.z,
        )
    }
    //fn as_vec3d(p: &Point3d) -> Vec3d {
    //    Vec3d{x: p.0.x, y: p.0.y, z: p.0.z}
    //}
}

#[derive(Copy, Clone, Debug)]
struct Color{r: f32, g: f32, b: f32}

/*impl Color {
    fn as_vec3d(c: &Color) -> Vec3d {
        Vec3d{x: c.r, y: c.g, z: c.b}
    }
}*/

const INTENSITY: Interval = Interval {
    min: 0.000,
    max: 0.999,
};

fn linear_to_gamma(l: f32) -> f32 {
    if l < 0.0 {
        0.0
    } else {
        f32::sqrt(l)
    }
}

fn write_color(f: &mut fs::File, c: &Color) {

    // apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(c.r);
    let g = linear_to_gamma(c.g);
    let b = linear_to_gamma(c.b);

    // translate the [0, 1] component values to the byte (color) range [0, 255]
    let ir = (256.0 * INTENSITY.clamp(r)) as u8;
    let ig = (256.0 * INTENSITY.clamp(g)) as u8;
    let ib = (256.0 * INTENSITY.clamp(b)) as u8;

    writeln!(f, "{ir} {ig} {ib}").expect("Cannot write to file");
}

use raylib::prelude::*;

fn main() {

    let c = Settings::new().unwrap();

    // World
    let world = scene::sea_of_balls_scene(c.clone());

    // Camera
    let cv = CameraView {
        vfov: 20.0,
        lookfrom: Point3d::new(13.0, 2.0, 3.0),
        lookat: Point3d::new(0.0, 0.0, 0.0),
        vup: Vec3d::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    };
    
    let camera = Arc::new(Camera::initialize(16.0 / 9.0, c.width, c.max_depth, c.samples_per_pixel, cv));

    // Render
    use std::time::Instant;
    let now = Instant::now();

    let thread_num = if c.multithread_enabled { c.threads } else { 1 };

    println!("Running renderer with {thread_num} threads");
    println!("Rendering image {}x{}, depth {} and {} samples per pixel",
        camera.image_width, camera.image_height,
        c.max_depth, c.samples_per_pixel);

    Camera::render(camera.clone(), Arc::new(world), thread_num);

    let mut elapsed = now.elapsed();
    println!("Calculated in: {:.2?}", elapsed);
    println!("Saving image to file...");

    let mut f = fs::File::create("rendered.ppm").expect("Cannot create rendered image file");
    writeln!(f, "P3\n{} {}\n255", camera.image_width, camera.image_height).expect("Cannot write to file");
    
    let pixels = camera.pixels.lock().unwrap();

    pixels.iter().for_each(|c| write_color(&mut f, c));

    elapsed = now.elapsed();
    println!("Total elapsed: {:.2?}", elapsed);
    
    let (mut rl, thread) = raylib::init()
        .size(camera.image_width as i32, camera.image_height as i32)
        .title("Hello, World")
        .build();

    rl.set_target_fps(1);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(raylib::color::Color::BLACK);
        //d.draw_text("Hello, world!", 12, 12, 20, raylib::color::Color::BLACK);
        pixels.iter().enumerate().for_each(|(i, c)| {
            // apply a linear to gamma transform for gamma 2
            let r = linear_to_gamma(c.r);
            let g = linear_to_gamma(c.g);
            let b = linear_to_gamma(c.b);

            // translate the [0, 1] component values to the byte (color) range [0, 255]
            let ir = (256.0 * INTENSITY.clamp(r)) as u8;
            let ig = (256.0 * INTENSITY.clamp(g)) as u8;
            let ib = (256.0 * INTENSITY.clamp(b)) as u8;

            let x = i as i32 / camera.image_width as i32;
            let y = i as i32 % camera.image_width as i32;

            d.draw_pixel(y, x, raylib::color::Color::new(ir, ig, ib, 255));
       });
    }
}


