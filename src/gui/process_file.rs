use std::path::PathBuf;
use iced::widget::{button, text, Column};
use rfd::FileDialog;


#[derive(Debug, Clone)]
pub enum  ProcessFileMessage{
    FileSelected(Option<PathBuf>),
    FolderSelect(Option<PathBuf>)
}


pub struct ProcessFile {
    pub file_path: PathBuf,
    pub use_file_process:Vec<String>
    
}

impl ProcessFile {
    pub fn new() -> Self {
        ProcessFile {
            file_path: PathBuf::new(),
            use_file_process: Vec::new()
        }
    }
    

    pub fn update(&mut self, message: ProcessFileMessage){
        match message {
            ProcessFileMessage::FileSelected(_) => {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.file_path = path;
                }
            }
            ProcessFileMessage::FolderSelect(_)=>{
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.file_path = path;
                }
            }
        }
    }

    pub fn view(&self) -> iced::Element<'static, ProcessFileMessage> {
        
        let info = if self.file_path.is_file() {
            text(format!("选择的文件路径: {:?}", self.file_path))
        } else if self.file_path.is_dir() {
            text(format!("<UNK>: {:?}", self.file_path))
        }else {
            text("请选择文件".to_string())
            
        };
        let btn1 = button("选择文件").on_press(ProcessFileMessage::FileSelected(None));
        let btn2 = button("选择文件夹").on_press(ProcessFileMessage::FolderSelect(None));
        Column::new().push(info).push(btn1).push(btn2).into()
    }
}