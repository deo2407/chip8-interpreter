use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Sink, Source};
use std::time::Duration;

const tone: f32 = 600.0;

pub struct Beeper {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Option<Sink>
}

impl Beeper {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            _stream,
            stream_handle,
            sink: None
        }
    }
    
    pub fn start_beep(&mut self) {
        if self.sink.is_none() || self.sink.as_ref().unwrap().empty() {
            let sink = Sink::try_new(&self.stream_handle).unwrap();

            let source = SineWave::new(tone).amplify(0.2).repeat_infinite();
            sink.append(source);
            self.sink = Some(sink);
        }
    }

    pub fn stop_beep(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
        self.sink = None;
    }
}