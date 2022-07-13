use crate::aabb::AABB;
use crate::random_int_range;
use crate::ray::Ray;
use crate::sphere::bounding_box_for_sort_nothing;
use crate::sphere::Object;
use crate::sphere::*;
use crate::sphere::{BoundingBox, Hit};
use crate::tool_func::unwrap_object;
use crate::HitRecord;

use crate::tool_func::*;
use std::cmp::Ordering;

//所有被hittable派生的都要加入object!!!
//算上一种形状
pub struct BVH_node {
    pub boxx: AABB, //tag!
    pub left: Object,
    pub right: Object,
}

impl BVH_node {
    pub fn copy(&self) -> BVH_node {
        BVH_node {
            boxx: self.boxx.copy(),
            left: unwrap_object(&self.left),
            right: unwrap_object(&self.right),
        }
    }

    pub fn new_by_three(list: &mut HittableList, time0: f64, time1: f64) -> BVH_node {
        let len = list.objects.len();
        BVH_node::bvh_node(&mut list.objects, 0, len as u32, time0, time1)
    }
    //tag here 7.13 !
    pub fn bvh_node(
        src_objects: &mut Vec<Object>,
        start: u32,
        end: u32,
        time0: f64,
        time1: f64,
    ) -> BVH_node {
        let objects: &mut Vec<Object> = src_objects;
        //for object in src_objects {
        //    objects.push(unwrap_object(object));
        //}

        let axis = random_int_range(0, 2);

        let box_a = &mut AABB::aabb();
        let box_b = &mut AABB::aabb();

        //怎么解决？？
        let comparator = |x: &Object, y: &Object| {
            match x {
                Object::None => bounding_box_for_sort_nothing(box_a),
                Object::Sp(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
                Object::Msp(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
                Object::BV(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
                Object::XY(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
                Object::XZ(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
                Object::YZ(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
                Object::Bo(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
                Object::Ro(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
                Object::Tr(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
                Object::Co(t) => t.bounding_box_for_sort(0.0, 0.0, box_a),
            };
            match y {
                Object::None => bounding_box_for_sort_nothing(box_b),
                Object::Sp(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
                Object::Msp(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
                Object::BV(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
                Object::XY(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
                Object::XZ(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
                Object::YZ(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
                Object::Bo(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
                Object::Ro(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
                Object::Tr(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
                Object::Co(t) => t.bounding_box_for_sort(0.0, 0.0, box_b),
            };
            f64::partial_cmp(&(box_a.min()[axis as usize]), &(box_b.min()[axis as usize])).unwrap()
        };

        let object_span: u32 = end - start;

        let mut lleft: Object; //记录最终构造结果
        let mut rright: Object;

        if object_span == 1 {
            lleft = unwrap_object(&objects[start as usize]);
            rright = unwrap_object(&objects[start as usize]);
        } else if object_span == 2 {
            //to delete
            if AABB::box_compare(
                &objects[start as usize],
                &objects[(start + 1) as usize],
                axis,
            ) {
                lleft = unwrap_object(&objects[start as usize]);
                rright = unwrap_object(&objects[(start + 1) as usize]);
            } else {
                lleft = unwrap_object(&objects[(start + 1) as usize]);
                rright = unwrap_object(&objects[start as usize]);
            }
        } else {
            //sort(objects.begin() + start,objects.begin() + end,comparator);

            //objects.sort_unstable_by(comparator);
            objects.sort_unstable_by(comparator);
            //sort fuck !!!!!!!!!!!!

            let mid = start + object_span / 2;
            //f*** you!
            lleft = Object::BV(Box::new(BVH_node::bvh_node(
                objects, start, mid, time0, time1,
            )));
            rright = Object::BV(Box::new(BVH_node::bvh_node(
                objects, mid, end, time0, time1,
            )));
        }

        let box_left = &mut AABB::aabb();
        let box_right = &mut AABB::aabb();

        let mut ok = false;
        ok = unwrap_object_bounding_box_no(&lleft, time0, time1, box_left);
        ok = unwrap_object_bounding_box_no(&rright, time0, time1, box_right);

        if ok {
            eprintln!("No bounding box in bvh_node constructor.");
        }

        let bbox = AABB::surrounding_box(box_left.copy(), box_right.copy());
        //eprintln!("over here");
        BVH_node {
            boxx: bbox,
            left: lleft,
            right: rright,
        }
    }
    pub fn new(bo: AABB, l: Object, r: Object) -> BVH_node {
        BVH_node {
            boxx: bo,
            left: l,
            right: r,
        }
    }
}

impl BoundingBox for BVH_node {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.boxx.copy();
        true
    }
    fn bounding_box_for_sort(&self, time0: f64, time1: f64, output_box: &mut AABB) -> AABB {
        self.bounding_box(time0, time1, output_box);
        output_box.copy()
    }
}
impl Hit for BVH_node {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.boxx.hit(r.copy(), t_min, t_max) {
            return false;
        }

        let hit_left: bool;
        hit_left = unwrap_object_hit(&self.left, r.copy(), t_min, t_max, rec);
        //match &self.left {
        //    Object::None => eprintln!("bvh_node hit false"),
        //    Object::Sp(t) => hit_left = t.hit(r.copy(), t_min, t_max, rec),
        //    Object::Msp(t) => hit_left = t.hit(r.copy(), t_min, t_max, rec),
        //    Object::BV(t) => hit_left = t.hit(r.copy(), t_min, t_max, rec),
        //}

        let hit_right: bool;
        hit_right = unwrap_object_hit(&self.right, r.copy(), t_min, t_max, rec);

        hit_left || hit_right
    }
}
/*
unwrap template :
match self.right {
    Object::None => eprintln!("bvh_node hit false"),
    Object::Sp(t) => hit_right = t.hit(r.copy(),t_min,t_max,rec),
    Object::Msp(t) => hit_right = t.hit(r.copy(),t_min,t_max,rec),
    Object::BV(t) => hit_right = t.hit(r.copy(),t_min,t_max,rec),
}
*/
