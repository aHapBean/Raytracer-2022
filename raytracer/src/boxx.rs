use crate::aabb::*;
use crate::material::*;
use crate::mod_vec3::*;
use crate::ray::*;
use crate::rect::*;
use crate::sphere::*;
type Point3 = Vec3;
pub struct Boxx {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: HittableList,
}

impl Boxx {
    pub fn new(p0: Point3, p1: Point3, ptr: &Material) -> Boxx {
        let box_min = p0.copy();
        let box_max = p1.copy();

        let mut sides = HittableList::hittablelist();
        sides.add(Object::XY(XYrect::new(p0.x, p1.x, p0.y, p1.y, p1.z, &ptr)));
        sides.add(Object::XY(XYrect::new(p0.x, p1.x, p0.y, p1.y, p0.z, &ptr)));

        sides.add(Object::XZ(XZrect::new(p0.x, p1.x, p0.z, p1.z, p1.y, &ptr)));
        sides.add(Object::XZ(XZrect::new(p0.x, p1.x, p0.z, p1.z, p0.y, &ptr)));
        //fuck

        sides.add(Object::YZ(YZrect::new(p0.y, p1.y, p0.z, p1.z, p1.x, &ptr)));
        sides.add(Object::YZ(YZrect::new(p0.y, p1.y, p0.z, p1.z, p0.x, &ptr)));
        Boxx {
            box_min,
            box_max,
            sides,
        }
    }

    pub fn copy(&self) -> Boxx {
        Boxx {
            box_min: (self.box_min.copy()),
            box_max: (self.box_max.copy()),
            sides: (self.sides.copy()),
        }
    }
    //pub fn new()
}
impl Hit for Boxx {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r.copy(), t_min, t_max, rec)
    }
}

impl BoundingBox for Boxx {
    fn bounding_box(&self, time0: f64, time1: f64, output_Boxx: &mut AABB) -> bool {
        *output_Boxx = AABB::new(self.box_min.copy(), self.box_max.copy());
        //eprintln!("bbbox对的.min[0] :{}", output_Boxx.max()[0]);
        true
    }
    fn bounding_box_for_sort(&self, time0: f64, time1: f64, output_Boxx: &mut AABB) -> AABB {
        self.bounding_box(time0, time1, output_Boxx);
        output_Boxx.copy()
    }
}
