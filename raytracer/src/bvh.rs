use crate::aabb::AABB;
use crate::random_int_range;
use crate::ray::Ray;
//use crate::sphere::bounding_box_for_sort_nothing;
use crate::sphere::Object;
use crate::sphere::{BoundingBox, Hit};
use crate::tool_func::unwrap_object;
use crate::HitRecord;

use crate::tool_func::*;

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
    pub fn bvh_node(
        src_objects: &mut Vec<Object>,
        start: u32,
        end: u32,
        time0: f64,
        time1: f64,
    ) -> BVH_node {
        let objects = src_objects;
        //tag!
        //?????????????????????????????????????????????????????
        let axis = random_int_range(0, 2);
        //let tmp_x:
        //let comparator = |x: &Object,y: &Object| {
        //    f64::partial_cmp(
        //
        //    )
        //}
        let box_a = &mut AABB::aabb();
        let box_b = &mut AABB::aabb();

        //怎么解决？？
        //let comparator = |x:&Object,y:&Object| {
        //    f64::partial_cmp(
        //        match x {
        //            Object::None => &bounding_box_for_sort_nothing(box_a).min()[axis as usize],
        //            Object::Sp(t) => &t.bounding_box_for_sort(0.0,0.0,box_a).min()[axis as usize],
        //            Object::Msp(t) => &t.bounding_box_for_sort(0.0,0.0,box_a).min()[axis as usize],
        //            Object::BV(t) => &t.bounding_box_for_sort(0.0,0.0,box_a).min()[axis as usize],
        //        },
        //        match y {
        //            Object::None => &bounding_box_for_sort_nothing(box_b).min()[axis as usize],
        //            Object::Sp(t) => &t.bounding_box_for_sort(0.0,0.0,box_b).min()[axis as usize],
        //            Object::Msp(t) => &t.bounding_box_for_sort(0.0,0.0,box_b).min()[axis as usize],
        //            Object::BV(t) => &t.bounding_box_for_sort(0.0,0.0,box_b).min()[axis as usize],
        //        }
        //    )
        //    .unwrap()
        //};

        let object_span: u32 = end - start;

        let mut lleft: Object; //记录最终构造结果
        let mut rright: Object;

        if object_span == 1 {
            lleft = unwrap_object(&objects[start as usize]);
            rright = unwrap_object(&objects[start as usize]);
        } else if object_span == 2 {
            //to delete
            if AABB::box_x_compare(&objects[start as usize], &objects[(start + 1) as usize]) {
                lleft = unwrap_object(&objects[start as usize]);
                rright = unwrap_object(&objects[(start + 1) as usize]);
            } else {
                lleft = unwrap_object(&objects[(start + 1) as usize]);
                rright = unwrap_object(&objects[start as usize]);
            }
        } else {
            //sort(objects.begin() + start,objects.begin() + end,comparator);

            //objects.sort_unstable_by(comparator);

            let mid = (start + object_span) / 2;
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
        ok = unwrap_object_bounding_box(&lleft, time0, time1, box_left);
        ok = unwrap_object_bounding_box(&rright, time0, time1, box_right);
        //match &lleft {
        //    Object::None => eprintln!("bvh_node constructor false"),
        //    Object::Sp(t) => ok = !t.bounding_box(time0, time1, box_left),
        //    Object::Msp(t) => ok = !t.bounding_box(time0, time1, box_left),
        //    Object::BV(t) => ok = !t.bounding_box(time0, time1, box_left),
        //}
        //match &rright {
        //    Object::None => eprintln!("bvh_node constructor false"),
        //    Object::Sp(t) => ok = !t.bounding_box(time0, time1, box_left),
        //    Object::Msp(t) => ok = !t.bounding_box(time0, time1, box_left),
        //    Object::BV(t) => ok = !t.bounding_box(time0, time1, box_left),
        //}

        if ok {
            eprintln!("No bounding box in bvh_node constructor.");
        }
        //if !lleft.bounding_box(time0,time1,box_left) || !rright.bounding_box(time0,time1,box_right) {
        //    eprintln!("No bounding box in bvh_node constructor.");
        //}

        let bbox = AABB::surrounding_box(box_left.copy(), box_right.copy());

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
