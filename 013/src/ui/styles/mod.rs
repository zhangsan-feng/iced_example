use iced_core::Color;

pub mod container_style;


pub fn convert_grb(a: f32, b: f32, c: f32) -> Color {
    Color {
        r: a / 255.0, 
        g: b / 255.0,
        b: c / 255.0,
        a: 1.0, 
    }
}