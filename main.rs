mod vec3_h;
mod write_color;
mod ray_h;

use crate::vec3_h::vec3::Vec3 as color;
use crate::vec3_h::vec3::Vec3;
use crate::vec3_h::vec3::unit_vector;
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

    let origin = Vec3::new(vec!(0.0, 0.0, 0.0));
    let horizontal = Vec3::new(vec!(viewport_width, 0.0, 0.0));
    let vertical =Vec3::new(vec!(0.0, viewport_height, 0.0));
    let lower_left_corner = &origin - 
        &horizontal/2.0 - &vertical/2.0 - Vec3::new(vec!(0.0, 0.0, focal_length));


    //Render
    
    println!("P3\n{} {}\n255", image_width, image_height);

    for i in (0..image_height as i32).rev() {
        for  j in 0..image_width {
            let start = Vec3::new(vec!(0.0, 0.0, 0.0));
            let org = Vec3::new(vec!(0.0, 0.0, 0.0));
            let u = j as f64 / (image_width-1) as f64;
            let v = i as f64 / (image_height-1.0);
            let r = Ray::new(org, &lower_left_corner + &horizontal * u + &vertical * v - &start);
            let pixel_color = ray_color(&r);
            out_color(&pixel_color);
        }
    }   
}
