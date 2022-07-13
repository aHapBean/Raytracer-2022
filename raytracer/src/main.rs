#![allow(warnings, unused)]

use std::fs::File;
use std::process::exit;

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

mod aabb;
mod bvh;
mod camera;
mod material;
mod mod_vec3;
mod moving_sphere;
mod ray;
mod rect;
mod sphere;
mod texture;
mod tool_func;
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

use crate::rect::XYrect;
use crate::texture::SolidColor;

use crate::tool_func::*;

type Color = Vec3;
type Point3 = Vec3;
fn abs(a: f64) -> f64 {
    if a < 0.0 {
        return -a;
    } else {
        return a;
    }
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
        return emitted;
    }; //?????unreachable
       //return Color::new(0.0, 0.0, 0.0);
       //let unit_direction = r.dir.unit_vector();
       //let t = 0.5 * (unit_direction.y + 1.0);
       //(1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
       //背景色
    return emitted
        + attenuation.copy() * ray_color(&scattered, background.copy(), world, depth - 1);
}

/*
fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin() - center.copy();
    let a: f64 = r.direction().dot(r.direction());
    let half_b: f64 = r.direction().dot(oc.copy()); //所有函数都不引用。+.copy()就可以
    let c: f64 = oc.dot(oc.copy()) - radius * radius;
    let det = half_b * half_b - a * c;
    if det > 0.0 {
        return (-half_b - det.sqrt()) / (a);
    } else {
        return -1.0;
    }
}*/

//       TO change ! the center!!!
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
    eprintln!("here 1");
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

    let difflight = Material::Dif(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Object::XY(XYrect::new(
        3.0, 5.0, 1.0, 3.0, -2.0, &difflight,
    )));
    world
}
fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    // to preserve the None
    //let sph = Object::None;

    //sph.cope();

    //my code
    //let pi: f64 = 3.1415926535897932385;
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width: f64 = 400.0; //??
    //let image_width: f64 = 2400.0; //??
    let image_height: f64 = image_width as f64 / aspect_ratio;

    let height = image_height;
    let width = image_width;
    let quality = 100; // From 0 to 100
    let path = "output/output.jpg";

    //world
    //for 2 balls;
    let mut world: HittableList;
    //let tp = world;
    //let mut world = random_scene();
    //let R = (pi / 4.0).cos();

    let lookfrom;
    let lookat;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov: f64;
    let dist_to_focus = 10.0;
    let mut aperture = 0.1;
    let background: Color;

    let flag: i32 = 4;
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
        //eprintln!("here 1");
        world = two_perlin_spheres();
        background = Color::new(0.7, 0.8, 1.00);
        lookfrom = Point3::new(13.0, 2.0, 3.0);
        lookat = Point3::new(0.0, 0.0, 0.0);
        vfov = 20.0;
    } else if flag == 4 {
        //eprintln!("here 1");
        world = earth();
        background = Color::new(0.7, 0.8, 1.00);
        lookfrom = Point3::new(13.0, 2.0, 3.0);
        lookat = Point3::new(0.0, 0.0, 0.0);
        vfov = 20.0;
    } else {
        world = simple_light();
        //samples_per
        background = Color::new(0.0, 0.0, 0.0);
        lookfrom = Point3::new(26.0, 3.0, 6.0);
        lookat = Point3::new(0.0, 2.0, 0.0);
        vfov = 20.0;
    }

    let cam: Camera = Camera::camera(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let samples_per_pixel = 50;
    let max_depth = 50;

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
    for j in 0..height as usize {
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
    }
    world.clear(); //??
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
