use std::ops::Add;
use rand::Rng;

#[derive(Debug, PartialEq, Default)]
pub struct  Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

}

impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3d {
        Vec3d{x, y, z}
    }

    pub fn random() -> Vec3d {
        Vec3d::new(
            rand::rng().random_range(0.0..1.0),
            rand::rng().random_range(0.0..1.0),
            rand::rng().random_range(0.0..1.0)
        )
    }

    pub fn random_range(i: f32, j: f32) -> Vec3d {
        Vec3d::new(
            rand::rng().random_range(i .. j),
            rand::rng().random_range(i .. j),
            rand::rng().random_range(i .. j),
        )
    }

    pub fn clone(&self) -> Vec3d {
        Vec3d::new(self.x, self.y, self.z)
    }
    
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-8f32;
        (self.x.abs() < eps) && (self.y.abs() < eps) && (self.z.abs() < eps)
    }
    /*pub fn add(&mut self, v: &Vec3d) -> &Self {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self
    }*/

    pub fn unit(v: &Vec3d) -> Vec3d {
        Vec3d::mul(v, 1.0 / v.length())
    }

    pub fn random_unit() -> Vec3d {
        loop {
            let p = &Self::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-10 < lensq && lensq <= 1.0 {
                return Vec3d::unit(p);
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3d) -> Vec3d {
        let p = &Vec3d::random_unit();
        if Self::dot(p, normal) > 0.0 {
            p.clone()
        } else {
            Vec3d::mul(p, -1.0)
        }
    }

    pub fn add(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
        Vec3d { x: v1.x + v2.x, y: v1.y + v2.y, z: v1.z + v2.z }
    }
    
    pub fn sub(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
        Vec3d { x: v1.x - v2.x, y: v1.y - v2.y, z: v1.z - v2.z }
    }

    pub fn mul(v: &Vec3d, t: f32) -> Vec3d {
        Vec3d{x: v.x *t, y: v.y * t, z: v.z * t}
    }

    pub fn dot(v: &Vec3d, u: &Vec3d) -> f32 {
        v.x * u.x + v.y * u.y + v.z * u.z
    }

    pub fn reflect(v: &Vec3d, n: &Vec3d) -> Vec3d {
        let d2 = 2.0 * Self::dot(v, n);
        Self::sub(v, &Self::mul(n, d2))
    }

    pub fn refract(uv: Vec3d, n: Vec3d, etai_over_etat: f32) -> Vec3d {
        let cos_theta = f32::min(Vec3d::dot(&Vec3d::mul(&uv.clone(), -1.0), &n.clone()), 1.0);
        let r_out_perp = Vec3d::mul(&(uv + Vec3d::mul(&n.clone(), cos_theta)), etai_over_etat);
        let r_out_parallel = Vec3d::mul(&n, -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())));
        r_out_perp + r_out_parallel
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3d::Vec3d;

    /*#[test]
    fn vec_add_vec() {
        let mut v1 = Vec3d{x: 1.0, y: 2.0, z: 3.0};
        v1.add(&Vec3d{x: 2.0, y: 3.0, z: 4.0});
        assert_eq!(v1, Vec3d{x: 3.0, y: 5.0, z: 7.0});
    }*/
    #[test]
    fn vec_add_vec() {
        let v1 = Vec3d{x: 1.0, y: 2.0, z: 3.0};
        let r = Vec3d::add(&v1, &Vec3d{x: 2.0, y: 3.0, z: 4.0});
        assert_eq!(r, Vec3d{x: 3.0, y: 5.0, z: 7.0});
    }
    
    #[test]
    fn vec_plus_vec() {
        let v1 = Vec3d{x: 1.0, y: 2.0, z: 3.0};
        let r = v1.clone() + Vec3d{x: 2.0, y: 3.0, z: 4.0};
        assert_eq!(r, Vec3d{x: 3.0, y: 5.0, z: 7.0});
        assert_eq!(v1, Vec3d{x: 1.0, y: 2.0, z: 3.0});
    }

   /*#[test]
    fn vec_add_vec_chained() {
        let mut v1 = Vec3d{x: 1.0, y: 2.0, z: 3.0};
        
        let mut v2 = Vec3d{x: -3.0, y: -5.0, z: -7.0};

        v2.add(v1.add(&Vec3d{x: 2.0, y: 3.0, z: 4.0}));
        


        assert_eq!(v2, Vec3d{x: 0.0, y: 0.0, z: 0.0});
    }*/

    #[test]
    fn mul_vec_scalar() {
        let v1 = Vec3d{x: 1.0, y: 2.0, z: 3.0};
        let v2 = Vec3d::mul(&v1, 0.5);

        assert_eq!(v2, Vec3d{x: 0.5, y: 1.0, z: 1.5});
    }

}

