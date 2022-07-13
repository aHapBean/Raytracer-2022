use crate::aabb::AABB;
use crate::material::*;
use crate::mod_vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::*;
use crate::tool_func::*;

type Point3 = Vec3;
pub struct XYrect {
    mp: Material,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYrect {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: &Material) -> XYrect {
        XYrect {
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
            mp: unwrap_material(&mat),
        }
    }
    pub fn copy(&self) -> XYrect {
        XYrect {
            x0: self.x0,
            x1: self.x1,
            y0: self.y0,
            y1: self.y1,
            k: self.k,
            mp: unwrap_material(&self.mp),
        }
    }
}
impl Hit for XYrect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().copy().z) / r.direction().copy().z;

        if t < t_min || t > t_max {
            return false;
        }

        let x = r.copy().origin().x + t * r.copy().direction().x;
        let y = r.copy().origin().y + t * r.copy().direction().y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r.copy(), outward_normal.copy());
        rec.mat_ptr = unwrap_material(&self.mp);
        rec.p = r.at(t);
        true
    }
}

impl BoundingBox for XYrect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
    fn bounding_box_for_sort(&self, time0: f64, time1: f64, output_box: &mut AABB) -> AABB {
        self.bounding_box(time0, time1, output_box);
        output_box.copy()
    }
}

pub struct XZrect {
    mp: Material,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}
impl XZrect {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: &Material) -> XZrect {
        XZrect {
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: unwrap_material(&mat),
        }
    }
    pub fn copy(&self) -> XZrect {
        XZrect {
            x0: self.x0,
            x1: self.x1,
            z0: self.z0,
            z1: self.z1,
            k: self.k,
            mp: unwrap_material(&self.mp),
        }
    }
}

impl Hit for XZrect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().copy().y) / r.direction().copy().y;

        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().copy().x + t * r.direction().copy().x;
        let z = r.origin().copy().z + t * r.direction().copy().z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        };
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);

        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r.copy(), outward_normal.copy());
        rec.mat_ptr = unwrap_material((&self.mp));
        rec.p = r.at(t);
        true
    }
}
impl BoundingBox for XZrect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.z0, self.k - 0.0001),
            Point3::new(self.x1, self.z1, self.k + 0.0001),
        );
        true
    }
    fn bounding_box_for_sort(&self, time0: f64, time1: f64, output_box: &mut AABB) -> AABB {
        self.bounding_box(time0, time1, output_box);
        output_box.copy()
    }
}
pub struct YZrect {
    mp: Material,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}
impl YZrect {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: &Material) -> YZrect {
        YZrect {
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: unwrap_material(&mat),
        }
    }
    pub fn copy(&self) -> YZrect {
        YZrect {
            y0: self.y0,
            y1: self.y1,
            z0: self.z0,
            z1: self.z1,
            k: self.k,
            mp: unwrap_material(&self.mp),
        }
    }
}

impl Hit for YZrect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().copy().x) / r.direction().copy().x;

        if t < t_min || t > t_max {
            return false;
        }
        let y = r.origin().copy().y + t * r.direction().copy().y;
        let z = r.origin().copy().z + t * r.direction().copy().z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        };
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);

        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r.copy(), outward_normal.copy());
        rec.mat_ptr = unwrap_material((&self.mp));
        rec.p = r.at(t);
        true
    }
}
impl BoundingBox for YZrect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.y0, self.z0, self.k - 0.0001),
            Point3::new(self.y1, self.z1, self.k + 0.0001),
        );
        true
    }
    fn bounding_box_for_sort(&self, time0: f64, time1: f64, output_box: &mut AABB) -> AABB {
        self.bounding_box(time0, time1, output_box);
        output_box.copy()
    }
}
