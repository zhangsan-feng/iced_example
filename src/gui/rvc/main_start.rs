
use iced::{Alignment, Element, Length};
use iced::theme::{Button, Scrollable};
use iced::widget::{button, container, scrollable, text, Checkbox, Column, Row, Text};


#[derive(Debug, Clone)]
pub enum MainStartMessage{
    ChangeState,
    IsListen,
    IsOutPut
}

pub struct MainStart{
    pub main_state:bool,
    pub is_listen:bool,
    pub is_output:bool
}

impl MainStart{
    pub fn new()->Self{
        MainStart{
            main_state:false,
            is_listen:false,
            is_output:false
        }
    }
    pub fn update(&mut self, message: MainStartMessage){
        match message {
            MainStartMessage::ChangeState=>{
         
                self.main_state=!self.main_state;
                println!("{}",self.main_state);
            }
            MainStartMessage::IsListen=>{
                self.is_listen=!self.is_listen;
            }
            MainStartMessage::IsOutPut=>{
                self.is_output=!self.is_output;
            }
        }
    }
    
    pub fn view(&self) -> Element<'_, MainStartMessage> {
      
        let button_text = Text::new(if self.main_state { "停止" } else { "开始" })
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center);
        
        let main_btn = button(button_text)
            .on_press(MainStartMessage::ChangeState)
            .width(100).height(50)
            .style(Button::Custom(Box::new(crate::gui::style::button_style::ButtonStyle1)));
        
        let box1 = Checkbox::new("输入监听", self.is_listen, |_| MainStartMessage::IsListen);
        let box2 = Checkbox::new("输出变声", self.is_output, |_| MainStartMessage::IsOutPut);
        
        
        Column::new()
            .push(
                Row::new().align_items(Alignment::Center)
                    .push(main_btn).spacing(30)
                    .push(box1)
                    .push(box2)
            )
            .push( 
                container(
                    scrollable(text("11111111111")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .size(14)
                    ).style(Scrollable::Custom(Box::new(crate::gui::style::scrollable_style::HiddenScrollbar)))
                )
                    .style(crate::gui::style::container_style::custom_container_style_2)
                    .padding(15)
            ).spacing(10).padding(10)
            .into()
    }
}