use cli_video_player::app::App;
use console::Term;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let term = Term::stdout();
    let (term_width, term_height) = term_size::dimensions().unwrap_or((80, 24));
    App::new(term, term_width, term_height).start()?;
    Ok(())
}

// use console::Term;
// use itertools::Itertools;
//
// use std::{
//     process::{Command, Stdio},
//     time::{Duration, Instant},
// };
//
// #[derive(Debug, Clone)]
// struct Frame {
//     buffer: Vec<RGB>,
// }
//
// impl Frame {
//     pub fn new(raw: Vec<RGB>) -> Self {
//         Self { buffer: raw }
//     }
// }
//
// #[derive(Debug, Copy, Clone)]
// struct RGB {
//     red: u8,
//     green: u8,
//     blue: u8,
// }
//
// impl RGB {
//     // A constructor function to create an RGB instance from raw bytes
//     pub fn from_bytes(red: u8, green: u8, blue: u8) -> Self {
//         RGB { red, green, blue }
//     }
// }
//
// const ASCII_ART: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
// fn draw_ascii(rgb: &RGB) -> char {
//     let grayscale = (0.2126 * f64::from(rgb.red)
//         + 0.7152 * f64::from(rgb.green)
//         + 0.0722 * f64::from(rgb.blue))
//     .round() as usize;
//     let index = (grayscale as usize * (ASCII_ART.len() - 1)) / 255;
//     return ASCII_ART[index];
// }
//
// fn main() -> color_eyre::Result<()> {
//     color_eyre::install()?;
//     let term = Term::stdout();
//     let (term_width, term_height) = term_size::dimensions().unwrap_or((80, 24));
//     term.write_line(
//         "
// ██████╗ ██╗     ███████╗ █████╗ ███████╗███████╗    ██╗    ██╗ █████╗ ██╗████████╗
// ██╔══██╗██║     ██╔════╝██╔══██╗██╔════╝██╔════╝    ██║    ██║██╔══██╗██║╚══██╔══╝
// ██████╔╝██║     █████╗  ███████║███████╗█████╗      ██║ █╗ ██║███████║██║   ██║
// ██╔═══╝ ██║     ██╔══╝  ██╔══██║╚════██║██╔══╝      ██║███╗██║██╔══██║██║   ██║
// ██║     ███████╗███████╗██║  ██║███████║███████╗    ╚███╔███╔╝██║  ██║██║   ██║
// ╚═╝     ╚══════╝╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝     ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝   ╚═╝
//
//                 ██╗    ██╗ █████╗ ██████╗ ███╗   ███╗██╗  ██╗   ██╗
//                 ██║    ██║██╔══██╗██╔══██╗████╗ ████║██║  ╚██╗ ██╔╝
//                 ██║ █╗ ██║███████║██████╔╝██╔████╔██║██║   ╚████╔╝
//                 ██║███╗██║██╔══██║██╔══██╗██║╚██╔╝██║██║    ╚██╔╝
//                 ╚███╔███╔╝██║  ██║██║  ██║██║ ╚═╝ ██║███████╗██║
//                  ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚═╝
//
// ",
//     )?;
//
//     // ffmpeg -i ./bad-apple.mp4 -vf "scale=-1:34,fps=1,select='between(t,50,51)'" -f rawvideo -pix_fmt rgb24 pipe:1
//     let output = Command::new("ffmpeg")
//         .args(&[
//             "-i",
//             "./bad-apple.mp4",
//             "-vf",
//             // format!("scale=-1:{},select='between(n, 60,75)'", term_height).as_str(),
//             format!("scale={}:{}", term_width, term_height - 1).as_str(),
//             // "-vframes",
//             // "1",
//             "-f",
//             "rawvideo",
//             "-pix_fmt",
//             "rgb24",
//             "pipe:1",
//         ])
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .output()?;
//
//     let stderr = String::from_utf8_lossy(&output.stderr);
//     let mut video_width = 0;
//     let mut video_height = 0;
//     for line in stderr.lines() {
//         // println!("{line} - ");
//         if let Some(_) = line.find("Video:") {
//             for a in line.split(", ") {
//                 a.split(" ")
//                     .filter(|str| match str.find("x") {
//                         Some(_) => true,
//                         None => false,
//                     })
//                     .for_each(|str| {
//                         let a = str.split("x").collect::<Vec<_>>();
//                         video_width = a.first().unwrap().parse::<usize>().unwrap_or(0);
//                         video_height = a.last().unwrap().parse::<usize>().unwrap_or(0);
//                     });
//             }
//         }
//     }
//
//     // println!("{video_width}x{video_height}");
//
//     let mut frame_data: Vec<Frame> = Vec::with_capacity(video_height);
//     // println!("{}", output.stdout.len());
//     let pixels = output
//         .stdout
//         .chunks_exact(3)
//         .map(|chunk| RGB::from_bytes(chunk[0], chunk[1], chunk[2]))
//         .collect::<Vec<_>>();
//     // println!("pixels len: {}", pixels.len());
//     //
//     // println!("frame len: {}", frame_data.len());
//     // println!("{:#?}", frame_data);
//
//     // println!(
//     //     "Total per frame: {}",
//     //     pixels.len() / (video_height * video_width)
//     // );
//     for frame_raw in pixels.chunks_exact(video_height * video_width) {
//         frame_data.push(Frame::new(frame_raw.to_vec()));
//     }
//     // println!("frame_data :{:#?} ", frame_data.len());
//     // std::thread::sleep(Duration::from_millis(1000));
//     let ready_frame = frame_data
//         .iter()
//         .map(|frame| frame.buffer.iter().map(|rgb| draw_ascii(&rgb)).join(""))
//         .collect::<Vec<_>>();
//
//     let target_fps = 30;
//     let frame_duration = Duration::from_secs(1) / target_fps;
//     let mut last_frame_time = Instant::now();
//     for frame in ready_frame.iter() {
//         let frame_time = last_frame_time.elapsed();
//         term.write_line(frame)?;
//         // Calculate time spent rendering the frame
//
//         // Sleep to maintain frame rate
//         if frame_time < frame_duration {
//             let sleep_time = frame_duration - frame_time;
//             std::thread::sleep(sleep_time);
//         }
//
//         // Update last frame time
//         term.clear_screen()?;
//         last_frame_time = Instant::now();
//     }
//     Ok(())
// }
