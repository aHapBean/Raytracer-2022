use crate::aabb::*;
use crate::degree_to_radians;
use crate::fmax;
use crate::material::*;
use crate::mod_vec3::*;
use crate::ray::Ray;
use crate::sphere::*;
use crate::tool_func::*;
const INFINITY: f64 = 1.79769e+40;
type Point3 = Vec3;
pub struct Translate {
    ptr: Object,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: &Object, displacement: Vec3) -> Translate {
        //eprintln!("the tr offset:{}",displacement[0]);
        //eprintln!("dis:{:#?}",displacement);

        Translate {
            ptr: unwrap_object(p),
            offset: displacement,
        }
    }
    pub fn copy(&self) -> Translate {
        Translate {
            ptr: unwrap_object(&self.ptr),
            offset: self.offset.copy(),
        }
    }
}
impl Hit for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(
            r.origin().copy() - self.offset.copy(),
            r.direction().copy(),
            r.time(),
        );

        if !unwrap_object_hit(&self.ptr, moved_r.copy(), t_min, t_max, rec) {
            return false;
        }

        rec.p = rec.p.copy() + self.offset.copy();
        rec.set_face_normal(moved_r.copy(), rec.normal.copy());

        true
    }
}
impl BoundingBox for Translate {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !unwrap_object_bounding_box_yes(&self.ptr, time0, time1, output_box) {
            return false;
        }
        *output_box = AABB::new(
            output_box.min().copy() + self.offset.copy(),
            output_box.max().copy() + self.offset.copy(),
        );
        true
    }
    fn bounding_box_for_sort(&self, time0: f64, time1: f64, output_Boxx: &mut AABB) -> AABB {
        self.bounding_box(time0, time1, output_Boxx);
        output_Boxx.copy()
    }
}

pub struct RotateY {
    ptr: Object,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(p: &Object, angle: f64) -> RotateY {
        let radians = degree_to_radians(angle);
        //eprintln!("the rad is {}",radians);
        let sin_theta = radians.sin();
        //eprintln!("the sin is {}",sin_theta);
        let cos_theta = radians.cos();

        let bbbox = &mut AABB::aabb();
        //eprintln!("bbbox.min[0] :{}", bbbox.max()[0]);
        let hasbox = unwrap_object_bounding_box_yes(p, 0.0, 1.0, bbbox);
        //if hasbox {eprintln!("there is a box!");}
        //eprintln!("post bbbox.min[0] :{}", bbbox.max()[0]);
        //else {eprintln!("there is not a box!");}
        //eprintln!("run rotate!");
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    //eprintln!("bbbox.min[0] :{}",bbbox.max()[0]);
                    let x = i as f64 * bbbox.max().x() + (1 - i) as f64 * bbbox.min().x();
                    let y = j as f64 * bbbox.max().y() + (1 - j) as f64 * bbbox.min().y();
                    let z = k as f64 * bbbox.max().z() + (1 - k) as f64 * bbbox.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    //eprintln!("x:{},newx:{}",x,newx);
                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        //eprintln!("min[c]:{} tester[c]:{}",max[c],tester[c]);
                        min[c] = fmin(min[c], tester[c]);
                        max[c] = fmax(max[c], tester[c]);
                    }
                    //eprintln!("min here:{} {} {}",min[0],min[1],min[2]);
                }
            }
        }
        let ss = sin_theta;
        let cc = cos_theta;
        let has = hasbox;
        //*bbbox = AABB::new(min,max);
        //???
        //eprintln!("{:#?}", min);
        //eprintln!("{:#?}", max);
        RotateY {
            ptr: unwrap_object(p),
            sin_theta: ss,
            cos_theta: cc,
            hasbox: has,
            bbox: AABB::new(min, max),
        }
    }

    pub fn copy(&self) -> RotateY {
        RotateY {
            ptr: unwrap_object(&self.ptr),
            sin_theta: self.sin_theta,
            cos_theta: self.cos_theta,
            hasbox: self.hasbox,
            bbox: self.bbox.copy(),
        }
    }
}

impl Hit for RotateY {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin().copy();
        let mut direction = r.direction().copy();
        //eprintln!("the ini origin[0] is {}",origin[0]);
        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];
        //eprintln!("the post origin[0] is {}",origin[0]);
        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin.copy(), direction.copy(), r.time());

        if !unwrap_object_hit(&self.ptr, rotated_r.copy(), t_min, t_max, rec) {
            return false;
        }
        //fuck you!

        //fuck youuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu!
        //eprintln!("hit here!");
        let mut p = rec.p.copy();
        let mut normal = rec.normal.copy();

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p.copy();
        //FUCK here !
        rec.set_face_normal(rotated_r.copy(), normal);
        true
    }
}

impl BoundingBox for RotateY {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.copy();
        self.hasbox
    }
    fn bounding_box_for_sort(&self, time0: f64, time1: f64, output_boxx: &mut AABB) -> AABB {
        self.bounding_box(time0, time1, output_boxx);
        output_boxx.copy()
    }
}
