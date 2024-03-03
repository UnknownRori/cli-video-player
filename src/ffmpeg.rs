/// Since I cannot interact with Ffmpeg from ffmpeg-next because my mingw can't install ffmpeg,
/// instead i resort to using precompiled binary
///
use std::process::{Command, Stdio};

use crate::{color::Color, frame::Frame};

pub struct Ffmpeg;

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone)]
pub struct FfmpegData {
    pub frames: Vec<Frame>,
    pub converted_size: Size,
}

impl Ffmpeg {
    pub fn get_data(path: &str, size: Size) -> color_eyre::Result<FfmpegData> {
        let output = Command::new("ffmpeg")
            .args(&[
                "-i",
                path,
                "-vf",
                // format!("scale=-1:{},select='between(n, 60,75)'", term_height).as_str(),
                format!("scale={}:{}", size.width, size.height - 1).as_str(),
                // "-vframes",
                // "30",
                "-f",
                "rawvideo",
                "-pix_fmt",
                "rgb24",
                "pipe:1",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        // println!("{stderr}");
        let mut video_width = 0;
        let mut video_height = 0;
        for line in stderr.lines() {
            // println!("{line} - ");
            if let Some(_) = line.find("Video:") {
                for a in line.split(", ") {
                    a.split(" ")
                        .filter(|str| match str.find("x") {
                            Some(_) => true,
                            None => false,
                        })
                        .for_each(|str| {
                            let a = str.split("x").collect::<Vec<_>>();
                            video_width = a.first().unwrap().parse::<usize>().unwrap_or(0);
                            video_height = a.last().unwrap().parse::<usize>().unwrap_or(0);
                        });
                }
            }
        }

        // TODO : Get metadata from ffmpeg and allocate with that frame count
        let mut frames: Vec<Frame> = Vec::new();
        let raw_pixels = output
            .stdout
            .chunks_exact(3)
            .map(|chunk| Color::from_bytes(chunk[0], chunk[1], chunk[2]))
            .collect::<Vec<_>>();
        // println!("{}", raw_pixels.len());

        for frame_raw in raw_pixels.chunks_exact(video_height * video_width) {
            frames.push(Frame::new(frame_raw.to_vec()));
        }

        Ok(FfmpegData {
            frames,
            converted_size: size,
        })
    }

    pub fn extract_audio(input: &str, output: &str) -> color_eyre::Result<()> {
        Command::new("ffmpeg")
            .args(&["-i", input, "-c:a", "pcm_s16le", output])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;
        Ok(())
    }
}
