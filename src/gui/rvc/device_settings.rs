use cpal::traits::{DeviceTrait, HostTrait};
use iced::{Element, Length};
use iced::widget::{Column, PickList, Row, Text};


#[derive(Debug, Clone)]
pub enum DeviceSettingsMessage{
    InputDeviceSelected(String),
    OutputDeviceSelected(String),
    SelectDeviceType(String),
}


pub struct DeviceSettings {
    input_devices: Vec<String>,
    output_devices: Vec<String>,
    device_type: Vec<String>,
    
    pub selected_input: Option<String>,
    pub selected_output: Option<String>,
    pub select_device_type: Option<String>,
}


impl DeviceSettings{
    pub fn new() -> DeviceSettings{
        let host = cpal::default_host();
        let in_devices = host.input_devices();
        let out_devices = host.output_devices();
        // let hosts = cpal::available_hosts();
        // println!("可用的音频后端: {:?}", hosts);
        
        DeviceSettings {
            input_devices: in_devices.unwrap().map(|device| device.name().unwrap_or("Unknown Device".to_string())).collect(),
            output_devices: out_devices.unwrap().map(|device| device.name().unwrap_or("Unknown Device".to_string())).collect(),
            selected_input: None,
            selected_output: None,
            device_type: cpal::available_hosts().iter().map(|host_id| format!("{:?}", host_id)).collect(),
            select_device_type: None,
        }
    }

    pub fn update(&mut self, message:DeviceSettingsMessage){
        match message{
            DeviceSettingsMessage::InputDeviceSelected(device) => {
                self.selected_input = Some(device);
            }
            DeviceSettingsMessage::OutputDeviceSelected(device) => {
                self.selected_output = Some(device);
            }
            DeviceSettingsMessage::SelectDeviceType(device) => {
                self.select_device_type = Some(device);
            }
        }
    }

    pub fn view(&self)-> Element<'_, DeviceSettingsMessage>{
        let input_label = Text::new("Input:").width(60);
        let input_select = PickList::new(
            self.input_devices.clone(),
            self.selected_input.clone(),
            DeviceSettingsMessage::InputDeviceSelected,
        )
            .text_size(12.0)
            .width(Length::Fill);

        let output_label = Text::new("Output:").width(60);
        let output_select = PickList::new(
            self.output_devices.clone(),
            self.selected_output.clone(),
            DeviceSettingsMessage::OutputDeviceSelected,
        )
            .text_size(12.0)
            .width(Length::Fill);
        let device_label =  Text::new("device:").width(60);
        let device_select = PickList::new(
            self.device_type.clone(),
            self.select_device_type.clone(),
            DeviceSettingsMessage::SelectDeviceType,
        )
            .text_size(12.0)
            .width(Length::Fill);

        Column::new()
            .push(
                Row::new()
                    .push(input_label)
                    .push(input_select)
                    .spacing(10)
            )
            .spacing(10)
            .push(
                Row::new()
                    .push(output_label)
                    .push(output_select)
                    .spacing(10),
            )
            .push(
                Row::new()
                    .push(device_label)
                    .push(device_select)
                    .spacing(10),
            )
            .into()
    }
}