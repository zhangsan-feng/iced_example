
use iced::widget::{button, column, container, row, text, Column, Container};
use iced::{Element, Length, Sandbox, Theme};
use crate::gui::rvc::rvc_layout::{RvcDataStore, RvcMessage};
use crate::gui::process_limit::{ProcessLimitMessage, ProcessLimit};

#[derive(Debug, Clone)]
pub enum Message {
    PageChange(usize),
    RvcMsg(RvcMessage),
    ProcessLimit(ProcessLimitMessage)
}

pub struct ApplicationGui {
    pub current_page: usize,
    pub rvc_page: RvcDataStore,
    pub process_limit_page: ProcessLimit,
}


impl Sandbox for ApplicationGui {
    type Message = Message;

    fn new() -> Self {
        ApplicationGui { 
            current_page: 0 , 
            rvc_page: RvcDataStore::new() ,
            process_limit_page: ProcessLimit::new() ,
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
            Message::ProcessLimit(message) => {
                self.process_limit_page.update(message)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let nav_items = vec!["首页", "设置", "关于"];


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
            1 => self.process_limit_page.view().map(Message::ProcessLimit),
            3 => Column::new().push(Container::new(text("404 页面不存在"))).into(),
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

