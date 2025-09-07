use std::{cmp::Ordering, sync::Arc};

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

// (HittableList) world = hittable_list(make_shared<bvh_node>(world))
// ----------------------- new hittable_list( add(world) )

// new bvh_node from world (HittableList)
// new HittableList from bvh_node (as Hittable Object)


impl BvhNode {
    pub(crate) fn new(hittable_list: &mut HittableList) -> BvhNode {
        let x = hittable_list.objects.len();
        return BvhNode::from_list(&mut hittable_list.objects, 0, x);
    }

    fn from_list(objects: &mut Vec<Hittable>, start: usize, end: usize) -> BvhNode {
        let _axis = 0; // TODO: put random 0-1-2 here

        let mut left = Arc::new(objects[start].clone());
        let mut right = Arc::new(objects[start].clone());

        let object_span = end - start;
        match object_span {
            1 => { /*do nothing*/ },
            2 => {
                right = Arc::new(objects[start + 1].clone());
            },
            _ => {
                // std::sort(std::begin(objects) + start, std::begin(objects)+ end, comparator)
                objects[start..end].sort_by(x_axis_comparator);
                let mid = start + object_span / 2;
                left = Arc::new(Hittable::BvhNode(BvhNode::from_list(objects, start, mid)));
                right = Arc::new(Hittable::BvhNode(BvhNode::from_list(objects, mid, end)));
                // left = make_shared<BvhNode>(objects, start, mid);
                // right = make_shared<BvhNode>(objects, midm end);
            }
        }

        let bbox = Aabb::from_boxes(left.bounding_box().clone(), right.bounding_box().clone());

        BvhNode {
            left,
            right,
            bbox
        }
    }
}
