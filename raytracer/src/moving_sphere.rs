use crate::material::*;
use crate::mod_vec3::*;
use crate::sphere::*;

use crate::aabb::AABB;
//use crate::material::Material::{Diel, Lam, Met};
//use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::*;
use crate::tool_func::*;
type Point3 = Vec3;

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Material,
}

impl MovingSphere {
    //pub MovingSphere() -> MovingSphere {
    //    MovingSphere {
    //        center0:
    //    }
    //}
    pub fn new(c0: Point3, c1: Point3, t0: f64, t1: f64, r: f64, mat: Material) -> MovingSphere {
        MovingSphere {
            center0: c0,
            center1: c1,
            time0: t0,
            time1: t1,
            radius: r,
            mat_ptr: mat,
        }
    }
    pub fn copy(&self) -> MovingSphere {
        MovingSphere {
            center0: self.center0.copy(),
            center1: self.center1.copy(),
            time0: self.time0,
            time1: self.time1,
            radius: self.radius,
            mat_ptr: unwrap_material(&self.mat_ptr),
        }
    }
    pub fn center(&self, time: f64) -> Point3 {
        self.center0.copy()
            + ((time - self.time0) / (self.time1 - self.time0))
                * (self.center1.copy() - self.center0.copy())
    }
}
impl Hit for MovingSphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // the pub is implied ??
        let oc: Vec3 = r.origin() - self.center(r.time());

        let a: f64 = r.direction().dot(r.direction());
        let half_b: f64 = r.direction().dot(oc.copy()); //所有函数都不引用。+.copy()就可以
        let c: f64 = oc.dot(oc.copy()) - self.radius * self.radius;
        let hb = half_b;
        let det = hb * half_b - a * c;
        if det < 0.0 {
            return false;
        }

        let sqdet = det.sqrt();
        let mut rt = (-half_b - sqdet) / a;
        if rt < t_min || rt > t_max {
            rt = (-half_b + sqdet) / a;
            if rt < t_min || rt > t_max {
                return false;
            }
        }
        //???
        //tag
        rec.t = rt;
        rec.p = r.copy().at(rec.t); //copy()
                                    //here ???
        rec.normal = (rec.p.copy() - self.center(r.time())) / self.radius;
        rec.mat_ptr = self.copy().mat_ptr; //???
                                           //?????!!!!!

        let outward_normal = (rec.p.copy() - self.center(r.time())) / self.radius;
        rec.set_face_normal(r.copy(), outward_normal.copy());

        //使用方法时不要加copy 不然方法可能无法更改到你自身
        true
    }
}

impl BoundingBox for MovingSphere {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        *output_box = AABB::surrounding_box(box0, box1);
        true
    }
    fn bounding_box_for_sort(&self, time0: f64, time1: f64, output_box: &mut AABB) -> AABB {
        self.bounding_box(time0, time1, output_box);
        output_box.copy()
    }
}
