pub mod ray {
    use crate::vec3_h::vec3::Vec3;
    use crate::unit_vector;
    

    
    pub struct Ray {
        point_origin: Vec3,
        direction: Vec3
    }

    impl Ray {
        pub fn new(Point_Origin: Vec3, Direction: Vec3) -> Ray {
            Ray {
                point_origin: Point_Origin,
                direction: Direction
            }
        }

        pub fn origin(&self) -> &Vec3 {
            &self.point_origin
        }

        pub fn direction(&self) -> &Vec3 {
            &self.direction            
        }

        pub fn location_at(&self, t: f64) -> Vec3 {
            &self.point_origin + &self.direction * t
        }
    }

    pub fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray ) -> bool {
        let oc: Vec3 = ray.origin() - center;
        let a: f64 = ray.direction().dot(ray.direction());
        let b: f64 = 2.0 * &oc.dot(ray.direction());
        let c: f64 = &oc.dot(&oc) - &radius * &radius;
        let discriminant = &b * &b - 4.0 * &a * &c;
        discriminant > 0.0  
    }
    
    pub fn ray_color(r: &Ray) -> Vec3 {
        if hit_sphere(&Vec3::new(vec!(0.0,0.0,-1.0)), 0.5, r) {
            return Vec3::new(vec!(1.0,0.0,0.0))
            }
        let unit_direction: Vec3 = unit_vector(&r.direction());
        let t = 0.5*(&unit_direction.y()+1.0);
        return Vec3::new(vec!(1.0,1.0,1.0))*(1.0-&t) + Vec3::new(vec!(0.5,0.7,1.0))*t
    }
}