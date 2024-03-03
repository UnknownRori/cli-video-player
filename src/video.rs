use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use console::Term;
use rodio::{Decoder, Sink};

use crate::{
    // audio::Audio,
    ffmpeg::{Ffmpeg, Size},
    frame::{Frame, ReadyFrame},
};

#[allow(dead_code)]
struct FPSLimiter {
    pub target_fps: f32,
    pub frame_duration: Duration,
    pub last_frame_time: Instant,
}

impl FPSLimiter {
    fn new(target_fps: f32) -> Self {
        let frame_duration = Duration::from_secs_f32(1.0 / target_fps);
        let last_frame_time = Instant::now();
        FPSLimiter {
            target_fps,
            frame_duration,
            last_frame_time,
        }
    }

    fn wait(&mut self) {
        let now = Instant::now();
        let elapsed = now - self.last_frame_time;
        if elapsed < self.frame_duration {
            std::thread::sleep(self.frame_duration - elapsed);
        }
        self.last_frame_time = Instant::now();
    }
}

enum FrameState {
    ReadyFrame(ReadyFrame),
    NotReady(Vec<Frame>),
}

pub struct Video {
    // size: Size,
    frames: FrameState,
}

impl Video {
    pub fn parse(file: &str, size: Size) -> color_eyre::Result<Self> {
        let result = Ffmpeg::get_data(file, size)?;

        Ok(Self {
            // size: result.converted_size,
            frames: FrameState::NotReady(result.frames),
        })
    }

    pub fn ready(&mut self) {
        match &self.frames {
            FrameState::NotReady(frames) => {
                self.frames = FrameState::ReadyFrame(frames.clone().into());
            }
            _ => {}
        }
    }

    /// TODO : Maybe the fps will come from ffmpeg metadata again
    pub fn play(&mut self, term: &Term, audio: &str, target_fps: f32) -> color_eyre::Result<()> {
        self.ready();
        match &self.frames {
            FrameState::ReadyFrame(frames) => {
                // INFO : Put this somewhere and added wrapper
                let file = File::open(audio).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();
                sink.append(source);
                sink.pause();

                let mut fps_limiter = FPSLimiter::new(target_fps);
                for frame in frames.buffer.iter() {
                    term.clear_screen()?;
                    term.write_line(frame)?;

                    sink.play();

                    fps_limiter.wait();
                }
                sink.stop();
            }
            _ => panic!("Shit happen"),
        }
        Ok(())
    }
}
