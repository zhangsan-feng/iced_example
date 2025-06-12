use iced::widget::{text, Column};

#[derive(Debug, Clone)]
pub enum ProcessLimitMessage {}
pub struct ProcessLimit {}
impl ProcessLimit {
    
    pub fn new() -> Self {
        ProcessLimit {}
    }
    pub fn update(&mut self, message: ProcessLimitMessage) {}
    pub fn view(&self) -> iced::Element<'static, ProcessLimitMessage> {
        
        
        Column::new().push(text("11111")).into()
    }
}