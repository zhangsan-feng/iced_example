use iced::{Border, Shadow};
use iced_core::{Background, Color, Theme, Vector};
use iced::widget::container::Style;
use crate::ui::styles::convert_grb;

pub fn style1() -> Box<dyn Fn(&Theme) -> Style> {
    Box::new(|_theme: &Theme| Style {
        text_color: Some(convert_grb(255.0, 255.0, 255.0)),

        background: Some(Background::Color(convert_grb(255.0, 255.0, 255.0))), 
        border: Border {
            color: Default::default(),
            width: 0.0,
            radius: Default::default(),
        },
        shadow: Shadow {
            color: convert_grb(255.0, 255.0, 255.0),
            offset: Vector { x: 1.0, y: 1.0 },
            blur_radius: 1.0,
        },
    })
}