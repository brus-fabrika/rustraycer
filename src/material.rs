use rand::Rng;

use crate::{camera::Ray, hit_record::HitRecord, vec3d::Vec3d, Color};

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> (Ray, Color, bool);
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hr: &HitRecord) -> (Ray, Color, bool) {
        let mut scatter_direction = Vec3d::add(&hr.normal, &Vec3d::random_unit());
        if scatter_direction.near_zero() {
            scatter_direction = hr.normal.clone();
        }
        let scattered = Ray::new(hr.point.clone(), scatter_direction);
        let attenuation = self.albedo.clone();
        (scattered, attenuation, true)
    }

}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> (Ray, Color, bool) {
        let scatter_direction = Vec3d::reflect(&ray_in.direction, &hr.normal);
        let scatter_direction = Vec3d::unit(&scatter_direction) + Vec3d::random_unit() * self.fuzz;
        let scattered = Ray::new(hr.point.clone(), scatter_direction);
        let attenuation = self.albedo.clone(); 
        let is_scattered = Vec3d::dot(&scattered.direction, &hr.normal) > 0.0;
        (scattered, attenuation, is_scattered)
    }

}

pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    pub refraction_index: f32
}

impl Dielectric {
    fn reflectance(&self, cosine: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> (Ray, Color, bool) {
        let ri = if hr.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_dir = Vec3d::unit(&ray_in.direction);

        let cos_theta = f32::min(Vec3d::dot(&Vec3d::mul(&unit_dir, -1.0), &hr.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;

        let cannot_refract = cannot_refract || (self.reflectance(cos_theta, ri) > rand::rng().random_range(0.0..1.0));

        let direction = if cannot_refract {Vec3d::reflect(&unit_dir, &hr.normal)} else { Vec3d::refract(unit_dir, hr.normal.clone(), ri) };

        let scattered = Ray::new(hr.point.clone(), direction);
        let attenuation = Color{r: 1.0, g: 1.0, b: 1.0};
        let is_scattered = true; 
        (scattered, attenuation, is_scattered)
    }

}

pub enum MaterialEnum {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for MaterialEnum {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> (Ray, Color, bool) {
        match self {
            MaterialEnum::Lambertian(lambertian) => lambertian.scatter(ray_in, hr),
            MaterialEnum::Metal(metal) => metal.scatter(ray_in, hr),
            MaterialEnum::Dielectric(dielectric) => dielectric.scatter(ray_in, hr),
        }
    }
}

