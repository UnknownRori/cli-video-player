#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const fn from_bytes(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub fn grayscale(&self) -> usize {
        return (0.2126 * f64::from(self.red)
            + 0.7152 * f64::from(self.green)
            + 0.0722 * f64::from(self.blue))
        .round() as usize;
    }
}
