use iced::{theme, Alignment, Element, Length};
use iced::widget::{Column, Checkbox, Slider, Text, Row, Space};

#[derive(Debug, Clone)]
pub enum GeneralSettingsMessage {
    SliderChanged(usize, f32),  
    CheckboxChanged(usize, bool),  
}

pub struct GeneralSettings {
    pub slider_values: [f32; 5],
    pub checkbox_values: [bool; 5],
}

impl GeneralSettings {
    pub fn new() -> Self {
        GeneralSettings {
            slider_values: [-60.0, 0.0, 0.0, 0.0, 0.0],
            checkbox_values: [false; 5],
        }
    }

    pub fn update(&mut self, message: GeneralSettingsMessage) {
        match message {
            GeneralSettingsMessage::SliderChanged(index, value) => {
                if index < self.slider_values.len() {
                    self.slider_values[index] = value;
                }
            }
            GeneralSettingsMessage::CheckboxChanged(index, value) => {
                if index < self.checkbox_values.len() {
                        if value {
                            for i in 0..self.checkbox_values.len() {
                                self.checkbox_values[i] = false;
                            }
                        }
                        self.checkbox_values[index] = value;
                }
            }
        }
    }

    pub fn view(&self) -> Element<GeneralSettingsMessage> {
        let controls = Column::new().spacing(10);

        let controls = (0..5).fold(controls, |column, i| {
            let label = Text::new(match i {
                0 => format!("响应阈值: {:.1}", self.slider_values[i]),
                1 => format!("音调设置: {:.1}", self.slider_values[i]),
                2 => format!("性别因子: {:.2}", self.slider_values[i]),
                3 => format!("索引特征: {:.2}", self.slider_values[i]),
                _ => format!("响度因子: {:.2}", self.slider_values[i]),
            }).width(200).size(14);

            let slider = Slider::new(
                match i {
                    0 => -60.0..=0.0,
                    1 => -16.0..=16.0,
                    2 => -2.00..=2.00,
                    3 => 0.00..=1.00,
                    _ => 0.00..=1.00,
                },
                self.slider_values[i],
                move |v| GeneralSettingsMessage::SliderChanged(i, v)
            ).step(0.01);

            column.push(Row::new()
                .push(label)
                .push(slider)
                .width(Length::Fill)
            )
        });

        let checkbox_row = (0..5).fold(
            Row::new(),
            |row, i| {
                row.push(
                    Row::new()
                        .push(Checkbox::new(
                            match i {
                                0 => "pm",
                                1 => "harvest",
                                2 => "crepe",
                                3 => "rmvpe",
                                4 => "fcpe",
                                _ => "",
                            },
                            self.checkbox_values[i],
                            move |v| GeneralSettingsMessage::CheckboxChanged(i, v),
                        ))
                        .width(Length::Fill)
                )
            }
        );

        controls.push(checkbox_row).into()
    }
}