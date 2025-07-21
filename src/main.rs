mod gui;
mod win32api;
mod application;
mod config;

use std::env;
use iced::{Font, Sandbox, Settings};
use iced::window;
use iced::window::{icon, Icon};


#[tokio::main]
async fn main() -> iced::Result  {
    // https://github.com/fogarecious/iced_tutorial/blob/main/README.md
    config::logger::logger_init(env::current_dir().unwrap().display().to_string()).await;
    
    match env::current_dir() {
        Ok(path) => {
            println!("当前工作目录是: {}", path.display());
        }
        Err(e) => {
            eprintln!("无法获取当前工作目录: {}", e);
        }
    }
    let mut settings = Settings::default();
    settings.window.size = (1200, 600);
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