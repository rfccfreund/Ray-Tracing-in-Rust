mod r3_vec;
mod write_color;
mod ray_h;

use crate::r3_vec::r3_vec_ops::r3_vector as color;
use crate::r3_vec::r3_vec_ops::r3_vector;
use crate::r3_vec::r3_vector::unit_vector;
use crate::write_color::write_color::out_color;
use crate::ray_h::ray::*;


fn main() {
    
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i16 = 400;
    let image_height: f64 = image_width as f64 / aspect_ratio;   
    
    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = r3_vector::new(0.0, 0.0, 0.0);
    let horizontal = r3_vector::new(viewport_width, 0.0, 0.0);
    let vertical =r3_vector::new(0.0, viewport_height, 0.0);
    // calculates the lower left corner of the screen by taking the center (origin)
    // and shifting to the left by the horizontal vector (viewport width/2) and
    // shifting down by the vertical vector (viewport_height/2) and into the screen
    // by the focal length
    let lower_left_corner = &origin - 
        &horizontal/2.0 - &vertical/2.0 - r3_vector::new(0.0, 0.0, focal_length);


    //Render
    
    println!("P3\n{} {}\n255", image_width, image_height);

    //interates over the entire screen starting in the bottom left
    for i in (0..image_height as i32).rev() {
        for  j in 0..image_width {
            let start = r3_vector::new(0.0, 0.0, 0.0);
            let org = r3_vector::new(0.0, 0.0, 0.0);
            //u and j go from 0 to 1  
            let u = j as f64 / (image_width-1) as f64;
            let v = i as f64 / (image_height-1.0);
            let r = Ray::new(org, &lower_left_corner + &horizontal * u + &vertical * v - &start);
            let pixel_color = ray_color(&r);
            out_color(&pixel_color); 
        }
    }   
}
