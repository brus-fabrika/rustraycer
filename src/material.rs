use crate::{camera::Ray, hit_record::HitRecord, vec3d::Vec3d, Color};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> (Ray, Color, bool);
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hr: &HitRecord) -> (Ray, Color, bool) {
        let mut scatter_direction = hr.normal.clone() + Vec3d::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hr.normal.clone();
        }
        let scattered = Ray::new(hr.point.clone(), scatter_direction);
        let attenuation = Color{r: self.albedo.r, g: self.albedo.g, b: self.albedo.b};
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
        let scatter_direction = Vec3d::unit(&scatter_direction) + Vec3d::mul(&Vec3d::random_unit(), self.fuzz);
        let scattered = Ray::new(hr.point.clone(), scatter_direction);
        let attenuation = Color{r: self.albedo.r, g: self.albedo.g, b: self.albedo.b};
        let is_scattered = Vec3d::dot(&scattered.direction, &hr.normal) > 0.0;
        (scattered, attenuation, is_scattered)
    }

}
