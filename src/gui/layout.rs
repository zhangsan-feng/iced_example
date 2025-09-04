
use iced::widget::{button, column, container, row, text, Column, Container};
use iced::{Element, Length, Sandbox, Theme};
use crate::gui::rvc::rvc_layout::{RvcDataStore, RvcMessage};


#[derive(Debug, Clone)]
pub enum Message {
    PageChange(usize),
    RvcMsg(RvcMessage),

}

pub struct ApplicationGui {
    pub current_page: usize,
    pub rvc_page: RvcDataStore,

}


impl Sandbox for ApplicationGui {
    type Message = Message;

    fn new() -> Self {
        ApplicationGui { 
            current_page: 0 , 
            rvc_page: RvcDataStore::new() ,

        }
    }

    fn title(&self) -> String {
        String::from("Iced example")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PageChange(index) => {
                self.current_page = index;
            }
            Message::RvcMsg(message) => {
                self.rvc_page.update(message);
            }

        }
    }

    fn view(&self) -> Element<Message> {
        let nav_items = vec!["首页", "设置", "关于","404"];


        let navigation = container(
            column(
                nav_items
                    .iter()
                    .enumerate()
                    .map(|(i, &item)| {
                        button(text(item).size(20))
                            .width(Length::Fill)
                            .style(if i == self.current_page {
                                iced::theme::Button::Primary
                            } else {
                                iced::theme::Button::Secondary
                            })
                            .on_press(Message::PageChange(i))
                            .into()
                    }).collect(),
            )
        )
            .width(Length::Fixed(150.0))
            .height(Length::Fill)
            .style(crate::gui::style::container_style::custom_container_style_1)
            .padding(5);

        
        let content = match self.current_page {
            0 => self.rvc_page.view().map(Message::RvcMsg),
            1 => Column::new().push(Container::new(text("设置"))).into(),
            2 => Column::new().push(Container::new(text("关于"))).into(),
            _ => text("404 页面不存在").into(),
        };
            
        
        row![
            navigation,
            container(content)
            .padding(10)
            .width(Length::Fill)
        ]
            .spacing(0)
            .into()
            
    }
}

