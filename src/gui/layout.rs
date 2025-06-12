
use iced::widget::{button, column, container, row, text, Column, Container};
use iced::{Element, Length, Sandbox, };
use crate::gui::process_file::{ProcessFile, ProcessFileMessage};
use crate::gui::process_limit::{ProcessLimitMessage, ProcessLimit};

#[derive(Debug, Clone)]
pub enum Message {
    PageChange(usize),
    ProcessFile(ProcessFileMessage),
    ProcessLimit(ProcessLimitMessage)
}

pub struct ApplicationGui {
    pub current_page: usize,
    pub process_file_page: ProcessFile,
    pub process_limit_page: ProcessLimit,
}


impl Sandbox for ApplicationGui {
    type Message = Message;

    fn new() -> Self {
        ApplicationGui { 
            current_page: 0 , 
            process_file_page: ProcessFile::new() ,
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
            Message::ProcessFile(message) => {
                self.process_file_page.update(message);
            }
            Message::ProcessLimit(message) => {
                self.process_limit_page.update(message)
            }
        }
    }

    fn view(&self) -> Element<'static, Message> {
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
            ).width(Length::Fixed(150.0))
        ).width(Length::Fixed(150.0)).padding(3);

        
        let content = match self.current_page {
            0 => self.process_file_page.view().map(Message::ProcessFile),
            1 => self.process_limit_page.view().map(Message::ProcessLimit),
            3 => Column::new().push(Container::new(text("404 页面不存在"))).into(),
            _ => text("404 页面不存在").into(),
        };
            
        
        row![navigation, container(content).width(Length::Fill),]
            .spacing(20)
            .into()
    }
}

