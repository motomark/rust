#![crate_name = "doc"]
use std::f32::consts::PI;

/// calculate the Area of a circle
/// 
/// 
fn calc_circle_area(radius: f32) -> f32 {
    let result = PI * radius * radius;
    return result;
}

fn main() {

   let rad = 4.5;
   let area = calc_circle_area(rad);
   println!("Area of circle with radius {:.2} is {:.2}", rad, area);
}
