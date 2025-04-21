use crate::{vec3d::Vec3d, Point3d};
use crate::camera::Ray;
use crate::interval::Interval;

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3d,
    pub normal: Vec3d,
    t: f32,
    front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3d) {
        // See the hit record normal vector
        // Note: outward_normal is assumed to have unit length
        self.front_face = Vec3d::dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal.clone() } else { Vec3d::mul(outward_normal, -1.0) }
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3d,
    radius: f32
}

impl Sphere {
    pub fn new(center: Point3d, radius: f32) -> Sphere {
        Sphere{center, radius}
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = Vec3d::sub(&self.center.as_vec3d(), &r.origin.as_vec3d());
        let a = r.direction.length_squared();
        let h = Vec3d::dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }
        
        let dsqrt = discriminant.sqrt();
        // find the nearest root that lies in the acceptable range
        let mut root = (h - dsqrt) / a;
        if !ray_t.surrounds(root) {
            root = (h + dsqrt) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = &Vec3d::mul(&Vec3d::sub(&p.as_vec3d(), &self.center.as_vec3d()), 1.0 / self.radius);

        let mut hr = HitRecord {
            t: root,
            point: p,
            normal: Vec3d::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        
        hr.set_face_normal(r, outward_normal);

        Some(hr)
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
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut temp_rec = HitRecord::default();

        let mut closest_so_far = ray_t.max;

        for o in self.objects.iter() {
            match o.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                Some(hr) => {
                    hit_anything = true;
                    closest_so_far = hr.t;
                    temp_rec = hr;
                },
                None => ()
            }
        }

        if hit_anything {
            Some(temp_rec)
        } else {
            None
        }
    } 
}
