#[derive(Debug, PartialEq)]
pub struct Vec3d{pub x: f32, pub y: f32, pub z: f32}

impl Vec3d {
    /*pub fn add(&mut self, v: &Vec3d) -> &Self {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self
    }*/
    
    pub fn add(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
        Vec3d { x: v1.x + v2.x, y: v1.y + v2.y, z: v1.z + v2.z }
    }

    pub fn mul(v: &Vec3d, t: f32) -> Vec3d {
        Vec3d{x: v.x *t, y: v.y * t, z: v.z * t}
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

