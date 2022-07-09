use crate::mod_vec3::Dot;
use crate::mod_vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Hit_record;
//use std::ops::Mul;
use crate::random_double;
//trait也需要声明
type Color = Vec3;

pub enum Material {
    None,
    Lam(Lambertian),
    Met(Metal),
    Diel(Dielectric),
}
/*
impl Material {
    pub fn copy(&self) -> Material {//对一个类型 ，类似的定义
        match self {
            None => Material {Material::None},
            Lam(some) => Material {Material::Lam(some)},
            Met(some) => Material {Material::Met(some)},
        }
    }
}
*/

pub trait Scatter {
    fn scatter(
        &self,
        r: &Ray,
        rec: &Hit_record,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
pub trait Reflect {
    fn reflect(v: Vec3, n: Vec3) -> Vec3;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn lambertian() -> Lambertian {
        Lambertian {
            albedo: Color::vec3(),
        }
    }
    pub fn new(al: Color) -> Lambertian {
        Lambertian { albedo: al }
    }
    pub fn copy(&self) -> Lambertian {
        Lambertian {
            albedo: self.albedo.copy(),
        } //额!!!!
    }
}
impl Scatter for Lambertian {
    //针对某个物体自身的散射
    fn scatter(
        &self,
        r: &Ray,
        rec: &Hit_record,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.copy().normal + Vec3::random_unit_vector(); // & mut 是一个可改引用，可改值的引用 ！！！

        if scatter_direction.near_zero() {
            scatter_direction = rec.copy().normal;
        }
        *scattered = Ray::new(rec.copy().p, scatter_direction);
        *attenuation = self.albedo.copy();
        return true;
    }
}
impl Reflect for Lambertian {
    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v.copy() - 2.0 * v.dot(n.copy()) * n.copy();
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn metal() -> Metal {
        Metal {
            albedo: Color::vec3(),
            fuzz: 0.0,
        }
    }
    pub fn new(al: Color, fu: f64) -> Metal {
        let f: f64;

        if fu > 1.0 {
            f = 1.0;
        } else {
            f = fu;
        }
        //eprintln!("f: {}", f);
        Metal {
            albedo: al,
            fuzz: f,
        }
    }
    pub fn copy(&self) -> Metal {
        Metal {
            albedo: self.albedo.copy(),
            fuzz: self.fuzz,
        }
    }
}
impl Reflect for Metal {
    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v.copy() - 2.0 * v.dot(n.copy()) * n.copy();
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hit_record,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Metal::reflect(
            Vec3::unit_vector(&r_in.copy().direction()),
            rec.copy().normal,
        );
        *scattered = Ray::new(
            rec.copy().p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        ); //???
        if Vec3::random_in_unit_sphere().length_squared() >= 1.0 {
            eprintln!("error!");
        }
        //这里共享的也是mod_vec3里面的Mul
        //那个算一个重载的？？
        //假如这里 f64 * f64 还是需要 use std::....
        //会冲突吗？
        //不会冲突，可以想想原理
        *attenuation = self.albedo.copy();
        return scattered.direction().dot(rec.copy().normal) > 0.0;
    }
}
pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    }
    return b;
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = fmin(-1.0 * uv.copy().dot(n.copy()), 1.0);
    let r_out_perp = etai_over_etat * (uv.copy() + cos_theta * n.copy());
    let r_out_parallel = -(Vec3::abs(1.0 - r_out_perp.length_squared())).sqrt() * n.copy();
    //eprintln!("test ratio: {}", etai_over_etat);
    //eprintln!("dot :{}", r_out_perp.dot(r_out_parallel.copy()));
    return r_out_perp + r_out_parallel;
}

pub struct Dielectric {
    pub ir: f64,
}
impl Dielectric {
    pub fn dielectric() -> Dielectric {
        Dielectric { ir: 0.0 }
    }
    pub fn new(irr: f64) -> Dielectric {
        Dielectric { ir: irr }
    }
    pub fn copy(&self) -> Dielectric {
        Dielectric::new(self.ir)
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        let mut tmp = 1.0 - cosine;
        for i in 0..4 {
            tmp = tmp * (1.0 - cosine); //???
        }
        return r0 + (1.0 - r0) * tmp;
    } //??
}
impl Scatter for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hit_record,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        }
        let unit_direction = Vec3::unit_vector(&r_in.direction());

        let cos_theta = fmin(-1.0 * unit_direction.dot(rec.copy().normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut direction: Vec3;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            direction = Dielectric::reflect(unit_direction.copy(), rec.copy().normal);
        } else {
            direction = refract(
                unit_direction.copy(),
                rec.copy().normal, //????
                refraction_ratio,
            );
        }
        //set_face normal那里可能有错误
        *scattered = Ray::new(rec.copy().p, direction.copy());
        return true;
    }
}
impl Reflect for Dielectric {
    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v.copy() - 2.0 * v.dot(n.copy()) * n.copy();
    }
}
