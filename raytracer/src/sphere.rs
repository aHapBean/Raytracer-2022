use crate::mod_vec3::Vec3;
use crate::ray::Ray;
//trait也需要
use crate::material::Material;
use crate::mod_vec3::Dot;
type Point3 = Vec3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Material,
}

impl HitRecord {
    pub fn hitrecord() -> HitRecord {
        HitRecord {
            p: Point3::vec3(),
            normal: Vec3::vec3(),
            t: 0.0,
            front_face: false,
            mat_ptr: Material::None,
        }
    } //point & normal ??
    pub fn new(pp: Point3, nor: Vec3, tt: f64, ff: bool, mat: Material) -> HitRecord {
        HitRecord {
            p: pp,
            normal: nor,
            t: tt,
            front_face: ff,
            mat_ptr: mat,
        }
    }
    pub fn copy(&self) -> HitRecord {
        //引用不能把。。因为这个相当于borrow
        let mat_ptr = match &self.mat_ptr {
            Material::None => Material::None,
            Material::Lam(some) => Material::Lam(some.copy()),
            Material::Met(some) => Material::Met(some.copy()),
            Material::Diel(some) => Material::Diel(some.copy()),
        };
        //这里可以吗，直接反正不行
        HitRecord::new(
            self.p.copy(),
            self.normal.copy(),
            self.t,
            self.front_face,
            mat_ptr,
        ) //????
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        if outward_normal.copy().dot(r.direction()) < 0.0 {
            self.front_face = true;
        } else {
            self.front_face = false;
        }

        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -1.0 * outward_normal;
        }
    }
}

pub trait Hit {
    //“基类”
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Material,
}

impl Sphere {
    //pub fn sphere() -> Sphere {
    //    Sphere {
    //        center: Point3::vec3(),
    //        radius: 0.0,
    //        mat_ptr: Material::None,
    //    }
    //}
    pub fn new(c: Point3, rad: f64, mat: Material) -> Sphere {
        Sphere {
            center: c,
            radius: rad,
            mat_ptr: mat,
        }
    }
    pub fn copy(&self) -> Sphere {
        let mat_ptr = match &self.mat_ptr {
            Material::None => Material::None,
            Material::Lam(some) => Material::Lam(some.copy()),
            Material::Met(some) => Material::Met(some.copy()),
            Material::Diel(some) => Material::Diel(some.copy()), //f64 !
        };
        Sphere::new(self.center.copy(), self.radius, mat_ptr)
    }
}

impl Hit for Sphere {
    //mut !!!!
    //                           引用
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // the pub is implied ??
        let oc: Vec3 = r.origin() - self.center.copy();
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
        rec.normal = (rec.p.copy() - self.center.copy()) / self.radius;
        rec.mat_ptr = self.copy().mat_ptr; //???
                                           //?????!!!!!

        let outward_normal = (rec.p.copy() - self.center.copy()) / self.radius;
        rec.set_face_normal(r.copy(), outward_normal.copy());

        //使用方法时不要加copy 不然方法可能无法更改到你自身
        true
    }
}
pub enum Object {
    None,
    Sp(Sphere),
}

pub struct HittableList {
    objects: Vec<Object>,
}

impl HittableList {
    pub fn hittablelist() -> HittableList {
        let mut world = HittableList { objects: vec![] };
        world.objects.push(Object::None);
        world.objects.pop();
        world
    }
    //pub fn new() -> HittableList {
    //    HittableList {
    //        objects: Vec::new(), //????
    //    }
    //}

    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, ob: Object) {
        //必须指明加入类型
        match ob {
            Object::None => self.objects.push(Object::None),
            Object::Sp(t) => self.objects.push(Object::Sp(t)), //here
        }; //神奇的用法
    }
}

//
impl Hit for HittableList {
    //某点的重叠光影
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let temp_rec: &mut HitRecord = &mut HitRecord::hitrecord();
        //???
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            match object {
                Object::Sp(object) => {
                    if object.copy().hit(r.copy(), t_min, closest_so_far, temp_rec) {
                        //r.copy()!!!
                        hit_anything = true;
                        closest_so_far = temp_rec.copy().t;
                        *rec = temp_rec.copy(); //????
                                                //*rec = temp_rec.copy();
                    }
                }
                Object::None => (),
            }
        }
        hit_anything
    }
}
