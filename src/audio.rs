use rodio::{Decoder, Sink};
use std::{fs::File, io::BufReader};

pub struct Audio {
    path: String,
    sink: Sink,
}

impl Audio {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        Self {
            path: path.to_owned(),
            sink,
        }
    }

    pub fn play(&mut self) {
        self.sink.play();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
}
