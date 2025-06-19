use iced::Background::Color;
use iced::{Element, Length};
use iced::widget::{Checkbox, Column, Row, Slider, Text};

#[derive(Debug, Clone)]
pub enum PerformanceSettingsMessage {
    SliderChanged(usize, f32),
    CheckboxChanged(usize, bool),
}

pub struct PerformanceSettings {
    sample_length:f32,
    harvest_process:i8,
    crossfade_length:f32,
    extra_time:f32,
    
    pub slider_values: [f32; 5],
    pub checkbox_values: [bool; 5],
}

impl PerformanceSettings {
    pub fn new() -> Self{
        PerformanceSettings {
            sample_length: 0.0,
            harvest_process: 0,
            crossfade_length: 0.0,
            extra_time: 0.0,
            slider_values: [-60.0, 0.0, 0.0, 0.0, 0.0],
            checkbox_values: [false; 5],
        }
    }

    pub fn update(&mut self, message: PerformanceSettingsMessage) {
        match message {
            PerformanceSettingsMessage::SliderChanged(index, value) => {
                if index < self.slider_values.len() {
                    self.slider_values[index] = value;
                }
            }
            PerformanceSettingsMessage::CheckboxChanged(index, value) => {
                if index < self.checkbox_values.len() {
                    self.checkbox_values[index] = value;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, PerformanceSettingsMessage> {
        let controls = Column::new().spacing(10);

        let controls = (0..4).fold(controls, |column, i| {
            let label = Text::new(match i {
                0 => format!("采样长度: {:.2}", self.slider_values[i]),
                1 => format!("harvest进程数量: {:.2}", self.slider_values[i]),
                2 => format!("淡入淡出长度: {:.2}", self.slider_values[i]),
                3 => format!("额外推理时长: {:.2}", self.slider_values[i]),
                _ => "".to_string(),
            }).width(200);

            let slider = Slider::new(
                match i {
                    0 => -0.1..=2.0,
                    1 => 1.0..=8.0,
                    2 => 0.01..=1.15,
                    3 => 0.05..=5.00,
                    _ => 0.0..=100.0,
                },
                self.slider_values[i],
                move |v| PerformanceSettingsMessage::SliderChanged(i, v)
            ).step(0.01);

            column.push(Row::new()
                .push(label)
                .push(slider)
                .width(Length::Fill)
            )
        });
        let checkbox_row = (0..3).fold(
            Row::new().spacing(10).padding(10),
            |row, i| {
                row.push(
                    Row::new()
                        .push(Checkbox::new(
                            match i {
                                0 => "输入降噪",
                                1 => "输出降噪",
                                2 => "启动相位编码器",
                                _ => "",
                            },
                            self.checkbox_values[i],
                            move |v| PerformanceSettingsMessage::CheckboxChanged(i, v),
                        ))
                        .width(Length::Fill)
                )
            }
        );

        controls.push(checkbox_row).into()
    }
}
