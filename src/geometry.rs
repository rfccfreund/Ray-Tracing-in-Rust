pub use std::ops;
use Vec3;

//vector class that creates 3 dim vector and impls additional functionality
//not found in the std vector 
pub mod euclid {
    pub struct Vec3 { 
        e: Vec<f64>
    }

    
    impl Vec3 {
        //TODO rewrite function to take 3 floats to produce vec 3
        pub fn new(s: Vec<f64>) -> Vec3 {
            Vec3 {
                e: s
            }
         }

        //fns to returns the x,y,z cordinate of a given euclid
        pub fn x(&self) ->  f64 {
            self.e[0]
        }

        pub fn y(&self) -> f64 {
            self.e[1]
        }

        pub fn z(&self) -> f64 {
            self.e[2]
        }
        
        //assist function for finding the vector length
        pub fn length_squared(&self) -> f64 {
            self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
        }

        //finds vector length
        pub fn length(&self) -> f64 {
            self.length_squared().sqrt()
        }
        

        //calculates the dot product
        pub fn dot(&self, y: &Vec3) -> f64 {
            self.e[0] * y.e[0] + self.e[1] * y.e[1] + self.e[2] * y.e[2] 
        }

        pub fn cross(&self, y: &Vec3) -> Vec3 {
            let euclid_cross = Vec3::new(vec![
                self.e[1] * y.e[2] - self.e[2] * y.e[1],
                self.e[2] * y.e[0] - self.e[0] * y.e[2],
                self.e[0] * y.e[1] - self.e[1] * y.e[0]]);

            euclid_cross
        }


             
    }
    //unit vector function
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        let unit_length = &v.length();
        let unit_v: Vec3 = v / *unit_length;
        unit_v
    }

}    
    //impl std operations for the euclid class to allow easier manipulation
    impl std::fmt::Display for euclid::Vec3 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f,"{} {} {}", self.x(), self.y(), self.z())
        }
    }
    
    // + impl
    impl ops::Add<euclid::Vec3> for euclid::Vec3 {
        type Output = euclid::Vec3;

        fn add(self, _rhs: euclid::Vec3) -> euclid::Vec3 {
            let vec_sum: euclid::Vec3 = euclid::Vec3::new(vec![self.x()+_rhs.x(), self.y()+_rhs.y(), self.z()+_rhs.z()]);                    

            vec_sum
        }
    }

    // + impl
    impl ops::Add<&euclid::Vec3> for &euclid::Vec3 {
        type Output = euclid::Vec3;

        fn add(self, _rhs: &euclid::Vec3) -> euclid::Vec3 {
            let vec_sum: euclid::Vec3 = euclid::Vec3::new(vec![self.x()+_rhs.x(), self.y()+_rhs.y(), self.z()+_rhs.z()]);                    

            vec_sum
        }
    }

    // + impl
    impl ops::Add<&euclid::Vec3> for euclid::Vec3 {
        type Output = euclid::Vec3;

        fn add(self, _rhs: &euclid::Vec3) -> euclid::Vec3 {
            let vec_sum: euclid::Vec3 = euclid::Vec3::new(vec![self.x()+_rhs.x(), self.y()+_rhs.y(), self.z()+_rhs.z()]);                    

            vec_sum
        }
    }

    // + impl
    impl ops::Add<euclid::Vec3> for &euclid::Vec3 {
        type Output = euclid::Vec3;

        fn add(self, _rhs: euclid::Vec3) -> euclid::Vec3 {
            let vec_sum: euclid::Vec3 = euclid::Vec3::new(vec![self.x()+_rhs.x(), self.y()+_rhs.y(), self.z()+_rhs.z()]);                    

            vec_sum
        }
    }

    // - impl
    impl ops::Sub<euclid::Vec3> for euclid::Vec3 {
        type Output = euclid::Vec3;

        fn sub(self, _rhs: euclid::Vec3) -> euclid::Vec3 {
            let vec_sub: euclid::Vec3 = euclid::Vec3::new(vec![self.x()-_rhs.x(), self.y()-_rhs.y(), self.z()-_rhs.z()]);                    

            vec_sub
        }
    }

    // - impl
    impl ops::Sub<&euclid::Vec3> for &euclid::Vec3 {
        type Output = euclid::Vec3;

        fn sub(self, _rhs: &euclid::Vec3) -> euclid::Vec3 {
            let vec_sub: euclid::Vec3 = euclid::Vec3::new(vec![self.x()-_rhs.x(), self.y()-_rhs.y(), self.z()-_rhs.z()]);                    

            vec_sub
        }
    }

    // - impl
    impl ops::Sub<&euclid::Vec3> for euclid::Vec3 {
        type Output = euclid::Vec3;

        fn sub(self, _rhs: &euclid::Vec3) -> euclid::Vec3 {
            let vec_sub: euclid::Vec3 = euclid::Vec3::new(vec![self.x()-_rhs.x(), self.y()-_rhs.y(), self.z()-_rhs.z()]);                    

            vec_sub
        }
    }  

    // - impl
    impl ops::Sub<euclid::Vec3> for &euclid::Vec3 {
        type Output = euclid::Vec3;

        fn sub(self, _rhs: euclid::Vec3) -> euclid::Vec3 {
            let vec_sub: euclid::Vec3 = euclid::Vec3::new(vec![self.x()-_rhs.x(), self.y()-_rhs.y(), self.z()-_rhs.z()]);                    

            vec_sub
        }
    }

    // * impl
    impl ops::Mul<f64> for euclid::Vec3 {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self {
            euclid::Vec3::new(vec!(self.x()*rhs, self.y()*rhs, self.z()*rhs))
        }
    }

    // * impl
    impl ops::Mul<f64> for &euclid::Vec3 {
        type Output = Vec3;

        fn mul(self, rhs: f64) -> Vec3 {
            euclid::Vec3::new(vec!(self.x()*rhs, self.y()*rhs, self.z()*rhs))
        }
    }

    // / impl
    impl ops::Div<f64> for euclid::Vec3 {
        type Output = Self;

        fn div(self, rhs: f64) -> Self {
            euclid::Vec3::new(vec!(self.x()/rhs, self.y()/rhs, self.z()/rhs))
        }
    }

    // / impl
    impl ops::Div<f64> for &euclid::Vec3 {
        type Output = Vec3;

        fn div(self, rhs: f64) -> Vec3 {
            euclid::Vec3::new(vec!(self.x()/rhs, self.y()/rhs, self.z()/rhs))
        }
    }

    // -() impl
    impl ops::Neg for euclid::Vec3 {
        type Output = Self;

        fn neg(self) -> euclid::Vec3 {
            let vec_neg: euclid::Vec3 = euclid::Vec3::new(vec![-self.x(), -self.y(), -self.z()]);                    

            vec_neg
        }
    }

    

    // += impl
    impl ops::AddAssign for euclid::Vec3 {
        fn add_assign(&mut self, other: Self) {
            *self =  euclid::Vec3::new(vec![self.x()+other.x(), self.y()+other.y(), self.z()+other.z()]);                            
        }
    }

    // *= impl
    impl ops::MulAssign<f64> for euclid::Vec3 {
        fn mul_assign(&mut self, rhs: f64) {
            *self = euclid::Vec3::new(vec![self.x()*rhs, self.y()*rhs, self.z()*rhs]);
        }
    }

    // /= impl
    impl ops::DivAssign<f64> for euclid::Vec3 {
        fn div_assign(&mut self, rhs: f64) {
            *self = euclid::Vec3::new(vec![self.x()/rhs, self.y()/rhs, self.z()/rhs]);
        }
    }

