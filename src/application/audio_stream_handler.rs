use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


pub struct AudioStreamHandler {
    
}

impl AudioStreamHandler {
    pub fn new() -> AudioStreamHandler {
        AudioStreamHandler {}
    }
    pub fn update(&self){
        
    }
    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        let host = cpal::default_host();
        let device = host.default_input_device().expect("no input device available");
        let config = device.default_input_config().unwrap();
        let stream = device.build_input_stream(&config.into(), move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // 在这里处理音频数据
            // process_audio(data);
        }, |err| eprintln!("an error occurred on stream: {}", err), None)?;

        stream.play()?;

        Ok(())
    }
    pub fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}