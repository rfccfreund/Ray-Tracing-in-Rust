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

    pub fn hit_sphere(center: &r3_vector, radius: f64, ray: &Ray ) -> f64 {
        let oc: r3_vector = ray.origin() - center;
        let a: f64 = ray.direction().dot(ray.direction());
        let b: f64 = 2.0 * &oc.dot(ray.direction());
        let c: f64 = &oc.dot(&oc) - &radius * &radius;
        let discriminant = &b * &b - 4.0 * &a * &c;
        if (discriminant < 0.0) {
            return -1.0;
        }
        else {
            return (-&b - discriminant.sqrt()) / (2.0*&a);
        }  
    }
    
    pub fn ray_color(r: &Ray) -> r3_vector {
        let t: f64 = hit_sphere(r3_vector::new(0.0,0.0,-1), 0.5, r);
        if (t > 0.0) {
            let N: r3_vector = (&r.location_at(&t) - r3_vector::new(0.0,0.0,-1.0)).unit_vector();
            return 0.5*r3_vector::new(N.x()+1, N.y()+1, N.z()+1);
        }
        let unit_direction: r3_vector = r.direction().unit_vector();
        let t = 0.5*(unit_direction.y() + 1.0);
        return r3_vector::new(1.0,1.0,1.0))*(1.0-&t) + r3_vector::new(0.5,0.7,1.0)*t
    }
}