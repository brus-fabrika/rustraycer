use std::sync::Arc;

use rand::Rng;

use crate::{aabb::Aabb, bhv::BvhNode, config::Settings, hit_record::{Hittable, HittableList, Sphere}, material::{Dielectric, Lambertian, MaterialEnum, Metal}, Color, Point3d};

pub(crate) fn sea_of_balls_scene(c: Settings) -> HittableList {
    let mut world = HittableList {
        objects: vec![],
        bbox: Aabb::default(),
    };

    let ground_material: Arc<MaterialEnum> = match c.ground.material.as_str() {
        "diffuse" => {
            let diffuse = c.ground.diffuse.expect("Ground diffuse params missing");
            let l = Lambertian{albedo: Color{r: diffuse.albedo[0], g: diffuse.albedo[1], b: diffuse.albedo[2]}};
            Arc::new(MaterialEnum::Lambertian(l))
        },
        "metal" => {
            let metal = c.ground.metal.expect("Ground metal params missing");
            let m = Metal{albedo: Color{r: metal.albedo[0], g: metal.albedo[1], b: metal.albedo[2]}, fuzz: metal.fuzz};
            Arc::new(MaterialEnum::Metal(m))
        },
        "dielectric" => {
            let dielectric = c.ground.dielectric.expect("Ground dielectric params missing");
            let d = Dielectric{refraction_index: dielectric.refraction};
            Arc::new(MaterialEnum::Dielectric(d))
        },
        _ => Arc::new(MaterialEnum::Lambertian(Lambertian{albedo: Color{r: 0.5, g: 0.5, b: 0.5}}))
    };

    let ground_point = Point3d::new(c.ground.center[0], c.ground.center[1], c.ground.center[2]);

    world.add(Hittable::Sphere(Sphere::new(ground_point, c.ground.radius, ground_material)));

    for a in (-110 .. 110).step_by(10) {
        for b in (-110 .. 110).step_by(10) {
            // generate only 20% of objects
            if rand::rng().random::<f32>() < 0.0 {
                continue;
            }
            let choose_mat: f32 = rand::rng().random();
            let center = Point3d::new (
                a as f32 * 0.1 + 0.9 * rand::rng().random::<f32>(),
                0.2,
                b as f32 * 0.1 + 0.9 * rand::rng().random::<f32>(),
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
                        
                       /* 
                        let c2 = Vec3d::new(
                            0.0,
                            rand::rng().random_range(0.0 .. 0.5),
                            0.0,
                        );
                        
                        let center2 = Point3d::from_vec3d(center.as_vec3d() + c2);
                       */ 
                        
                        world.add(
                            Hittable::Sphere(
                                //Sphere::new_dynamic(center, center2, 0.2, Arc::new(MaterialEnum::Lambertian(Lambertian{albedo})))
                                Sphere::new(center, 0.2, Arc::new(MaterialEnum::Lambertian(Lambertian{albedo})))
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
                            Hittable::Sphere(
                                Sphere::new(center, 0.2, Arc::new(MaterialEnum::Metal(Metal{albedo, fuzz})))
                            )
                        );
                    },
                    
                    _ => {
                        world.add(
                            Hittable::Sphere(
                                Sphere::new(center, 0.2, Arc::new(MaterialEnum::Dielectric(Dielectric{refraction_index: 1.5})))
                            )
                        );
                    }
                }
            }
        }
    }
    
    world.add(
        Hittable::Sphere(
            Sphere::new(Point3d::new(0.0, 1.0, 0.0), 1.0, Arc::new(MaterialEnum::Dielectric(Dielectric{refraction_index: 1.5})))
        )
    );
    
    world.add(
        Hittable::Sphere(
            Sphere::new(Point3d::new(-4.0, 1.0, 0.0), 1.0, Arc::new(MaterialEnum::Lambertian(Lambertian{albedo: Color{r: 0.4, g: 0.2, b: 0.1}})))
        )
    );
  
    world.add(
        Hittable::Sphere(
            Sphere::new(Point3d::new(4.0, 1.0, 0.0), 1.0, Arc::new(MaterialEnum::Metal(Metal{albedo: Color{r: 0.7, g: 0.6, b: 0.5}, fuzz: 0.0})))
        )
    );

/*
    let material_ground = Arc::new(Lambertian{albedo: Color{r: 0.8, g: 0.8, b: 0.0,}});
    let material_center = Arc::new(Lambertian{albedo: Color{r: 0.1, g: 0.2, b: 0.5,}});
    let material_left = Arc::new(Dielectric{refraction_index: 1.5});
    let material_bubble = Arc::new(Dielectric{refraction_index: 1.5});
    let material_right = Arc::new(Metal{albedo: Color{r: 0.8, g: 0.6, b: 0.2,}, fuzz: 1.0});

    world.add(Sphere::new(Point3d::new(0.0, 0.0, -1.2), 0.5, material_center.clone()));

    world.add(Sphere::new(Point3d::new(-1.0, 0.6, -2.0), 0.5, material_center.clone()));
    world.add(Sphere::new(Point3d::new(5.0, 0.6, -5.0), 1.0, material_center.clone()));
    
    world.add(Sphere::new(Point3d::new(-1.0, 0.0, -1.0), 0.5, material_left.clone()));
    world.add(Sphere::new(Point3d::new(3.0, 0.0, -1.0), 0.4, material_bubble.clone()));
    
   world.add(Sphere::new(Point3d::new(1.0, 0.0, -1.0), 0.5, material_right.clone()));
    
    world.add(Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0, material_ground.clone()));
*/

    println!("Rendering World with {} hittable objects - using bounding box optimization", world.len());
    
    HittableList::new(Hittable::BvhNode(BvhNode::new(&mut world)))
} 
