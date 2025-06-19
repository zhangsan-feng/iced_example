
use iced::widget::scrollable;
use iced::{Color, Theme, BorderRadius, Background};
use iced::widget::scrollable::{Scrollbar, Scroller};


pub struct HiddenScrollbar;

impl scrollable::StyleSheet for HiddenScrollbar {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            border_radius: 50.0.into(),  
            border_width: 50.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: Color::from_rgb(0.3, 0.3, 0.7),
                border_radius: 50.0.into(),  
                border_width: 50.0,    
                border_color: Color::WHITE,
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        Scrollbar{
            background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            border_radius: 5.0.into(),
            border_width: 1.0,
            border_color: Color::BLACK,
            scroller: scrollable::Scroller {
                color: Color::from_rgb(0.5, 0.5, 0.7),
                border_radius: 5.0.into(),
                border_width: 1.0,
                border_color: Color::BLACK,
            },
        }
    }
}