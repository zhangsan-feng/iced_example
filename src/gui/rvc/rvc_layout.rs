
use iced::widget::{button, container, pick_list, text, text_input, Column, PickList, Row, Text};
use iced::{Color, Element, Length, Theme};
use crate::application::audio_stream_handler;
use crate::gui::rvc::device_settings::{DeviceSettings, DeviceSettingsMessage};
use crate::gui::rvc::general_settings::{GeneralSettings, GeneralSettingsMessage};
use crate::gui::rvc::main_start::{MainStart, MainStartMessage};
use crate::gui::rvc::model_settings::{ModelSettings, ModelSettingsMessage};
use crate::gui::rvc::performance_settings::{PerformanceSettings,PerformanceSettingsMessage};




#[derive(Debug, Clone)]
pub enum RvcMessage {
    LoadModelMsg(ModelSettingsMessage),
    DeviceSettingsMsg(DeviceSettingsMessage),
    GeneralSettingsMsg(GeneralSettingsMessage),
    PerformanceSettingsMsg(PerformanceSettingsMessage),
    MainStartMessage(MainStartMessage),
}

pub struct RvcDataStore {
    model_settings: ModelSettings,
    device_settings: DeviceSettings,
    general_settings_message: GeneralSettings,
    performance_settings_message: PerformanceSettings,
    main_start: MainStart
}



impl RvcDataStore {
    pub fn new() -> Self {
        RvcDataStore {
            model_settings: ModelSettings::new(),
            device_settings: DeviceSettings::new(),
            general_settings_message: GeneralSettings::new(),
            performance_settings_message:PerformanceSettings::new(),
            main_start: MainStart::new()
        }
    }

    pub fn update(&mut self, message: RvcMessage) {
        match message {
            RvcMessage::DeviceSettingsMsg(message) => {
                self.device_settings.update(message);
            }
   
            RvcMessage::LoadModelMsg(message) => {
                self.model_settings.update(message)
            }
            RvcMessage::GeneralSettingsMsg(message) => {
                self.general_settings_message.update(message)
            }
            RvcMessage::PerformanceSettingsMsg(message) => {
                self.performance_settings_message.update(message)
                
            }
            RvcMessage::MainStartMessage(message) => {
                self.main_start.update(message);

                let sb = audio_stream_handler::AudioStreamHandler::new();
                
                // self.general_settings_message
                // self.performance_settings_message
                // self.device_settings
                // self.model_settings
                
                if self.main_start.main_state == true {

                }
                if self.main_start.main_state == false {

                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, RvcMessage> {
        
        
        Column::new()
            .push(
                Row::new()
                    .push(
                        container(
                            Column::new()
                                .push(self.model_settings.view().map(RvcMessage::LoadModelMsg))
                                .spacing(10)
                                .push(self.device_settings.view().map(RvcMessage::DeviceSettingsMsg))
                                .spacing(10),
                        )
                            .style(crate::gui::style::container_style::custom_container_style_1)
                            .padding(15)
                            .width(500),
                    )
                    .padding(iced::Padding { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 })
                
                    .push(
                        container(self.performance_settings_message.view().map(RvcMessage::PerformanceSettingsMsg))
                            .style(crate::gui::style::container_style::custom_container_style_1)
                            .padding(15)
                            .width(Length::Fill)
                    )
                    .spacing(10)
                    .padding(iced::Padding { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 }),
            )
            .push(
                Row::new()
                    .push( container(self.general_settings_message.view().map(RvcMessage::GeneralSettingsMsg))
                        .style(crate::gui::style::container_style::custom_container_style_1)
                        .padding(15)
                        .width(500)
                        // .height(Length::Fill)
                       )
            
                    .spacing(10)
                    .push( container(self.main_start.view().map(RvcMessage::MainStartMessage))
                        .style(crate::gui::style::container_style::custom_container_style_1)
                        .padding(15)
                        .height(Length::Fill)
                        .width(Length::Fill)).spacing(10)
            ).spacing(10)

            .into()
    }
}
