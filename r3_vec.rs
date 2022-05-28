pub use std::ops;

use self::r3_vec_ops::r3_vector;

//vector class that creates 3 dim vector and impls additional functionality
//not found in the std vector 
pub mod r3_vec_ops {
    pub struct r3_vector { 
        x: f64,
        y: f64,
        z: f64
    }

    
    impl r3_vector {
        //TODO rewrite function to take 3 floats to produce vec 3
        pub fn new(i: f64, j: f64, k: f64) -> r3_vector {
            r3_vector {
                x: i,
                y: j,
                z: k
            }
         }

        //fns to returns the x,y,z cordinate of a given r3_vector
        pub fn x(&self) ->  f64 {
            self.x
        }

        pub fn y(&self) -> f64 {
            self.y
        }

        pub fn z(&self) -> f64 {
            self.z
        }
        
        //assist function for finding the vector length
        pub fn length_squared(&self) -> f64 {
            self.x*self.x + self.y*self.y + self.z*self.z
        }

        //finds vector length
        pub fn length(&self) -> f64 {
            self.length_squared().sqrt()
        }
        

        //calculates the dot product
        pub fn dot(&self, vec: &r3_vector) -> f64 {
            self.x * vec.x + self.y * vec.y + self.z * vec.z
        }

        pub fn cross(&self, vec: &r3_vector) -> r3_vector {
            let r3_vector_cross = r3_vector::new( 
                self.y * vec.z - self.z * vec.y,
                self.z * vec.x - self.x * vec.z,
                self.x * vec.y - self.y * vec.x);

            r3_vector_cross
        }


             
    }
    //unit vector function
    pub fn unit_vector(v: &r3_vector) -> r3_vector {
        let unit_length = &v.length();
        let unit_v: r3_vector = v / *unit_length;
        unit_v
    }

}    
    //impl std operations for the r3_vector class to allow easier manipulation
    impl std::fmt::Display for r3_vec_ops::r3_vector {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f,"{} {} {}", self.x(), self.y(), self.z())
        }
    }
    
    // + impl
    impl ops::Add<r3_vec_ops::r3_vector> for r3_vec_ops::r3_vector {
        type Output = r3_vec_ops::r3_vector;

        fn add(self, _rhs: r3_vec_ops::r3_vector) -> r3_vec_ops::r3_vector {
            let vec_sum: r3_vec_ops::r3_vector = r3_vec_ops::r3_vector::new(self.x()+_rhs.x(), self.y()+_rhs.y(), self.z()+_rhs.z());                    

            vec_sum
        }
    }

    // + impl
    impl ops::Add<&r3_vec_ops::r3_vector> for &r3_vec_ops::r3_vector {
        type Output = r3_vec_ops::r3_vector;

        fn add(self, _rhs: &r3_vec_ops::r3_vector) -> r3_vec_ops::r3_vector {
            let vec_sum: r3_vec_ops::r3_vector = r3_vec_ops::r3_vector::new(self.x()+_rhs.x(), self.y()+_rhs.y(), self.z()+_rhs.z());                    

            vec_sum
        }
    }

    // + impl
    impl ops::Add<&r3_vec_ops::r3_vector> for r3_vec_ops::r3_vector {
        type Output = r3_vec_ops::r3_vector;

        fn add(self, _rhs: &r3_vec_ops::r3_vector) -> r3_vec_ops::r3_vector {
            let vec_sum: r3_vec_ops::r3_vector = r3_vec_ops::r3_vector::new(self.x()+_rhs.x(), self.y()+_rhs.y(), self.z()+_rhs.z());                    

            vec_sum
        }
    }

    // + impl
    impl ops::Add<r3_vec_ops::r3_vector> for &r3_vec_ops::r3_vector {
        type Output = r3_vec_ops::r3_vector;

        fn add(self, _rhs: r3_vec_ops::r3_vector) -> r3_vec_ops::r3_vector {
            let vec_sum: r3_vec_ops::r3_vector = r3_vec_ops::r3_vector::new(self.x()+_rhs.x(), self.y()+_rhs.y(), self.z()+_rhs.z());                    

            vec_sum
        }
    }

    // - impl
    impl ops::Sub<r3_vec_ops::r3_vector> for r3_vec_ops::r3_vector {
        type Output = r3_vec_ops::r3_vector;

        fn sub(self, _rhs: r3_vec_ops::r3_vector) -> r3_vec_ops::r3_vector {
            let vec_sub: r3_vec_ops::r3_vector = r3_vec_ops::r3_vector::new(self.x()-_rhs.x(), self.y()-_rhs.y(), self.z()-_rhs.z());                    

            vec_sub
        }
    }

    // - impl
    impl ops::Sub<&r3_vec_ops::r3_vector> for &r3_vec_ops::r3_vector {
        type Output = r3_vec_ops::r3_vector;

        fn sub(self, _rhs: &r3_vec_ops::r3_vector) -> r3_vec_ops::r3_vector {
            let vec_sub: r3_vec_ops::r3_vector = r3_vec_ops::r3_vector::new(self.x()-_rhs.x(), self.y()-_rhs.y(), self.z()-_rhs.z());                    

            vec_sub
        }
    }

    // - impl
    impl ops::Sub<&r3_vec_ops::r3_vector> for r3_vec_ops::r3_vector {
        type Output = r3_vec_ops::r3_vector;

        fn sub(self, _rhs: &r3_vec_ops::r3_vector) -> r3_vec_ops::r3_vector {
            let vec_sub: r3_vec_ops::r3_vector = r3_vec_ops::r3_vector::new(self.x()-_rhs.x(), self.y()-_rhs.y(), self.z()-_rhs.z());                    

            vec_sub
        }
    }  

    // - impl
    impl ops::Sub<r3_vec_ops::r3_vector> for &r3_vec_ops::r3_vector {
        type Output = r3_vec_ops::r3_vector;

        fn sub(self, _rhs: r3_vec_ops::r3_vector) -> r3_vec_ops::r3_vector {
            let vec_sub: r3_vec_ops::r3_vector = r3_vec_ops::r3_vector::new(self.x()-_rhs.x(), self.y()-_rhs.y(), self.z()-_rhs.z());                    

            vec_sub
        }
    }

    // * impl
    impl ops::Mul<f64> for r3_vec_ops::r3_vector {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self {
            r3_vec_ops::r3_vector::new(self.x()*rhs, self.y()*rhs, self.z()*rhs)
        }
    }

    // * impl
    impl ops::Mul<f64> for &r3_vec_ops::r3_vector {
        type Output = r3_vector;

        fn mul(self, rhs: f64) -> r3_vector {
            r3_vec_ops::r3_vector::new(self.x()*rhs, self.y()*rhs, self.z()*rhs)
        }
    }

    // / impl
    impl ops::Div<f64> for r3_vec_ops::r3_vector {
        type Output = Self;

        fn div(self, rhs: f64) -> Self {
            r3_vec_ops::r3_vector::new(self.x()/rhs, self.y()/rhs, self.z()/rhs)
        }
    }

    // / impl
    impl ops::Div<f64> for &r3_vec_ops::r3_vector {
        type Output = r3_vector;

        fn div(self, rhs: f64) -> r3_vector {
            r3_vec_ops::r3_vector::new(self.x()/rhs, self.y()/rhs, self.z()/rhs)
        }
    }

    // -() impl
    impl ops::Neg for r3_vec_ops::r3_vector {
        type Output = Self;

        fn neg(self) -> r3_vec_ops::r3_vector {
            let vec_neg: r3_vec_ops::r3_vector = r3_vec_ops::r3_vector::new(-self.x(), -self.y(), -self.z());                    

            vec_neg
        }
    }

    

    // += impl
    impl ops::AddAssign for r3_vec_ops::r3_vector {
        fn add_assign(&mut self, other: Self) {
            *self =  r3_vec_ops::r3_vector::new(self.x()+other.x(), self.y()+other.y(), self.z()+other.z());                            
        }
    }

    // *= impl
    impl ops::MulAssign<f64> for r3_vec_ops::r3_vector {
        fn mul_assign(&mut self, rhs: f64) {
            *self = r3_vec_ops::r3_vector::new(self.x()*rhs, self.y()*rhs, self.z()*rhs);
        }
    }

    // /= impl
    impl ops::DivAssign<f64> for r3_vec_ops::r3_vector {
        fn div_assign(&mut self, rhs: f64) {
            *self = r3_vec_ops::r3_vector::new(self.x()/rhs, self.y()/rhs, self.z()/rhs);
        }
    }

