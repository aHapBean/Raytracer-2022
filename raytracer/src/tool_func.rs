use crate::material::*;
use crate::mod_vec3::Vec3;
use crate::ray::*;
use crate::sphere::*;
use crate::texture::Texture;
use crate::texture::Value;

use crate::aabb::*;

type Color = Vec3;
type Point3 = Vec3;

//这个!可能错吗？？？
pub fn unwrap_object_bounding_box(ob: &Object, time0: f64, time1: f64, bo: &mut AABB) -> bool {
    let mut ok = false;
    match ob {
        Object::None => eprintln!("bvh_node constructor false"),
        Object::Sp(t) => ok = !t.bounding_box(time0, time1, bo),
        Object::Msp(t) => ok = !t.bounding_box(time0, time1, bo),
        Object::BV(t) => ok = !t.bounding_box(time0, time1, bo),
        Object::XY(t) => ok = !t.bounding_box(time0, time1, bo),
    };
    ok
}
pub fn unwrap_object_hit(ob: &Object, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let mut ok = false;
    match ob {
        Object::None => eprintln!("bvh_node hit false"),
        Object::Sp(t) => ok = t.hit(r.copy(), t_min, t_max, rec),
        Object::Msp(t) => ok = t.hit(r.copy(), t_min, t_max, rec),
        Object::BV(t) => ok = t.hit(r.copy(), t_min, t_max, rec),
        Object::XY(t) => ok = t.hit(r.copy(), t_min, t_max, rec),
    }
    ok
}

pub fn unwrap_material_scatter(
    mat: &Material,
    r: &Ray,
    rec: &HitRecord,
    attenuation: &mut Color,
    scattered: &mut Ray,
) -> bool {
    let mut ok = false;
    match mat {
        Material::None => eprintln!("None occured ! in tool func"),
        Material::Lam(material) => ok = material.scatter(r, &rec, attenuation, scattered),
        Material::Met(material) => ok = material.scatter(r, &rec, attenuation, scattered),
        Material::Diel(material) => ok = material.scatter(r, &rec, attenuation, scattered),
        Material::Dif(material) => ok = material.scatter(r, &rec, attenuation, scattered),
    };
    ok
}

pub fn unwrap_material_emitted(mat: &Material, u: f64, v: f64, p: Point3) -> Color {
    let mut res = Color::new(0.0, 0.0, 0.0);
    match mat {
        Material::None => eprintln!("None occured ! in tool func"),
        Material::Lam(material) => res = material.emitted(u, v, p.copy()),
        Material::Met(material) => res = material.emitted(u, v, p.copy()),
        Material::Diel(material) => res = material.emitted(u, v, p.copy()),
        Material::Dif(material) => res = material.emitted(u, v, p.copy()),
    };
    res
}

pub fn unwrap_object(ob: &Object) -> Object {
    match ob {
        Object::None => Object::None,
        Object::Sp(t) => Object::Sp(t.copy()),
        Object::Msp(t) => Object::Msp(t.copy()),
        Object::BV(t) => Object::BV(Box::new(t.copy())), //tag!
        Object::XY(t) => Object::XY(t.copy()),
    }
}
//TAPL
pub fn unwrap_material(mat: &Material) -> Material {
    match mat {
        Material::None => Material::None,
        Material::Lam(t) => Material::Lam(t.copy()),
        Material::Met(t) => Material::Met(t.copy()),
        Material::Diel(t) => Material::Diel(t.copy()),
        Material::Dif(t) => Material::Dif(t.copy()),
    }
}

pub fn unwrap_texture(a: &Texture) -> Texture {
    let res = match a {
        Texture::So(t) => Texture::So(t.copy()),
        Texture::Ch(t) => Texture::Ch(Box::new(t.copy())),
        Texture::No(t) => Texture::No(t.copy()),
        Texture::Im(t) => Texture::Im(t.copy()),
    }; //这个地方copy 可能对性能影响较大，看看能否优化
       //data !!!!!!!!!
    res
}

pub fn unwrap_texture_color(a: &Texture, u: f64, v: f64, p: &Point3) -> Color {
    //may be recursive
    let res = match a {
        Texture::So(t) => t.value(u, v, p),
        Texture::Ch(t) => t.value(u, v, p),
        Texture::No(t) => t.value(u, v, p),
        Texture::Im(t) => t.value(u, v, p),
    };
    res
}

pub fn copy_vec<T: Copy>(a: &Vec<T>) -> Vec<T> {
    //here ???
    let mut res: Vec<T> = vec![];
    let len = a.len();
    for i in 0..len {
        res.push(a[i]);
    }
    res
}
pub fn copy_vec_for_vec3(a: &Vec<Vec3>) -> Vec<Vec3> {
    let mut res: Vec<Vec3> = vec![];
    let len = a.len();
    for i in 0..len {
        res.push(a[i].copy());
    }
    res
}

pub fn floor(a: f64) -> f64 {
    let mut b = a as i32;
    if b as f64 > a {
        b -= 1;
    }
    b as f64
}
