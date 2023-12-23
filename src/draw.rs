use piston_window::types::{Color, Width};
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn convert_coordinate_f64(coordinate: i32) -> f64 {
    (coordinate as f64) * BLOCK_SIZE
    // type casting
}

pub fn convert_coordinate_u32(coordinate: i32) -> u32 {
    convert_coordinate_f64(coordinate) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = convert_coordinate_f64(x);
    let gui_y = convert_coordinate_f64(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = convert_coordinate_f64(x);
    let y = convert_coordinate_f64(y);
    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
