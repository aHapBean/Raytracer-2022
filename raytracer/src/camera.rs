use crate::mod_vec3::Cross;
use crate::mod_vec3::Vec3; //mod 类似于一棵树
use crate::ray::Ray;

type Point3 = Vec3;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn camera(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        //let aspect_ratio = 16.0 / 9.0;
        let theta = crate::degree_to_radians(vfov);
        let h = (theta / 2.0).tan(); //??????

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height as f64;

        let ww = Vec3::unit_vector(&(lookfrom.copy() - lookat.copy()));
        let uu = Vec3::unit_vector(&vup.cross(ww.copy())); //???
        let vv = ww.cross(uu.copy()); //可能改错

        let oorigin = lookfrom;
        let hhorizontal = focus_dist * viewport_width * uu.copy();
        let vvertical = focus_dist * viewport_height * vv.copy();
        let llower_left_corner: Vec3 = oorigin.copy()
            - hhorizontal.copy() / 2.0
            - vvertical.copy() / 2.0
            - focus_dist * ww.copy();

        let llens_radius = aperture / 2.0;

        Camera {
            origin: oorigin,
            horizontal: hhorizontal,
            vertical: vvertical,
            lower_left_corner: llower_left_corner,
            u: uu,
            v: vv,
            w: ww,
            lens_radius: llens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        //这要变吗？？
        let rd = self.lens_radius * crate::random_in_unit_dist();
        let offset = self.u.copy() * rd.copy().x + self.v.copy() * rd.copy().y;
        Ray {
            orig: self.origin.copy() + offset.copy(),
            dir: self.lower_left_corner.copy()
                + s * self.horizontal.copy()
                + t * self.vertical.copy()
                - self.origin.copy()
                - offset.copy(),
        }
    }
}
