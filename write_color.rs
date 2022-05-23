

pub mod write_color {
    use crate::vec3_h::vec3::Vec3 as color;

    pub fn out_color(pixel_color: &color) {
        let x = pixel_color.x() * 255.99;
        let y = pixel_color.y() * 255.99;
        let z = pixel_color.z() * 255.99; 

        println!("{} {} {}", x, y, z);
    }
}