mod gui;
mod win32api;
mod application;
mod config;

use iced::{Font, Sandbox, Settings};
use iced::window;
use iced::window::{icon, Icon};


#[tokio::main]
async fn main() -> iced::Result  {
    // config::logger::logger_init("./logs/").await;
    
    let mut settings = Settings::default();
    settings.window.size = (600, 400);
    settings.window.resizable = false;
    settings.window.transparent = true;
    settings.window.position = window::Position::Centered;
    settings.default_font = Font::with_name("Microsoft YaHei");

    let icon_bytes = include_bytes!("icons/title_icon.png");
    let img = image::load_from_memory(icon_bytes).expect("REASON").into_rgba8();
    let (width, height) = (img.width(), img.height());
    let rgba_data = img.into_raw();
    settings.window.icon = Some(
        Icon::from(icon::from_rgba(rgba_data, width, height).unwrap())
    );
    gui::layout::ApplicationGui::run(settings)
}