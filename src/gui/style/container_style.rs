use iced::{Color, Theme};
use iced::widget::container;


pub fn custom_container_style_1(theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(220.0 / 255.0, 221.0 / 255.0, 224.0 / 255.0).into()),
        border_radius: 5.0.into(),
        border_width: 2.0,
        border_color: Color::BLACK,
        text_color: None,
    }
}

pub fn custom_container_style_2(theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(1.0, 1.0, 1.0).into()),
        border_radius: 5.0.into(),
        border_width: 2.0,
        border_color: Color::BLACK,
        text_color: None,
    }
}
