use std::sync::Arc;

use crate::aabb::Aabb;
use crate::bhv::BvhNode;
use crate::material::MaterialEnum;
use crate::{vec3d::Vec3d, Point3d};
use crate::camera::Ray;
use crate::interval::Interval;

#[derive(Default)]
pub(crate) struct HitRecord {
    pub(crate) point: Point3d,
    pub(crate) normal: Vec3d,
    pub(crate) t: f32,
    pub(crate) front_face: bool
}

impl HitRecord {
    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3d) {
        // See the hit record normal vector
        // Note: outward_normal is assumed to have unit length
        self.front_face = Vec3d::dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { outward_normal * -1.0 }
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, Arc<MaterialEnum>)>;
    fn bounding_box(&self) -> &Aabb;
}

#[derive(Clone)]
pub struct Sphere {
    //center: Point3d,
    center: Ray,
    radius: f32,
    material: Arc<MaterialEnum>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3d, radius: f32, material: Arc<MaterialEnum>) -> Sphere {
        Self::new_dynamic(center.clone(), center, radius, material)
    }

    pub fn new_dynamic(center: Point3d, center2: Point3d, radius: f32, material: Arc<MaterialEnum>) -> Sphere {
        let d = center2.as_vec3d() - center.as_vec3d();
        let rvec = Vec3d::new(radius, radius, radius);

        let center = Ray::new(center, d, None);

        let box1 = Aabb::from_points(
            &Point3d::from_vec3d(center.at(0.0).as_vec3d() - rvec.clone()),
            &Point3d::from_vec3d(center.at(0.0).as_vec3d() + rvec.clone()),

        );
        
        let box2 = Aabb::from_points(
            &Point3d::from_vec3d(center.at(1.0).as_vec3d() - rvec.clone()),
            &Point3d::from_vec3d(center.at(1.0).as_vec3d() + rvec.clone()),

        );

        let bbox = Aabb::from_boxes(box1, box2);

        Sphere {
            center,
            radius: radius.max(0.0), 
            material,
            bbox
        }
    }

}

impl Hit for Sphere {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, Arc<MaterialEnum>)> {
        let current_center = self.center.at(r.tm);
        let oc = current_center.as_vec3d() - r.origin.as_vec3d();
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
        let outward_normal = (p.as_vec3d() - current_center.as_vec3d()) / self.radius;

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

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Hittable>,
    pub bbox: Aabb,
}

impl HittableList {
    pub fn new(o: Hittable) -> HittableList {
        let mut hl = HittableList{
            objects: vec![],
            bbox: Aabb::default(),
        };
        hl.add(o);
        return hl;
    }
    
    pub fn add(&mut self, o: Hittable) {
        self.bbox = Aabb::from_boxes(self.bbox.clone(), o.bounding_box().clone());
        self.objects.push(o);
    }
}

impl Hit for HittableList {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, Arc<MaterialEnum>)> {
        let mut temp_rec = HitRecord::default();

        let mut closest_so_far = ray_t.max;

        let mut hit_mat: Option<Arc<MaterialEnum>> = None;

        for o in self.objects.iter() {
            if let Some((hr, m)) = o.hit(r, Interval{min: ray_t.min, max: closest_so_far}) {
                closest_so_far = hr.t;
                temp_rec = hr;
                hit_mat = Some(m);
            }
        }

        hit_mat.map(|mat| (temp_rec, mat))
    } 
}

#[derive(Clone)]
pub enum Hittable {
    Sphere(Sphere),
    List(HittableList),
    BvhNode(BvhNode),
}

impl Hit for Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, Arc<MaterialEnum>)> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(r, ray_t),
            Hittable::List(list) => list.hit(r, ray_t),
            Hittable::BvhNode(bvh_node) => bvh_node.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> &Aabb {
         match self {
            Hittable::Sphere(sphere) => sphere.bounding_box(),
            Hittable::List(list) => list.bounding_box(),
            Hittable::BvhNode(bvh_node) => bvh_node.bounding_box(),
        }
    }
}
