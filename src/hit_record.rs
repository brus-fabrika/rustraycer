use std::rc::Rc;

use crate::material::Material;
use crate::{vec3d::Vec3d, Point3d};
use crate::camera::Ray;
use crate::interval::Interval;

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3d,
    pub normal: Vec3d,
    t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3d) {
        // See the hit record normal vector
        // Note: outward_normal is assumed to have unit length
        self.front_face = Vec3d::dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { outward_normal * -1.0 }
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, Rc<dyn Material>)>;
}

pub struct Sphere {
    center: Point3d,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3d, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere{center, radius, material}
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, Rc<dyn Material>)> {
        let oc = self.center.as_vec3d() - r.origin.as_vec3d();
        let a = r.direction.length_squared();
        let h = Vec3d::dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }
        
        let dsqrt = discriminant.sqrt();
        // find the nearest root that lies in the acceptable range
        let root = (h - dsqrt) / a;
        if !ray_t.surrounds(root) {
            let root = (h + dsqrt) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p.as_vec3d() - self.center.as_vec3d()) / self.radius;

        let mut hr = HitRecord {
            t: root,
            point: p,
            normal: Vec3d::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        
        hr.set_face_normal(r, outward_normal);

        Some((hr, self.material.clone()))
    } 
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hit>>
}

impl HittableList {
    pub fn add(&mut self, o: Box<dyn Hit>) {
        self.objects.push(o);
    }
}

impl Hit for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, Rc<dyn Material>)> {
        let mut temp_rec = HitRecord::default();

        let mut closest_so_far = ray_t.max;

        let mut hit_mat: Option<Rc<dyn Material>> = None;

        for o in self.objects.iter() {
            if let Some((hr, m)) = o.hit(r, Interval{min: ray_t.min, max: closest_so_far}) {
                closest_so_far = hr.t;
                temp_rec = hr;
                hit_mat = Some(m);
            }
        }

        if let Some(mat) = hit_mat {
            Some((temp_rec, mat))
        } else {
            None
        }
    } 
}
