//mod Vec3:
use crate::random_double;
use crate::random_double_range;
use ops::Div;
use ops::Mul;
use std::ops;
use std::ops::Add;
use std::ops::Sub;

pub trait Dot<T> {
    //除了运算符重载，其他用引用！
    fn dot(&self, t: T) -> f64;
} //why no pug ???
pub trait Cross<T> {
    fn cross(&self, t: T) -> T;
}

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn vec3() -> Vec3 {
        Vec3 {
            x: 0 as f64,
            y: 0 as f64,
            z: 0 as f64,
        }
    }
    pub fn new(a: f64, b: f64, c: f64) -> Vec3 {
        Vec3 { x: a, y: b, z: c }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        (Vec3::length_squared(&self)).sqrt() //??
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(&self) -> Vec3 {
        //指明作用域
        let len = Vec3::length(&self);
        Vec3::new(self.x / len, self.y / len, self.z / len)
    }
    pub fn copy(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }
    //这个sphere不是球
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.copy().length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Vec3 {
        //to delete
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if normal.copy().dot(in_unit_sphere.copy()) > 0.0 {
            in_unit_sphere
        } else {
            -1.0 * in_unit_sphere
        }
    }
    pub fn abs(s: f64) -> f64 {
        if s < 0.0 {
            -s
        } else {
            s
        }
    }
    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        Vec3::abs(self.x) < s && Vec3::abs(self.y) < s && Vec3::abs(self.z) < s
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, t: Vec3) -> Vec3 {
        Vec3::new(self.x * t.x, self.y * t.y, self.z * t.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, t: Vec3) -> Vec3 {
        Vec3::new(self * t.x, self * t.y, self * t.z)
    } //妙啊，后面的才是实例化的？？
}

impl Dot<Vec3> for Vec3 {
    fn dot(&self, t: Vec3) -> f64 {
        self.x * t.x + self.y * t.y + self.z * t.z
    }
}

impl Cross<Vec3> for Vec3 {
    fn cross(&self, t: Vec3) -> Vec3 {
        Vec3::new(
            self.y * t.z - self.z * t.y,
            self.z * t.x - self.x * t.z,
            self.x * t.y - self.y * t.x,
        )
    }
}
/*
impl Copy<Vec3> for Vec3 {
    fn copy()
}
*/

/*
impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x / t,self.y / t,self.z / t)
    }
}
*/
