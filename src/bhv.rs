use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use crate::{aabb::Aabb, camera::Ray, hit_record::{Hit, HitRecord, Hittable, HittableList}, interval::Interval, material::MaterialEnum};

#[derive(Clone)]
pub(crate) struct BvhNode {
    left: Arc<Hittable>,
    right: Arc<Hittable>,
    bbox: Aabb,
}

impl Hit for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, Arc<MaterialEnum>)> {
        if !self.bbox.hit(r, ray_t.clone()) {
            return None;
        }

        if let Some((hr_l, mat)) = self.left.hit(r, ray_t.clone()) {
            if let Some((hr_r, mat)) = self.right.hit(r, Interval::new(ray_t.min, hr_l.t)) {
                return Some((hr_r, mat));
            } else {
                return Some((hr_l, mat));
            }
        } else {
            if let Some((hr_r, mat)) = self.right.hit(r, Interval::new(ray_t.min, ray_t.max)) {
                return Some((hr_r, mat));
            }
        }

        return None;
    }
    
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

}

fn box_compare(a: &Hittable, b: &Hittable, axis_index: i32) -> Ordering {
    if a.bounding_box().axis_interval(axis_index).min < b.bounding_box().axis_interval(axis_index).min - 0.001 {
        Ordering::Less
    } else if a.bounding_box().axis_interval(axis_index).min > b.bounding_box().axis_interval(axis_index).min + 0.001 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn x_axis_comparator(a: &Hittable, b: &Hittable) -> Ordering {
    box_compare(a, b, 0)
}

fn y_axis_comparator(a: &Hittable, b: &Hittable) -> Ordering {
    box_compare(a, b, 1)
}

fn z_axis_comparator(a: &Hittable, b: &Hittable) -> Ordering {
    box_compare(a, b, 2)
}

impl BvhNode {
    pub(crate) fn new(hittable_list: &mut HittableList) -> BvhNode {
        let x = hittable_list.objects.len();
        return BvhNode::from_list(&mut hittable_list.objects, 0, x);
    }

    fn from_list(objects: &mut Vec<Hittable>, start: usize, end: usize) -> BvhNode {

        let left;
        let right;

        let mut bbox = Aabb::empty();
        
        for o in &objects[start .. end] {
            bbox = Aabb::from_boxes(bbox, o.bounding_box().clone());
        }
        
        let axis = bbox.longest_axis();
        

        let object_span = end - start;
        match object_span {
            1 => { 
                left = Arc::new(objects[start].clone());
                right = Arc::new(objects[start].clone());

            },
            2 => {
                left = Arc::new(objects[start].clone());
                right = Arc::new(objects[start + 1].clone());
            },
            _ => {
                //let axis = rand::rng().random_range(0 ..= 2);
                let comp = match axis {
                    0 => x_axis_comparator,
                    1 => y_axis_comparator,
                    _ => z_axis_comparator
                };

                objects[start..end].sort_by(comp);
                
                let mid = start + object_span / 2;
                left = Arc::new(Hittable::BvhNode(BvhNode::from_list(objects, start, mid)));
                right = Arc::new(Hittable::BvhNode(BvhNode::from_list(objects, mid, end)));
            }
        }

        //let bbox = Aabb::from_boxes(left.bounding_box().clone(), right.bounding_box().clone());

        BvhNode {
            left,
            right,
            bbox
        }
    }
}
