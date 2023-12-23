use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE:f64 = 25.0;

pub fn convert_coordinate_f64(coordinate: i32) -> f64{
    (coordinate as f64) * BLOCK_SIZE
    // type casting
}

pub fn convert_coordinate_u32(coordinate: i32) -> u32{
    convert_coordinate_f64(coordinate) as u32
}

pub fn draw_block(color:Color , x:i32 ,y:i32 , con:&Context, g:&mut G2d){
    let gui_x = convert_coordinate_f64(x);
    let gui_y = convert_coordinate_f64(y);

    
}

