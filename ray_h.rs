pub mod ray {
    use crate::r3_vec::r3_vec_ops::r3_vector;
    use crate::unit_vector;
    

    
    pub struct Ray {
        point_origin: r3_vector,
        direction: r3_vector
    }

    impl Ray {
        pub fn new(Point_Origin: r3_vector, Direction: r3_vector) -> Ray {
            Ray {
                point_origin: Point_Origin,
                direction: Direction
            }
        }

        pub fn origin(&self) -> &r3_vector {
            &self.point_origin
        }

        pub fn direction(&self) -> &r3_vector {
            &self.direction            
        }

        pub fn location_at(&self, t: f64) -> r3_vector {
            &self.point_origin + &self.direction * t
        }
    }

    pub fn hit_sphere(center: &r3_vector, radius: f64, ray: &Ray ) -> bool {
        let oc: r3_vector = ray.origin() - center;
        let a: f64 = ray.direction().dot(ray.direction());
        let b: f64 = 2.0 * &oc.dot(ray.direction());
        let c: f64 = &oc.dot(&oc) - &radius * &radius;
        let discriminant = &b * &b - 4.0 * &a * &c;
        discriminant > 0.0  
    }
    
    pub fn ray_color(r: &Ray) -> r3_vector {
        if hit_sphere(&r3_vector::new(0.0,0.0,-1.0), 0.5, r) {
            return r3_vector::new(1.0,0.0,0.0)
            }
        let unit_direction: r3_vector = unit_vector(&r.direction());
        let t = 0.5*(&unit_direction.y()+1.0);
        return r3_vector::new(1.0,1.0,1.0))*(1.0-&t) + r3_vector::new(0.5,0.7,1.0)*t
    }
}