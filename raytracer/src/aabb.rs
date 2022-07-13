use crate::ray::Ray;
use crate::sphere::*;

use crate::material::fmin;
use crate::mod_vec3::Vec3;
use crate::tool_func::unwrap_object_bounding_box_no;
type Point3 = Vec3;

pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn aabb() -> AABB {
        AABB {
            minimum: Point3::new(0.0, 0.0, 0.0),
            maximum: Point3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn new(mi: Point3, ma: Point3) -> AABB {
        AABB {
            minimum: mi,
            maximum: ma,
        }
    }
    pub fn min(&self) -> Point3 {
        self.minimum.copy()
    }
    pub fn max(&self) -> Point3 {
        self.maximum.copy()
    }
    pub fn copy(&self) -> AABB {
        AABB {
            minimum: self.minimum.copy(),
            maximum: self.maximum.copy(),
        }
    }

    pub fn hit(&self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let invD: f64 = 1.0 as f64 / r.copy().direction()[a];
            let mut t0: f64 = (self.min().copy()[a] - r.copy().origin()[a]) * invD;
            let mut t1: f64 = (self.max().copy()[a] - r.copy().origin()[a]) * invD;
            if invD < 0.0 {
                //tag!
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Point3::new(
            fmin(box0.min().x, box1.min().x),
            fmin(box0.min().y, box1.min().y),
            fmin(box0.min().z, box1.min().z),
        );
        let big = Point3::new(
            fmin(box0.max().x, box1.max().x),
            fmin(box0.max().y, box1.max().y),
            fmin(box0.max().z, box1.max().z),
        );
        AABB {
            minimum: small,
            maximum: big,
        }
    }

    pub fn box_compare(a: &Object, b: &Object, axis: i32) -> bool {
        let box_a: AABB = AABB::aabb();
        let box_b: AABB = AABB::aabb();

        let box_a = &mut AABB::aabb();
        let box_b = &mut AABB::aabb();

        let mut ok = unwrap_object_bounding_box_no(b, 0.0, 0.0, box_a);
        ok = unwrap_object_bounding_box_no(b, 0.0, 0.0, box_b);

        if ok {
            eprintln!("No bounding box in bvh_node constructor.");
        }

        box_a.min()[axis as usize] < box_b.min()[axis as usize]
    }

    pub fn box_x_compare(a: &Object, b: &Object) -> bool {
        AABB::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a: &Object, b: &Object) -> bool {
        AABB::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a: &Object, b: &Object) -> bool {
        AABB::box_compare(a, b, 2)
    }
}
