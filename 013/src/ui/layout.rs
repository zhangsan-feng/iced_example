
use iced::widget::{button, column, container, row, text, Column, Container};
use iced::{Element, Length, Theme};
use iced::widget::button::Status;


#[derive(Debug, Clone)]
pub enum Message {
    PageChange(usize),
}

pub struct ApplicationGui {
    pub current_page: usize,


}

impl Default for ApplicationGui {
    fn default() -> Self {
        ApplicationGui {
            current_page: 0 ,
        }
    }
}

impl ApplicationGui {

    fn title(&self) -> String {
        String::from("Iced example")
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::PageChange(index) => {
                self.current_page = index;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let nav_items = vec!["首页", "设置", "关于","404"];


        let navigation = container(
            column(
                nav_items
                    .iter()
                    .enumerate()
                    .map(|(i, &item)| {
                        button(text(item).size(20))
                            .width(Length::Fill)
                            .style(move |theme: &Theme, status: Status| {
                                if i == self.current_page {
                                    button::primary(theme, status)
                                } else {
                                    button::secondary(theme, status)
                                }
                            })
                            .on_press(Message::PageChange(i))
                            .into()
                    }).collect::<Vec<_>>(),
            )
        )
            .width(Length::Fixed(200.0))
            .height(Length::Fill)
            .style(crate::ui::styles::container_style::style1())
            .padding(5);


        let content = match self.current_page {
            1 =>  column![row![
                text("Page 1"),
            ]],
            2 => column![ text("Page 2"),],
            _ => column![ text("Page 3"),],
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

