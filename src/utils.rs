use crate::{color::Color, konst::ASCII_ART};

pub fn draw_ascii(rgb: &Color) -> char {
    let grayscale = rgb.grayscale();
    let index = (grayscale as usize * (ASCII_ART.len() - 1)) / 255;
    return ASCII_ART[index];
}
