//怎么将这个转到主文件夹下
//不加那层套皮？？

use crate::mod_vec3::Vec3;
pub type Point3 = Vec3;
pub type Dir3 = Vec3;
//use crate::mod_vec3::Dot;

pub struct Ray {
    pub orig: Point3,
    pub dir: Dir3,
    pub tm: f64,
}

impl Ray {
    pub fn ray() -> Ray {
        Ray {
            orig: Vec3::vec3(),
            dir: Vec3::vec3(),
            tm: 0.0,
        }
    }

    pub fn new(origin: Point3, direction: Dir3, time: f64) -> Ray {
        //类名首字母大写
        Ray {
            orig: origin.copy(),
            dir: direction.copy(),
            tm: time,
        }
    }

    pub fn copy(&self) -> Ray {
        //注意
        Ray::new(self.orig.copy(), self.dir.copy(), self.tm)
    }

    pub fn origin(&self) -> Point3 {
        self.orig.copy()
    }

    pub fn direction(&self) -> Dir3 {
        self.dir.copy()
    } //,??
    pub fn time(&self) -> f64 {
        self.tm
    }
    pub fn at(&self, t: f64) -> Point3 {
        //t时刻光线位置
        //光线
        self.orig.copy() + t * self.dir.copy()
    }
}
