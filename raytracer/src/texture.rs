use crate::abs;
use crate::fog::Isotropic;
use crate::mod_vec3::Dot;
use crate::mod_vec3::Vec3;
use crate::random_int_range;
use crate::tool_func::*;

pub use image::{imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbImage};

type Color = Vec3;
type Point3 = Vec3;
pub enum Texture {
    So(SolidColor),
    Ch(Box<Checker_texture>),
    No(Noise_texture),
    Im(ImageTexture),
}

pub trait Value {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    pub color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }

    pub fn new_by_color(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color_value: Color::new(r, g, b),
        }
    }

    pub fn copy(&self) -> SolidColor {
        SolidColor {
            color_value: self.color_value.copy(),
        }
    }
}
impl Value for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color_value.copy()
    }
}

pub struct Checker_texture {
    pub odd: Texture,
    pub even: Texture,
}

impl Checker_texture {
    pub fn checker_texture(c1: Color, c2: Color) -> Checker_texture {
        Checker_texture {
            even: Texture::So(SolidColor::new(c1)),
            odd: Texture::So(SolidColor::new(c2)),
        }
    }
    pub fn new(_even: &Texture, _odd: &Texture) -> Checker_texture {
        Checker_texture {
            even: unwrap_texture(_even),
            odd: unwrap_texture(_odd),
        }
    }

    pub fn copy(&self) -> Checker_texture {
        Checker_texture {
            even: unwrap_texture(&self.even),
            odd: unwrap_texture(&self.odd),
        }
    }
}

impl Value for Checker_texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines =
            (10.0 * p.copy().x).sin() * (10.0 * p.copy().y).sin() * (10.0 * p.copy().z).sin();
        if sines < 0.0 {
            return unwrap_texture_color(&self.odd, u, v, p);
        } else {
            return unwrap_texture_color(&self.even, u, v, p);
        }
    }
}

pub struct Perlin {
    pub point_count: u32,
    pub ranvec: Vec<Vec3>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
}
impl Perlin {
    pub fn copy(&self) -> Perlin {
        Perlin {
            point_count: self.point_count,
            ranvec: copy_vec_for_vec3(&self.ranvec),
            perm_x: copy_vec(&self.perm_x),
            perm_y: copy_vec(&self.perm_y),
            perm_z: copy_vec(&self.perm_z),
        }
    }
    pub fn perlin() -> Perlin {
        let mut ppoint_count = 256;
        let mut rranvec: Vec<Vec3> = vec![];
        for _i in 0..ppoint_count {
            rranvec.push(Vec3::random_range(-1.0, 1.0).unit_vector());
        }

        let pperm_x = Perlin::perlin_generate_perm(ppoint_count);
        let pperm_y = Perlin::perlin_generate_perm(ppoint_count);
        let pperm_z = Perlin::perlin_generate_perm(ppoint_count);

        Perlin {
            point_count: ppoint_count,
            ranvec: rranvec,
            perm_x: pperm_x,
            perm_y: pperm_y,
            perm_z: pperm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let mut u = p.copy().x - floor(p.copy().x);
        let mut v = p.copy().y - floor(p.copy().y);
        let mut w = p.copy().z - floor(p.copy().z);

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = floor(p.copy().x) as i32;
        let j = floor(p.copy().y) as i32;
        let k = floor(p.copy().z) as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [
            [
                [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
                [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
            ],
            [
                [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
                [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
            ],
        ];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize]
                        .copy()
                }
            }
        }
        Perlin::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.copy();
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(temp_p.copy());
            weight *= 0.5;
            temp_p = 2.0 * temp_p.copy();
        }
        return abs(accum);
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += ((i as f64) * uu + (1 - i) as f64 * (1.0 - uu))
                        * ((j as f64) * vv + (1 - j) as f64 * (1.0 - vv))
                        * ((k as f64) * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i][j][k].dot(weight_v) as f64;
                }
            }
        }
        accum
    }

    fn perlin_generate_perm(point_count: u32) -> Vec<i32> {
        let mut p: Vec<i32> = vec![];
        //???这里复制是否会降低我的速度？？
        //eprintln!("rethink the copy!!");
        for i in 0..point_count {
            p.push(i as i32);
        }

        Perlin::permute(&mut p, point_count);
        p
    }

    fn permute(p: &mut Vec<i32>, point_count: u32) {
        for i in (0..point_count).rev() {
            let target: usize = random_int_range(0, i as i32) as usize;
            let tmp = p[i as usize];
            p[i as usize] = p[target];
            p[target] = tmp;
        }
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += ((i as f64) * u + (1 - i) as f64 * (1.0 - u))
                        * ((j as f64) * v + (1 - j) as f64 * (1.0 - v))
                        * ((k as f64) * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k] as f64;
                }
            }
        }
        accum
    }
}

pub struct Noise_texture {
    pub noise: Perlin,
    pub scale: f64,
}
impl Noise_texture {
    pub fn noise_texture(sc: f64) -> Noise_texture {
        Noise_texture {
            noise: Perlin::perlin(), //调用默认构造函数
            scale: sc,
        }
    }
    pub fn copy(&self) -> Noise_texture {
        Noise_texture {
            noise: self.noise.copy(),
            scale: self.scale,
        }
    }
}

impl Value for Noise_texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * /*self.noise.noise(p.copy())*/
         0.5
        * (1.0
            + (self.scale * p.copy().z + 10.0 * self.noise.turb(self.scale * p.copy(), 7))
                .sin())
    }
}

pub struct ImageTexture {
    bytes_per_pixel: i32,
    width: i32,
    height: i32,

    data: RgbImage,
    bytes_per_scanline: i32,
    //filename:String,
}

impl ImageTexture {
    //pub fn ImageTexture() -> ImageTexture {
    //    ImageTexture {
    //        bytes_per_pixel:3,
    //        width:0,
    //        height:0,
    //        bytes_per_scanline:0,
    //        data:
    //    }
    //}
    pub fn new(filename: String) -> ImageTexture {
        let bbytes_per_pixel = 3 as i32;

        let image: RgbImage = image::open(filename).unwrap().to_rgb8();
        let (wwidth, hheight) = image.dimensions();

        let bbytes_per_scanline = bbytes_per_pixel * wwidth as i32;

        ImageTexture {
            bytes_per_pixel: bbytes_per_pixel,
            width: wwidth as i32,
            height: hheight as i32,

            data: image,
            bytes_per_scanline: bbytes_per_scanline,
            //filename,
        }
    }
    pub fn copy(&self) -> ImageTexture {
        ImageTexture {
            bytes_per_pixel: self.bytes_per_pixel,
            width: self.width,
            height: self.height,

            data: self.data.clone(),
            bytes_per_scanline: self.bytes_per_scanline,
            //filename:self.filename.clone(),
        }
    }
}

impl Value for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        //if self.data.size() == 0 {
        //    return Color::new(0.0,1.0,1.0);
        //}

        let u = crate::clamp(u, 0.0, 1.0);
        let v = 1.0 - crate::clamp(v, 0.0, 1.0);
        //tag here !
        let mut i = u * self.width as f64;
        let mut j = v * self.height as f64;

        if i >= self.width as f64 {
            i = self.width as f64 - 1.0;
        }
        if j >= self.height as f64 {
            j = self.height as f64 - 1.0;
        }

        let color_scale = 1.0 / 255.0;
        Color::new(
            self.data.get_pixel(i as u32, j as u32)[0] as f64 * color_scale,
            self.data.get_pixel(i as u32, j as u32)[1] as f64 * color_scale,
            self.data.get_pixel(i as u32, j as u32)[2] as f64 * color_scale,
        )
    }
}
