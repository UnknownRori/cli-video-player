use itertools::Itertools;

use crate::{color::Color, utils::draw_ascii};

#[derive(Debug, Clone)]
pub struct Frame {
    pub buffer: Vec<Color>,
}

impl Frame {
    pub fn new(raw: Vec<Color>) -> Self {
        Self { buffer: raw }
    }
}

#[derive(Debug, Clone)]
pub struct ReadyFrame {
    pub buffer: Vec<String>,
}

impl From<Vec<Frame>> for ReadyFrame {
    fn from(frame_data: Vec<Frame>) -> Self {
        let buffer = frame_data
            .iter()
            .map(|frame| frame.buffer.iter().map(|rgb| draw_ascii(&rgb)).join(""))
            .collect::<Vec<_>>();

        Self { buffer }
    }
}
