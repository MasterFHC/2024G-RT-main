pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use crate::Interval;
use crate::util;
use crate::hit_record;
use std::sync::Arc;
use crate::textures::{texture, SolidColor};

pub trait material : Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct lambertian {
    tex: Arc<dyn texture + Send + Sync>,
}

impl lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn new_from_texture(tex: Arc<dyn texture + Send + Sync>) -> Self {
        Self {
            tex,
        }
    }
}

impl material for lambertian {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + util::random_on_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, r_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }
}

pub struct metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: util::fmax(0.0, util::fmin(1.0, fuzz)),
        }
    }
}

impl material for metal {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = util::reflect(r_in.b_direction, rec.normal);
        let reflected = reflected.unit_vector() + util::random_on_unit_sphere() * self.fuzz;
        *scattered = Ray::new(rec.p, reflected, r_in.time);
        *attenuation = self.albedo;
        scattered.b_direction * rec.normal > 0.0
    }
}

pub struct dielectric {
    pub refraction_index: f64,
}

impl dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index,
        }
    }
}

impl material for dielectric {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.b_direction.unit_vector();
        let cos_theta = util::fmin((unit_direction * (-1.0)) * rec.normal, 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let refracted = if cannot_refract || util::reflectance(cos_theta, refraction_ratio) > util::random_f64_0_1(){
            util::reflect(unit_direction, rec.normal)
        } else {
            util::refract(unit_direction, rec.normal, refraction_ratio)
        };
        *scattered = Ray::new(rec.p, refracted, r_in.time);
        true
    }
}