#[allow(dead_code)]

use crate::interval::Interval;
use crate::{camera::Ray, Point3d};

#[derive(Clone)]
pub(crate) struct Aabb {
    pub(crate) x: Interval,
    pub(crate) y: Interval,
    pub(crate) z: Interval,
}

impl Aabb {
    pub(crate) fn default() -> Aabb {
        Aabb {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }

    pub(crate) fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb {
            x: Interval { min: x.min, max: x.max }, 
            y: Interval { min: y.min, max: y.max },
            z: Interval { min: z.min, max: z.max },
        }
    }

    pub(crate) fn from_points(a: &Point3d, b: &Point3d) -> Aabb {
        Aabb::new(
            if a.0.x <= b.0.x {Interval::new(a.0.x, b.0.x)} else {Interval::new(b.0.x, a.0.x)}, 
            if a.0.y <= b.0.y {Interval::new(a.0.y, b.0.y)} else {Interval::new(b.0.y, a.0.y)}, 
            if a.0.z <= b.0.z {Interval::new(a.0.z, b.0.z)} else {Interval::new(b.0.z, a.0.z)}, 
        )
    }

    pub(crate) fn from_boxes(b1: Aabb, b2: Aabb) -> Aabb {
        Aabb::new(
            Interval::from_intervals(&b1.x, &b2.x),
            Interval::from_intervals(&b1.y, &b2.y),
            Interval::from_intervals(&b1.z, &b2.z),
        )
    }


    pub(crate) fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x
        }
    }

    pub(crate) fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let mut ray_t = ray_t;

        let ray_orig = &r.origin;
        let ray_direction = &r.direction;
        
        for axis in 0..2 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_direction[axis as usize];
            
            let t0 = (ax.min - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis as usize]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min { ray_t.min = t0; }
                if t1 < ray_t.max { ray_t.max = t1; }
            } else {
                if t1 > ray_t.min { ray_t.min = t1; }
                if t0 < ray_t.max { ray_t.max = t0; }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        
        return true;
    }
}
