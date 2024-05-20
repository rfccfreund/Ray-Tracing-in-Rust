pub mod light_ray {
    use crate::geometry::euclid::Vec3;
    use unit_vector;
         
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

    pub fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray ) -> f64 {
        let oc: Vec3 = ray.origin() - center;
        let a: f64 = ray.direction().dot(ray.direction());
        let b: f64 = 2.0 * &oc.dot(ray.direction());
        let c: f64 = &oc.dot(&oc) - &radius * &radius;
        let discriminant = &b * &b - 4.0 * &a * &c;

        if(discriminant < 0.0) {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()/ (2.0*a))
        }
         
    }
    
    pub fn ray_color(r: &Ray) -> Vec3 {
        let t = hit_sphere(&Vec3::new(vec!(0.0,0.0,-1.0)), 0.5, r);
        
        if( t > 0.0) {
            let norm_vec: Vec3 = unit_vector(&(r.location_at(t) - Vec3::new(vec!(0.0,0.0,-1.0))));
            return Vec3::new(vec!(&norm_vec.x() + 1.0, &norm_vec.y() + 1.0, &norm_vec.z() + 1.0)) * 0.5;
            }


        let unit_direction: Vec3 = unit_vector(&r.direction());
        let t = 0.5*(&unit_direction.y()+1.0);
        return Vec3::new(vec!(1.0,1.0,1.0))*(1.0-&t) + Vec3::new(vec!(0.5,0.7,1.0))*t
    }
}