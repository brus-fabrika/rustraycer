mod config;
mod camera;
mod vec3d;
mod hit_record;
mod material;
mod interval;

use std::sync::Arc;
use std::fs;
use std::io::Write;

use camera::{Camera, CameraView};

use config::Settings;
use hit_record::{HittableList, Sphere};
use interval::Interval;
use material::{Dielectric, Lambertian, Material, Metal};
use rand::Rng;

use crate::vec3d::Vec3d;

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



fn main() {

    let c = Settings::new().unwrap();

    // World
    let mut world = HittableList::default();

    let ground_material: Arc<dyn Material> = match c.ground.material.as_str() {
        "diffuse" => {
            let diffuse = c.ground.diffuse.expect("Ground diffuse params missing");
            Arc::new(Lambertian{albedo: Color{r: diffuse.albedo[0], g: diffuse.albedo[1], b: diffuse.albedo[2]}})
        },
        "metal" => {
            let metal = c.ground.metal.expect("Ground metal params missing");
            Arc::new(Metal{albedo: Color{r: metal.albedo[0], g: metal.albedo[1], b: metal.albedo[2]}, fuzz: metal.fuzz})
        },
        "dielectric" => {
            let dielectric = c.ground.dielectric.expect("Ground dielectric params missing");
            Arc::new(Dielectric{refraction_index: dielectric.refraction})
        },
        _ => Arc::new(Lambertian{albedo: Color{r: 0.5, g: 0.5, b: 0.5,}})
    };

    let ground_point = Point3d::new(c.ground.center[0], c.ground.center[1], c.ground.center[2]);

    world.add(Box::new(Sphere::new(ground_point, c.ground.radius, ground_material)));

    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_mat: f32 = rand::rng().random();
            let center = Point3d::new (
                a as f32 + 0.9 * rand::rng().random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::rng().random::<f32>(),
            );
            
            let t = center.sub(Point3d::new(4.0, 0.2, 0.0)).as_vec3d();
            if t.length() > 0.9 {
                match choose_mat {
                    0.0 .. 0.8 => {
                        // diffuse
                        let albedo = Color{
                            r: rand::rng().random::<f32>() * rand::rng().random::<f32>(),
                            g: rand::rng().random::<f32>() * rand::rng().random::<f32>(),
                            b: rand::rng().random::<f32>() * rand::rng().random::<f32>(),
                        };
                        
                        world.add(
                            Box::new(
                                Sphere::new(center, 0.2, Arc::new(Lambertian{albedo}))
                            )
                        );
                   
                    },
                    
                    0.8 .. 0.95 => {
                        // metal
                        let albedo = Color{
                            r: rand::rng().random_range(0.5 .. 1.0),
                            g: rand::rng().random_range(0.5 .. 1.0),
                            b: rand::rng().random_range(0.5 .. 1.0),
                        };
                        let fuzz = rand::rng().random_range(0.0 .. 0.5);
                        
                        world.add(
                            Box::new(
                                Sphere::new(center, 0.2, Arc::new(Metal{albedo, fuzz}))
                            )
                        );
                    },
                    
                    _ => {
                        world.add(
                            Box::new(
                                Sphere::new(center, 0.2, Arc::new(Dielectric{refraction_index: 1.5}))
                            )
                        );
                    }
                }
            }
        }
    }
    
    world.add(
        Box::new(
            Sphere::new(Point3d::new(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric{refraction_index: 1.5}))
        )
    );
    
    world.add(
        Box::new(
            Sphere::new(Point3d::new(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian{albedo: Color{r: 0.4, g: 0.2, b: 0.1}}))
        )
    );
  
    world.add(
        Box::new(
            Sphere::new(Point3d::new(4.0, 1.0, 0.0), 1.0, Arc::new(Metal{albedo: Color{r: 0.7, g: 0.6, b: 0.5}, fuzz: 0.0}))
        )
    );

/*
    let material_ground = Arc::new(Lambertian{albedo: Color{r: 0.8, g: 0.8, b: 0.0,}});
    let material_center = Arc::new(Lambertian{albedo: Color{r: 0.1, g: 0.2, b: 0.5,}});
    let material_left = Arc::new(Dielectric{refraction_index: 1.5});
    let material_bubble = Arc::new(Dielectric{refraction_index: 1.5});
    let material_right = Arc::new(Metal{albedo: Color{r: 0.8, g: 0.6, b: 0.2,}, fuzz: 1.0});

    world.add(Box::new(Sphere::new(Point3d::new(0.0, 0.0, -1.2), 0.5, material_center.clone())));

    world.add(Box::new(Sphere::new(Point3d::new(-1.0, 0.6, -2.0), 0.5, material_center.clone())));
    world.add(Box::new(Sphere::new(Point3d::new(5.0, 0.6, -5.0), 1.0, material_center.clone())));
    
    world.add(Box::new(Sphere::new(Point3d::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Box::new(Sphere::new(Point3d::new(3.0, 0.0, -1.0), 0.4, material_bubble.clone())));
    
    world.add(Box::new(Sphere::new(Point3d::new(1.0, 0.0, -1.0), 0.5, material_right.clone())));
    
    world.add(Box::new(Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0, material_ground.clone())));
*/
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
}


