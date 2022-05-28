

pub mod write_color {
    use crate::r3_vec::r3_vec_ops::r3_vector as color;

    pub fn out_color(pixel_color: &color) {
        let x = pixel_color.x() * 255.99;
        let y = pixel_color.y() * 255.99;
        let z = pixel_color.z() * 255.99; 

        println!("{} {} {}", x, y, z);
    }
}