#![allow(warnings, unused)]

use std::fs::File;
use std::process::exit;

use bvh::BVH_node;
use fog::ConstantMedium;
use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

mod aabb;
mod boxx;
mod bvh;
mod camera;
mod fog;
mod material;
mod mod_vec3;
mod moving_sphere;
mod ray;
mod rect;
mod sphere;
mod texture;
mod tool_func;
mod translate;
//模块声明

use moving_sphere::MovingSphere;
//use crate::mod_vec3::Dot;
use mod_vec3::Vec3;

use crate::sphere::Hit;
use crate::sphere::HittableList;
use crate::sphere::Object;

use ray::Ray; //类名就首字母大写

use sphere::HitRecord;
use sphere::Sphere; //trait 也要 use !

use camera::Camera;

use crate::material::Material;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal, Scatter};
use crate::texture::{Noise_texture, Texture};
use core::f32::consts::PI;
use rand::Rng;

use crate::texture::Checker_texture;
use crate::texture::ImageTexture;

use crate::rect::{XYrect, XZrect, YZrect};
use crate::texture::SolidColor;

use crate::boxx::*;

use crate::tool_func::*;
use crate::translate::*;

use indicatif::MultiProgress;
pub use std::{
    //fs::File,
    //process::exit,
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};

type Color = Vec3;
type Point3 = Vec3;
fn abs(a: f64) -> f64 {
    if a < 0.0 {
        return -a;
    } else {
        return a;
    }
}
fn fmax(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    }
    b
}
//第二个！
fn fmin(a: f64, b: f64) -> f64 {
    if a > b {
        return b;
    }
    a
}
fn degree_to_radians(degrees: f64) -> f64 {
    //let pi = 3.141_592_653_589_793; //;;_238_5;
    degrees * PI as f64 / 180.0
}
fn random_double() -> f64 {
    //rand::rng.gen::<f64>()
    let mut a = rand::thread_rng();
    a.gen_range(0.0..=1.0)
}
fn random_double_range(a: f64, b: f64) -> f64 {
    random_double() * (b - a) + a
}
fn random_int_range(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, (max + 1) as f64) as i32
}
fn random_in_unit_dist() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double_range(-1.0, 1.0),
            random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
/*fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let tmp = 255.999;
    let mut r = pixel_color.copy().x;
    let mut g = pixel_color.copy().y;
    let mut b = pixel_color.copy().z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    println!(
        "{} {} {}",
        256.0 * clamp(r, 0.0, 0.999),
        256.0 * clamp(g, 0.0, 0.999),
        256.0 * clamp(b, 0.0, 0.999),
    );
}*/
/*fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let infinity = 1.79769e+308;

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::hitrecord();

    if world.hit(r.copy(), 0.001, infinity, &mut rec) {
        //let target: Point3 = rec.copy().p + rec.copy().normal + Vec3::random_in_hemisphere(rec.normal.copy());
        let mut scattered: Ray = Ray::ray();
        let mut attenuation: Color = Color::vec3(); //衰减
        let material = rec.copy().mat_ptr; //????
        let mut ok = false;
        ok = unwrap_material_scatter(&material, r, &rec, &mut attenuation, &mut scattered);
        //match material {
        //    Material::None => eprintln!("None occured !"), //None设置有问题
        //    //他这个指向派生类的指针这里都可以不用加，和C++框架不同！
        //    Material::Lam(material) => {
        //        ok = material.scatter(r, &rec, &mut attenuation, &mut scattered)
        //    }
        //    Material::Met(material) => {
        //        ok = material.scatter(r, &rec, &mut attenuation, &mut scattered)
        //    }
        //    Material::Diel(material) => {
        //        ok = material.scatter(r, &rec, &mut attenuation, &mut scattered)
        //    }
        //};
        if ok {
            return attenuation.copy() * ray_color(&scattered, &world, depth - 1);
        } //?????unreachable
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    //背景色
} //作用 :配置光线的颜色
  //根据光线的信息，两个信息！起点和方向！
  //妙
  */

fn ray_color(r: &Ray, background: Color, world: &HittableList, depth: i32) -> Color {
    let infinity = 1.79769e+308;

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::hitrecord();

    if !world.hit(r.copy(), 0.001, infinity, &mut rec) {
        return background;
    }

    let mut scattered: Ray = Ray::ray();
    let mut attenuation: Color = Color::vec3();
    let material = rec.copy().mat_ptr;

    let emitted = unwrap_material_emitted(&rec.mat_ptr, rec.u, rec.v, rec.p.copy());
    let mut ok = false;
    ok = unwrap_material_scatter(&material, r, &rec, &mut attenuation, &mut scattered);

    if !ok {
        //let unit_direction = r.direction().unit_vector();
        //let t = 0.5 * (unit_direction.y() + 1.0);
        //return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        return emitted;
    };
    return emitted
        + attenuation.copy() * ray_color(&scattered, background.copy(), world, depth - 1);
}

fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::hittablelist();

    //let checker =
    //    Checker_texture::checker_texture(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let pretext = Noise_texture::noise_texture(4.0);
    let ball = Object::Sp(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lam(Lambertian::new(&Texture::No(pretext.copy()))),
    ));

    world.add(ball);
    let ball = Object::Sp(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Material::Lam(Lambertian::new(&Texture::No(pretext.copy()))),
        //Material::Lam(Lambertian::new(&Texture::Ch(Box::new(checker.copy())))),
    ));
    world.add(ball);
    world
}
fn two_spheres() -> HittableList {
    let mut world = HittableList::hittablelist();

    let checker =
        Checker_texture::checker_texture(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ball = Object::Sp(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Material::Lam(Lambertian::new(&Texture::Ch(Box::new(checker.copy())))),
    ));
    world.add(ball);
    let ball = Object::Sp(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Material::Lam(Lambertian::new(&Texture::Ch(Box::new(checker.copy())))),
    ));
    world.add(ball);
    world
}
fn random_scene() -> HittableList {
    let mut world = HittableList::hittablelist();

    //let ground_material = Material::Lam(Lambertian::new(&Texture::So(Solid_color::new(Color::new(0.5, 0.5, 0.5)))));
    let checker =
        Checker_texture::checker_texture(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Material::Lam(Lambertian::new(&Texture::Ch(Box::new(checker))));
    let ball = Object::Sp(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));
    world.add(ball);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center.copy() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    //此时albedo是一个颜色，而改变后真实的应该是一种纹理
                    let sphere_material =
                        Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(albedo))));
                    let center2 =
                        center.copy() + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0); //tags!
                    world.add(Object::Msp(MovingSphere::new(
                        center.copy(),
                        center2.copy(),
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Material::Met(Metal::new(albedo, fuzz));
                    world.add(Object::Sp(Sphere::new(center.copy(), 0.2, sphere_material)));
                } else {
                    let sphere_material = Material::Diel(Dielectric::new(1.5));
                    world.add(Object::Sp(Sphere::new(center.copy(), 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Material::Diel(Dielectric::new(1.5));
    world.add(Object::Sp(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.4, 0.2, 0.1,
    )))));
    world.add(Object::Sp(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Met(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Object::Sp(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}
fn earth() -> HittableList {
    //这个texture的所有权按道理在构造函数里面的可以直接给出，写复杂了一些些
    let mut world: HittableList = HittableList::hittablelist();
    //eprintln!("here 1");
    let earth_texture = Texture::Im(ImageTexture::new("image/earthmap.jpg".to_string()));
    let earth_surface = Material::Lam(Lambertian::new(&earth_texture));
    let globe = Object::Sp(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    world.add(globe);
    world
}
fn simple_light() -> HittableList {
    let mut world: HittableList = HittableList::hittablelist();

    let pertext = Noise_texture::noise_texture(4.0);
    world.add(Object::Sp(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lam(Lambertian::new(&Texture::No(pertext.copy()))),
    )));
    world.add(Object::Sp(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Material::Lam(Lambertian::new(&Texture::No(pertext.copy()))),
    )));

    let difflight = Material::Dif(DiffuseLight::new(Color::new(6.0, 6.0, 6.0)));
    world.add(Object::XY(XYrect::new(
        3.0, 5.0, 1.0, 3.0, -2.0, &difflight,
    )));

    let difflight = Material::Dif(DiffuseLight::new(Color::new(6.0, 6.0, 6.0)));
    world.add(Object::Sp(Sphere::new(
        Point3::new(-3.0, 1.5, 1.0),
        1.5,
        difflight,
    )));

    world
}
fn cornell_box() -> HittableList {
    let mut world = HittableList::hittablelist();
    let red = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.12, 0.45, 0.15,
    )))));

    let light = Material::Dif(DiffuseLight::new(Color::new(30.0, 30.0, 30.0)));

    world.add(Object::YZ(YZrect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, &green,
    )));
    world.add(Object::YZ(YZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red)));
    world.add(Object::XZ(XZrect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, &light,
    )));
    world.add(Object::XZ(XZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white)));
    world.add(Object::XZ(XZrect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, &white,
    )));
    world.add(Object::XY(XYrect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, &white,
    )));

    let box1 = Object::Bo(Box::new(Boxx::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        &white,
    )));
    let box2 = Object::Ro(Box::new(RotateY::new(&box1, 15.0)));
    //let box2 = Objec::
    let box3 = Object::Tr(Box::new(Translate::new(
        &box2,
        Vec3::new(265.0, 0.0, 295.0),
    )));

    world.add(box3);

    //world.add(Object::Bo(Box::new(Boxx::new(
    //    Point3::new(130.0, 0.0, 65.0),
    //    Point3::new(295.0, 165.0, 230.0),
    //    &white,
    //))));

    let box2 = Object::Bo(Box::new(Boxx::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        &white,
    )));
    let box2 = Object::Ro(Box::new(RotateY::new(&box2, -18.0)));
    let box2 = Object::Tr(Box::new(Translate::new(&box2, Vec3::new(130.0, 0.0, 65.0))));
    world.add(box2);
    //world.add(Object::Bo(Box::new(Boxx::new(
    //    Point3::new(265.0, 0.0, 295.0),
    //    Point3::new(430.0, 330.0, 460.0),
    //    &white,
    //))));

    world
}
fn cornell_smoke() -> HittableList {
    let mut world: HittableList = HittableList::hittablelist();
    let red = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.12, 0.45, 0.15,
    )))));

    let light = Material::Dif(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    world.add(Object::YZ(YZrect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, &green,
    )));
    world.add(Object::YZ(YZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red)));
    world.add(Object::XZ(XZrect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, &light,
    )));
    world.add(Object::XZ(XZrect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, &white,
    )));
    world.add(Object::XZ(XZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white)));
    world.add(Object::XY(XYrect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, &white,
    )));
    let box1 = Object::Bo(Box::new(Boxx::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        &white,
    )));
    let box1 = Object::Ro(Box::new(RotateY::new(&box1, 45.0)));
    //let box1 = Objec::
    let box1 = Object::Tr(Box::new(Translate::new(
        &box1,
        Vec3::new(265.0, 0.0, 295.0),
    )));

    world.add(Object::Co(Box::new(ConstantMedium::new_by_color(
        &box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    ))));

    let box2 = Object::Bo(Box::new(Boxx::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        &white,
    )));
    let box2 = Object::Ro(Box::new(RotateY::new(&box2, -30.0)));
    let box2 = Object::Tr(Box::new(Translate::new(&box2, Vec3::new(130.0, 0.0, 65.0))));
    //world.add(box2);
    world.add(Object::Co(Box::new(ConstantMedium::new_by_color(
        &box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    ))));

    world
}
fn final_scene() -> HittableList {
    let mut boxesl = HittableList::hittablelist();
    let ground = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.48, 0.83, 0.53,
    )))));
    let mut objects = HittableList::hittablelist();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64 * w) as f64;
            let z0 = -1000.0 + (j as f64 * w) as f64;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            objects.add(Object::Bo(Box::new(Boxx::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                &ground,
            ))));
            //boxesl.add(Object::Bo(Box::new(Boxx::new(
            //    Point3::new(x0, y0, z0),
            //    Point3::new(x1, y1, z1),
            //    &ground,
            //))));
        }
    }

    //let mut objects = HittableList::hittablelist();
    //objects.add(Object::BV(Box::new(BVH_node::new_by_three(
    //    &mut boxesl,
    //    0.0,
    //    1.0,
    //))));

    let light = Material::Dif(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    objects.add(Object::XZ(XZrect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, &light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = Point3::new(30.0, 0.0, 0.0) + center1.copy();

    let moving_sphere_mat = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(
        Color::new(0.7, 0.3, 0.1),
    ))));
    objects.add(Object::Msp(MovingSphere::new(
        center1.copy(),
        center2.copy(),
        0.0,
        1.0,
        50.0,
        moving_sphere_mat,
    )));

    objects.add(Object::Sp(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Material::Diel(Dielectric::new(1.5)),
    )));

    objects.add(Object::Sp(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Material::Met(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Object::Sp(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Material::Diel(Dielectric::new(1.5)),
    ));
    objects.add(unwrap_object(&boundary));

    objects.add(Object::Co(Box::new(ConstantMedium::new_by_color(
        &boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ))));
    boundary = Object::Sp(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Material::Diel(Dielectric::new(1.5)),
    ));
    objects.add(Object::Co(Box::new(ConstantMedium::new_by_color(
        &boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    ))));

    //地球
    //let emat = Material::Lam(Lambertian::new(&Texture::Im(ImageTexture::new(
    //    "image/earthmap.jpg".to_string(),
    //))));
    //objects.add(Object::Sp(Sphere::new(
    //    Point3::new(400.0, 200.0, 400.0),
    //    100.0,
    //    emat,
    //)));
    let emat = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        1.0, 1.0, 1.0,
    )))));
    objects.add(Object::Sp(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let pertext = Texture::No(Noise_texture::noise_texture(0.1));
    objects.add(Object::Sp(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Material::Lam(Lambertian::new(&pertext)),
    )));

    let mut boxes2 = HittableList::hittablelist();
    let white = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.73, 0.73, 0.73,
    )))));

    let ns: i32 = 500;
    for j in 0..ns as u32 {
        let white = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
            0.73, 0.73, 0.73,
        )))));
        //boxes2.add(Object::Sp(Sphere::new(
        //    Point3::random_range(0.0, 165.0),
        //    10.0,
        //    white,
        //)));
        objects.add(Object::Sp(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white,
        )));
    }

    //objects.add(Object::Tr(Box::new(Translate::new(
    //    &Object::Ro(Box::new(RotateY::new(
    //        &Object::BV(Box::new(BVH_node::new_by_three(&mut boxes2, 0.0, 1.0))),
    //        15.0,
    //    ))),
    //    Vec3::new(-100.0, 270.0, 395.0),
    //))));

    objects
}
fn my_scene() -> HittableList {
    let mut world: HittableList = HittableList::hittablelist();

    let red = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.999, 0.999, 0.999,
    )))));
    let green = Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
        0.12, 0.45, 0.15,
    )))));

    let light = Material::Dif(DiffuseLight::new(Color::new(10.0, 10.0, 10.0)));

    world.add(Object::Sp(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        Material::Lam(Lambertian::new(&Texture::Ch(Box::new(
            Checker_texture::new(
                &Texture::So(SolidColor::new(Color::new(0.73, 0.73, 0.73))),
                &Texture::So(SolidColor::new(Color::new(0.0, 0.0, 0.0))),
            ),
        )))),
    )));

    //边界
    //let boundary = Object::Sp(Sphere::new(
    //    Point3::new(0.0, 0.0, 0.0),
    //    5000.0,
    //    Material::Diel(Dielectric::new(1.5)),
    //));
    //world.add(Object::Co(Box::new(ConstantMedium::new_by_color(
    //    &boundary,
    //    0.0001,
    //    Color::new(1.0, 1.0, 1.0),
    //))));

    //加个大光球,上方的，用来提供光线
    world.add(Object::Sp(Sphere::new(
        Point3::new(-4.5, 20.0, -5.8),
        6.0,
        unwrap_material(&Material::Dif(DiffuseLight::new(Color::new(1.0, 1.0, 1.0)))),
    )));

    //后面的小光球
    world.add(Object::Sp(Sphere::new(
        Point3::new(-3.8, 1.3, -5.8),
        0.35,
        unwrap_material(&Material::Dif(DiffuseLight::new(Color::new(0.0, 1.0, 0.8)))),
    )));
    world.add(Object::XY(XYrect::new(
        -3.8,
        -3.798,
        1.65,
        7.0,
        -5.8,
        &unwrap_material(&Material::Dif(DiffuseLight::new(Color::new(0.0, 1.0, 0.8)))),
    )));

    //绿球
    world.add(Object::Sp(Sphere::new(
        Point3::new(-0.8, 1.0, -1.0),
        1.0,
        unwrap_material(&green),
    )));

    //最后面的蓝色大球
    world.add(Object::Sp(Sphere::new(
        Point3::new(-3.8, 0.8, -3.0),
        0.8,
        Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
            0.0, 0.2, 0.8,
        ))))),
    )));

    //盒子和大球
    world.add(Object::Bo(Box::new(Boxx::new(
        Vec3::new(1.5, 0.0, -1.0),
        Vec3::new(3.5, 2.0, 1.0),
        &white,
    ))));
    world.add(Object::Sp(Sphere::new(
        Point3::new(2.5, 2.8, 0.0),
        0.8,
        red,
    )));

    //盒子右边加一个发光矩形
    world.add(Object::YZ(YZrect::new(
        0.0,
        0.5,
        1.2,
        1.6,
        4.5,
        &unwrap_material(&Material::Dif(DiffuseLight::new(Color::new(1.0, 0.5, 0.0)))),
    )));
    world.add(Object::YZ(YZrect::new(
        0.2,
        0.7,
        0.9,
        1.3,
        -5.4,
        &unwrap_material(&Material::Dif(DiffuseLight::new(Color::new(
            156.0 / 256.0,
            204.0 / 256.0,
            226.0 / 256.0,
        )))),
    )));

    //前后
    world.add(Object::XY(XYrect::new(-0.3, 0.3, 0.0, 0.6, -7.0, &light)));
    world.add(Object::XY(XYrect::new(-0.85, -0.35, 0.0, 0.5, 5.1, &light)));

    //加四个发光的box!!!
    //改成了两个玻璃的盒子
    //let box1 = Object::Bo(Box::new(Boxx::new(
    //    Vec3::new(2.2, 0.0, 3.5),
    //    Vec3::new(2.6, 0.4, 3.505),
    //    &Material::Diel(Dielectric::new(1.5)),
    //)));
    //let box1 = Object::Ro(Box::new(RotateY::new(&box1, 45.0)));
    //let box1 = Object::Tr(Box::new(Translate::new(&box1, Vec3::new(-1.5, 0.0, 2.8))));
    //world.add(box1);

    let box1 = Object::Bo(Box::new(Boxx::new(
        Vec3::new(-3.8, 0.0, -0.7),
        Vec3::new(-3.4, 0.4, -0.705),
        &Material::Diel(Dielectric::new(1.5)),
    )));
    let box1 = Object::Ro(Box::new(RotateY::new(&box1, 45.0)));
    let box1 = Object::Tr(Box::new(Translate::new(&box1, Vec3::new(-2.3, 0.0, -5.4))));
    world.add(box1);

    //动效小球
    world.add(Object::Msp(MovingSphere::new(
        Vec3::new(0.5, 0.3, 0.0),
        Vec3::new(4.5, 0.3, 0.0),
        0.0,
        1.0,
        0.2,
        unwrap_material(&light),
    )));

    //右后的光球
    world.add(Object::Sp(Sphere::new(
        //4.8,0.4,-0.9
        Point3::new(3.9, 0.4, -4.3),
        0.4,
        unwrap_material(&Material::Dif(DiffuseLight::new(Color::new(0.0, 0.0, 1.0)))),
    )));
    //交替花纹盒子
    let box1 = Object::Bo(Box::new(Boxx::new(
        Vec3::new(-4.4, 0.0, 3.4),
        Vec3::new(-3.0, 1.4, 4.8),
        &Material::Lam(Lambertian::new(&Texture::Ch(Box::new(
            Checker_texture::new(
                &Texture::So(SolidColor::new(Color::new(1.0, 0.0, 1.0))),
                &Texture::So(SolidColor::new(Color::new(0.0, 1.0, 0.0))),
            ),
        )))),
    )));
    let box1 = Object::Ro(Box::new(RotateY::new(&box1, 15.0)));
    let box1 = Object::Tr(Box::new(Translate::new(&box1, Vec3::new(5.7, 0.0, 0.0))));
    world.add(box1);

    //前面的光球
    world.add(Object::Sp(Sphere::new(
        Point3::new(3.0, 3.0, 4.0),
        0.35,
        unwrap_material(&light),
    )));
    world.add(Object::XY(XYrect::new(
        3.0,
        3.0006,
        3.35,
        7.0,
        4.0,
        &unwrap_material(&light),
    )));

    //加一个在前面光球略右边的一个玻璃球，悬空的
    //这个材质好像还没显现出来
    world.add(Object::Co(Box::new(ConstantMedium::new(
        &Object::Sp(Sphere::new(
            Vec3::new(0.4, 1.8, 2.0),
            0.3,
            unwrap_material(&light),
        )),
        0.5,
        &Texture::So(SolidColor::new(Color::new(1.0, 1.0, 1.0))),
    ))));

    //左下角的金属球
    world.add(Object::Sp(Sphere::new(
        Point3::new(-2.0, 0.8, 0.8),
        0.8,
        Material::Met(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));
    //metal左前加一个box 或者什么东西
    world.add(Object::Bo(Box::new(Boxx::new(
        Vec3::new(-3.2, 0.0, 2.6),
        Vec3::new(-2.1, 1.1, 3.7),
        &white,
    ))));
    //可以考虑再加一个，两个物品相对着
    //小长条
    let box1 = Object::Bo(Box::new(Boxx::new(
        Vec3::new(-1.8, 0.0, 3.5),
        Vec3::new(-0.5, 0.3, 3.9),
        &green,
    )));
    let box2 = Object::Ro(Box::new(RotateY::new(&box1, 30.0)));
    //let box2 = Objec::
    let box3 = Object::Tr(Box::new(Translate::new(&box2, Vec3::new(-0.2, 0.0, 1.5))));
    world.add(box3);

    //加一个金属球右前的银河大球
    //world.add(Object::Sp(Sphere::new(
    //    Point3::new(, 1.0, 1.7),
    //    1.0,
    //    Material::Lam(Lambertian::new(&Texture::Im(ImageTexture::new(
    //        "image/mikeyway.jpg".to_string(),
    //    )))),
    //)));
    world.add(Object::Sp(Sphere::new(
        Point3::new(0.9, 1.35, 3.1),
        1.35,
        //Material::Lam(Lambertian::new(&Texture::Im(ImageTexture::new(
        //    "image/mikeyway.jpg".to_string(),
        //)))),
        Material::Lam(Lambertian::new(&Texture::So(SolidColor::new(Color::new(
            1.0, 1.0, 1.0,
        ))))),
    )));

    //左后玻璃小球
    world.add(Object::Sp(Sphere::new(
        Point3::new(-4.3, 2.0, 0.5),
        0.5,
        Material::Diel(Dielectric::new(1.5)),
    )));
    world.add(Object::XY(XYrect::new(
        -4.3,
        -4.2998,
        2.5,
        5.0,
        0.5,
        &unwrap_material(&light),
    )));
    //let difflight = Material::Dif(DiffuseLight::new(Color::new(6.0, 6.0, 6.0)));
    //world.add(Object::XY(XYrect::new(
    //    3.0, 5.0, 1.0, 3.0, -2.0, &difflight,
    //)));

    world
}

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1
    println!(
        "\n         {}    {}\n",
        style("zxd's Ray Tracer").cyan(),
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).yellow(),
    );
    println!(
        "{} 💿 {}",
        style("[1/5]").bold().dim(),
        style("Initlizing...").green()
    );
    let begin_time = Instant::now();
    const THREAD_NUMBER: usize = 7;

    //Image
    let mut aspect_ratio: f64 = 16.0 / 9.0;
    let mut image_width: f64 = 400.0;
    let mut image_height: f64 = image_width as f64 / aspect_ratio;

    //world
    let mut world: HittableList;

    let mut lookfrom;
    let mut lookat;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov: f64;
    let dist_to_focus = 10.0;
    let mut aperture = 0.1;
    let background: Color;
    let mut samples_per_pixel = 50;
    let max_depth = 50;

    let flag: i32 = 8;
    if flag == 1 {
        world = random_scene();
        background = Color::new(0.70, 0.80, 1.00);
        lookfrom = Point3::new(13.0, 2.0, 3.0);
        lookat = Point3::new(0.0, 0.0, 0.0);
        vfov = 20.0;
        aperture = 0.1;
    } else if flag == 2 {
        world = two_spheres();
        background = Color::new(0.7, 0.8, 1.00);
        lookfrom = Point3::new(13.0, 2.0, 3.0);
        lookat = Point3::new(0.0, 0.0, 0.0);
        vfov = 20.0;
    } else if flag == 3 {
        world = two_perlin_spheres();
        background = Color::new(0.7, 0.8, 1.00);
        lookfrom = Point3::new(13.0, 2.0, 3.0);
        lookat = Point3::new(0.0, 0.0, 0.0);
        vfov = 20.0;
    } else if flag == 4 {
        world = earth();
        background = Color::new(0.7, 0.8, 1.00);
        lookfrom = Point3::new(13.0, 2.0, 3.0);
        lookat = Point3::new(0.0, 0.0, 0.0);
        vfov = 20.0;
    } else if flag == 5 {
        world = my_scene();
        background = Color::new(0.00005, 0.0005, 0.0005);
        lookfrom = Point3::new(1.9, 7.7, 10.0);
        //lookat = Point3::new(-0.8, 0.8, -1.0);
        lookat = Point3::new(0.13, -0.09, -1.0);
        lookfrom = lookfrom.copy() - lookat.copy() * 10.0;
        samples_per_pixel = 600;
        image_width = 960.0;
        aspect_ratio = 16.0 / 9.0;
        image_height = image_width / aspect_ratio;
        vfov = 20.0;
    } else if flag == 6 {
        world = cornell_box();
        aspect_ratio = 1.0;
        image_width = 1200.0; //change here !

        image_height = image_width / aspect_ratio;
        samples_per_pixel = 200;
        background = Color::new(0.0, 0.0, 0.0);
        lookfrom = Point3::new(278.0, 278.0, -800.0);
        lookat = Point3::new(278.0, 278.0, 0.0);
        vfov = 40.0;
    } else if flag == 7 {
        world = cornell_smoke();
        aspect_ratio = 1.0;
        image_width = 600.0;
        samples_per_pixel = 50;
        image_height = image_width / aspect_ratio;
        lookfrom = Vec3::new(278.0, 278.0, -800.0);
        lookat = Vec3::new(278.0, 278.0, 0.0);
        background = Color::new(0.0, 0.0, 0.0);
        vfov = 40.0;
    } else if flag == 8 {
        world = final_scene();
        aspect_ratio = 1.0;
        image_width = 400.0;
        image_height = image_width / aspect_ratio;
        samples_per_pixel = 50;
        lookfrom = Point3::new(478.0, 278.0, -600.0);
        lookat = Point3::new(278.0, 278.0, 0.0);
        background = Color::new(0.0, 0.0, 0.0);
        vfov = 40.0;
    } else {
        //???
        world = my_scene();
        vfov = 40.0;
        lookfrom = Point3::new(478.0, 278.0, -600.0);
        lookat = Point3::new(278.0, 278.0, 0.0);
        background = Point3::new(0.0, 0.0, 0.0);
    }

    let cam: Camera = Camera::camera(
        lookfrom.copy(),
        lookat.copy(),
        vup.copy(),
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let height = image_height;
    let width = image_width;
    let quality = 50; // From 0 to 100
    let path = "output/output.jpg";

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(width.to_string() + &"x".to_string() + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    // Generate image

    println!(
        "{} 🚀 {} {} {}",
        style("[2/5]").bold().dim(),
        style("Rendering with").green(),
        style((THREAD_NUMBER + 2).to_string()).yellow(),
        style("Threads...").green(),
    );

    let SECTION_LINE_NUM: usize = (image_height as usize) / (THREAD_NUMBER * 4);
    let mut output_pixel_color = Vec::<Color>::new();
    let mut thread_pool = Vec::<_>::new();
    let multiprogress = Arc::new(MultiProgress::new());
    multiprogress.set_move_cursor(true); // turn on this to reduce flickering

    for thread_id in 0..(THREAD_NUMBER + 2) {
        let line_beg = match thread_id {
            0 => 0,
            1 => SECTION_LINE_NUM * 3,
            2 => SECTION_LINE_NUM * 6,
            3 => SECTION_LINE_NUM * 10,
            4 => SECTION_LINE_NUM * 13, //28000
            5 => SECTION_LINE_NUM * 16, //和下一行对应 倒数第四个

            6 => SECTION_LINE_NUM * 18,
            7 => SECTION_LINE_NUM * 21,
            _ => SECTION_LINE_NUM * 24,
        };
        let line_end = match thread_id {
            0 => SECTION_LINE_NUM * 3,
            1 => SECTION_LINE_NUM * 6,
            2 => SECTION_LINE_NUM * 10,
            3 => SECTION_LINE_NUM * 13,
            4 => SECTION_LINE_NUM * 16,
            5 => SECTION_LINE_NUM * 18,
            6 => SECTION_LINE_NUM * 21,
            7 => SECTION_LINE_NUM * 24,
            _ => image_height as usize,
        };

        let world = world.copy();

        let cam: Camera = Camera::camera(
            lookfrom.copy(),
            lookat.copy(),
            vup.copy(),
            vfov,
            aspect_ratio,
            aperture,
            dist_to_focus,
            0.0,
            1.0,
        );

        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new(
            ((line_end - line_beg) * (image_width as usize)) as u64,
        ));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

        let (tx, rx) = mpsc::channel();

        let back = background.copy();
        thread_pool.push((
            thread::spawn(move || {
                let mut progress = 0;
                progress_bar.set_position(0);

                let channel_send = tx;

                let mut section_pixel_color = Vec::<Color>::new();

                //let mut rnd = rand::thread_rng();

                for y in line_beg..line_end {
                    for x in 0..image_width as usize {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _i in 0..samples_per_pixel {
                            let u = (x as f64 + random_double()) / (image_width - 1.0) as f64;
                            let v = (y as f64 + random_double()) / (image_height - 1.0) as f64;
                            let ray = cam.get_ray(u, v);
                            let back = back.copy();
                            pixel_color =
                                pixel_color.copy() + ray_color(&ray, back, &world, max_depth);
                        }
                        section_pixel_color.push(pixel_color);

                        progress += 1;
                        progress_bar.set_position(progress);
                    }
                }
                channel_send.send(section_pixel_color).unwrap();
                progress_bar.finish_with_message("Finished.");
                //println!("rust says {}", capture);
            }),
            rx,
        ));
    }
    // 等待所有线程结束
    multiprogress.join().unwrap();

    //========================================================

    println!(
        "{} 🚛 {}",
        style("[3/5]").bold().dim(),
        style("Collecting Threads Results...").green(),
    );

    //let mut thread_finish_successfully = true;
    let collecting_progress_bar = ProgressBar::new((THREAD_NUMBER + 2) as u64);
    // join 和 recv 均会阻塞主线程
    for thread_id in 0..(THREAD_NUMBER + 2) {
        let thread = thread_pool.remove(0);
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
                collecting_progress_bar.inc(1);
            }
            Err(_) => {
                //thread_finish_successfully = false;
                println!(
                    "      ⚠️ {}{}{}",
                    style("Joining the ").red(),
                    style(thread_id.to_string()).yellow(),
                    style("th thread failed!").red(),
                );
            }
        }
    }

    collecting_progress_bar.finish_and_clear();

    println!(
        "{} 🏭 {}",
        style("[4/5]").bold().dim(),
        style("Generating Image...").green()
    );

    let mut pixel_id = 0;

    for y in 0..image_height as u32 {
        for x in 0..image_width as u32 {
            //let pixel_color = output_pixel_color[pixel_id].calc_color(samples_per_pixel);
            let mut r = output_pixel_color[pixel_id].copy().x;
            let mut g = output_pixel_color[pixel_id].copy().y;
            let mut b = output_pixel_color[pixel_id].copy().z;

            let scale = 1.0 / samples_per_pixel as f64;
            r = (r * scale).sqrt();
            g = (g * scale).sqrt();
            b = (b * scale).sqrt();

            let pixel_color = [
                (256.0 * clamp(r, 0.0, 0.999)) as u8,
                (256.0 * clamp(g, 0.0, 0.999)) as u8,
                (256.0 * clamp(b, 0.0, 0.999)) as u8,
            ];

            let pixel = img.get_pixel_mut(x, (image_height - y as f64 - 1.0) as u32);
            *pixel = image::Rgb(pixel_color);

            pixel_id += 1;
        }
    }

    /*for j in 0..height as usize {
        for i in 0..width as usize {
            let mut my_pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                //_s ???
                let u = (i as f64 + random_double()) as f64 / (image_width - 1.0);
                let v = (j as f64 + random_double()) as f64 / (image_height - 1.0);
                let r: Ray = cam.get_ray(u, v);
                my_pixel_color =
                    my_pixel_color.copy() + ray_color(&r, background.copy(), &world, max_depth);
            }
            //let tmp = 255.999;
            let mut r = my_pixel_color.copy().x;
            let mut g = my_pixel_color.copy().y;
            let mut b = my_pixel_color.copy().z;

            let scale = 1.0 / samples_per_pixel as f64;
            r = (r * scale).sqrt();
            g = (g * scale).sqrt();
            b = (b * scale).sqrt();

            let pixel_color = [
                (256.0 * clamp(r, 0.0, 0.999)) as u8,
                (256.0 * clamp(g, 0.0, 0.999)) as u8,
                (256.0 * clamp(b, 0.0, 0.999)) as u8,
            ];

            let pixel = img.get_pixel_mut(i as u32, height as u32 - j as u32 - 1);
            *pixel = image::Rgb(pixel_color);
            progress.inc(1);
        }
    }*/
    world.clear();
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
