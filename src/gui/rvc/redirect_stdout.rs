use iced::widget::{button, column, scrollable, text};
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct OutputCapture {
    buffer: Arc<Mutex<String>>,
}

impl Write for OutputCapture {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let output = String::from_utf8_lossy(buf).to_string();
        self.buffer.lock().unwrap().push_str(&output);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Message {
    OutputUpdated(String),
    GenerateOutput,
    Tick,
}

struct OutputViewer {
    captured_output: String,
    output_buffer: Arc<Mutex<String>>,
}

impl Application for OutputViewer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let buffer = Arc::new(Mutex::new(String::new()));

        // 重定向标准输出
        let capture = OutputCapture {
            buffer: Arc::clone(&buffer),
        };
        let captured = Box::new(capture);
        let leaked = Box::leak(captured);
 

        (
            OutputViewer {
                captured_output: String::new(),
                output_buffer: buffer,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Output Viewer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OutputUpdated(new_output) => {
                self.captured_output = new_output;
            }
            Message::GenerateOutput => {
                println!("这是通过println输出的内容");
                println!("当前时间: {:?}", std::time::SystemTime::now());
            }
            Message::Tick => {
                let new_content = self.output_buffer.lock().unwrap().clone();
                if new_content != self.captured_output {
                    return Command::perform(async { new_content }, Message::OutputUpdated);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            button("生成输出").on_press(Message::GenerateOutput),

        ]
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .into()
    }

}

