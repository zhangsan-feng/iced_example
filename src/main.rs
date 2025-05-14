

use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Font, Sandbox, Settings};
use iced::window;
use iced::window::{icon, Icon};

#[derive(Debug, Clone)]
pub enum Message {
    NavigationSelect(usize),
}

struct NavigationApp {
    selected_page: usize,
}

impl Sandbox for NavigationApp {
    type Message = Message;

    fn new() -> Self {
        NavigationApp { selected_page: 0 }
    }

    fn title(&self) -> String {
        String::from("Iced example")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NavigationSelect(index) => {
                self.selected_page = index;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let nav_items = vec!["首页", "设置", "关于"];
        
        // 创建导航栏
        let navigation = column(
            nav_items
                .iter()
                .enumerate()
                .map(|(i, &item)| {
                    button(text(item).size(20))
                        .padding(10)
                        .width(Length::Fill)
                        .style(if i == self.selected_page {
                            iced::theme::Button::Primary
                        } else {
                            iced::theme::Button::Secondary
                        })
                        .on_press(Message::NavigationSelect(i))
                        .into()
                })
                .collect(),
        )
        .spacing(5)
        .padding(20)
        .width(Length::Fixed(150.0));

        // 创建内容区域
        let content = match self.selected_page {
            0 => text("这是首页内容").size(24),
            1 => text("这是设置页面").size(24),
            2 => text("这是关于页面").size(24),
            _ => text("未知页面").size(24),
        };

        // 组合导航栏和内容
        let content = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        row![navigation, content]
            .spacing(20)
            .into()
    }
}

#[tokio::main]
async fn main() -> iced::Result  {
    let mut settings = Settings::default();
    settings.window.size = (800, 400);
    settings.window.position = window::Position::Centered;
    settings.default_font = Font::with_name("Microsoft YaHei");

    let icon_bytes = include_bytes!("icons/title_icon.png");
    let img = image::load_from_memory(icon_bytes).expect("REASON").into_rgba8(); // 转换为 RGBA8 格式
    let (width, height) = (img.width(), img.height());
    let rgba_data = img.into_raw();
    settings.window.icon = Some(
        Icon::from(icon::from_rgba(rgba_data, width, height).unwrap())
    );
    NavigationApp::run(settings)
}
