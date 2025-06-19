use std::path::PathBuf;
use iced::Element;
use iced::widget::{button, text, text_input, Column, Row};
use rfd::FileDialog;



#[derive(Debug, Clone)]
pub enum ModelSettingsMessage{
    ModelFileSelected,
    IndexFileSelected,
    IndexPathChanged(String),
}

pub struct ModelSettings {
    pub model_file: Option<PathBuf>,
    pub index_file: Option<PathBuf>,
    index_input_value: String,
}

impl ModelSettings {
    pub fn new() -> ModelSettings {
        ModelSettings {
            model_file: None,
            index_file: None,
            index_input_value: "".to_string(),
        }
    }
    
    pub fn update(&mut self, message: ModelSettingsMessage) {
        match message {
            ModelSettingsMessage::ModelFileSelected => {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.model_file = Option::from(path);
                }
            }
            ModelSettingsMessage::IndexFileSelected => {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.index_file = Option::from(path);
                }
            }
            ModelSettingsMessage::IndexPathChanged(text) => {
                self.index_input_value = text;
            }
        }
 

    }
    
    
    pub fn view(&self) -> Element<'_, ModelSettingsMessage> {
        let model_label = text_input(
            "模型文件路径",
            self.model_file
                .as_ref()
                .map(|p| p.to_str().unwrap_or(""))
                .unwrap_or(""),
        )
            .on_input(ModelSettingsMessage::IndexPathChanged)
            .on_paste(ModelSettingsMessage::IndexPathChanged);
        let model_file_btn = button("选择模型").on_press(ModelSettingsMessage::ModelFileSelected);
        
        let index_label = text_input(
            "索引文件路径",
            self.index_file
                .as_ref()
                .map(|p| p.to_str().unwrap_or(""))
                .unwrap_or(""),
        )
            .on_input(ModelSettingsMessage::IndexPathChanged)
            .on_paste(ModelSettingsMessage::IndexPathChanged);
        let index_file_btn = button("选择索引").on_press(ModelSettingsMessage::IndexFileSelected);
        
        Column::new()
            .push(
                Row::new()
                    .push(index_label)
                    .push(index_file_btn)
                    .spacing(10)
            ).spacing(10)
            .push(
                Row::new()
                    .push(model_label)
                    .push(model_file_btn)
                    .spacing(10)
            ).into()
    }
}