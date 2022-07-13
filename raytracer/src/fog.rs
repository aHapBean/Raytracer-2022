use crate::aabb::*;
use crate::material::*;
use crate::mod_vec3::*;
use crate::random_double;
use crate::ray::*;
use crate::sphere::*;
use crate::texture::*;
use crate::tool_func::*;

type Color = Vec3;
type Point3 = Vec3;
const INFINITY: f64 = 1.79769e+308;
pub struct ConstantMedium {
    boundary: Object,
    phase_function: Material,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: &Object, d: f64, a: &Texture) -> ConstantMedium {
        let boundary = unwrap_object(b);
        let neg_inv_density = -1.0 / d;
        let phase_function = Material::Iso(Isotropic::new(a));
        ConstantMedium {
            boundary,
            neg_inv_density,
            phase_function,
        }
    }
    pub fn copy(&self) -> ConstantMedium {
        ConstantMedium {
            boundary: unwrap_object(&self.boundary),
            neg_inv_density: self.neg_inv_density,
            phase_function: unwrap_material(&self.phase_function),
        }
    }

    pub fn new_by_color(b: &Object, d: f64, c: Color) -> ConstantMedium {
        let boundary = unwrap_object(b);
        let neg_inv_density = -1.0 / d;
        let phase_function = Material::Iso(Isotropic::new_by_color(c));
        ConstantMedium {
            boundary,
            neg_inv_density,
            phase_function,
        }
    }
}

impl Hit for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let enableDebug = false;
        let debugging = enableDebug && random_double() < 0.00001;

        let rec1: &mut HitRecord = &mut HitRecord::hitrecord();
        let rec2: &mut HitRecord = &mut HitRecord::hitrecord();

        if !unwrap_object_hit(&self.boundary, r.copy(), -INFINITY, INFINITY, rec1) {
            return false;
        }
        if !unwrap_object_hit(&self.boundary, r.copy(), rec1.t + 0.0001, INFINITY, rec2) {
            return false;
        }

        if debugging {
            eprintln!("\nt_min = {},t_max = {}", t_min, t_max);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln(); //tag !

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!(
                "hit_distance = {}\nrec.t = {}\n rec.p.x = {}",
                hit_distance,
                rec.t,
                rec.p.copy().x
            );
        }
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat_ptr = unwrap_material(&self.phase_function);
        true
    }
}
impl BoundingBox for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::AABB) -> bool {
        unwrap_object_bounding_box_yes(&self.boundary, time0, time1, output_box)
    }
    fn bounding_box_for_sort(&self, time0: f64, time1: f64, output_Boxx: &mut AABB) -> AABB {
        self.bounding_box(time0, time1, output_Boxx);
        output_Boxx.copy()
    }
}
//这个应该放到texture去 ！！！
pub struct Isotropic {
    pub albedo: Texture,
}
impl Isotropic {
    pub fn new_by_color(c: Color) -> Isotropic {
        let albedo = Texture::So(SolidColor::new(c));
        Isotropic { albedo }
    }
    pub fn new(a: &Texture) -> Isotropic {
        Isotropic {
            albedo: unwrap_texture(a),
        }
    }
    pub fn copy(&self) -> Isotropic {
        Isotropic {
            albedo: unwrap_texture(&self.albedo),
        }
    }
}
impl Scatter for Isotropic {
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p.copy(), Vec3::random_in_unit_sphere(), r.time());
        *attenuation = unwrap_texture_color(&self.albedo, rec.u, rec.v, &rec.p);
        true
    }
}
impl Emitted for Isotropic {
    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
