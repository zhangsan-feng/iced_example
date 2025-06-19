
use iced::widget::button;
use iced::{Background, Color, Theme};

pub struct ButtonStyle1;

impl button::StyleSheet for ButtonStyle1 {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(
                Color::from_rgb(85.0 / 255.0, 151.0 / 255.0, 90.0 / 255.0))),
            border_radius: 10.0.into(),
            border_width: 2.0,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            ..Default::default()
        }
    }

 
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(
                Color::from_rgb(85.0 / 255.0, 151.0 / 255.0, 90.0 / 255.0)
            )),  
            ..self.active(style)
        }
    }


    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_color:  Color::from_rgb(85.0 / 255.0, 151.0 / 255.0, 90.0 / 255.0),  
            ..self.hovered(style)
        }
    }
}