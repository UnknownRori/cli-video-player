use std::fs::remove_file;

use clap::Parser;
use console::Term;

use crate::{
    args::Args,
    // audio::Audio,
    ffmpeg::{Ffmpeg, Size},
    video::Video,
};

pub struct App {
    args: Args,
    term: Term,
    size: Size,
}

impl App {
    pub fn new(term: Term, width: usize, height: usize) -> Self {
        Self {
            args: Args::parse(),
            term,
            size: Size { width, height },
        }
    }

    pub fn start(&self) -> color_eyre::Result<()> {
        let file = &self.args.input;

        self.term.write_line(
            "
    ██████╗ ██╗     ███████╗ █████╗ ███████╗███████╗    ██╗    ██╗ █████╗ ██╗████████╗
    ██╔══██╗██║     ██╔════╝██╔══██╗██╔════╝██╔════╝    ██║    ██║██╔══██╗██║╚══██╔══╝
    ██████╔╝██║     █████╗  ███████║███████╗█████╗      ██║ █╗ ██║███████║██║   ██║
    ██╔═══╝ ██║     ██╔══╝  ██╔══██║╚════██║██╔══╝      ██║███╗██║██╔══██║██║   ██║
    ██║     ███████╗███████╗██║  ██║███████║███████╗    ╚███╔███╔╝██║  ██║██║   ██║
    ╚═╝     ╚══════╝╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝     ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝   ╚═╝

                    ██╗    ██╗ █████╗ ██████╗ ███╗   ███╗██╗  ██╗   ██╗
                    ██║    ██║██╔══██╗██╔══██╗████╗ ████║██║  ╚██╗ ██╔╝
                    ██║ █╗ ██║███████║██████╔╝██╔████╔██║██║   ╚████╔╝
                    ██║███╗██║██╔══██║██╔══██╗██║╚██╔╝██║██║    ╚██╔╝
                    ╚███╔███╔╝██║  ██║██║  ██║██║ ╚═╝ ██║███████╗██║
                     ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚═╝

    ",
        )?;

        let mut video = Video::parse(file, self.size)?;
        let audio_out = "./output.wav";
        Ffmpeg::extract_audio(&file, audio_out)?;

        // INFO : Very stupid way
        // let mut audio = Audio::new("output.wav"); // make this depends on video;
        video.play(&self.term, audio_out, 30.5)?; // TODO : FPS should come from ffmpeg metadata

        // Do clean up
        remove_file(audio_out)?;
        Ok(())
    }
}
