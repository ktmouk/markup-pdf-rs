use std::num::ParseIntError;

use printpdf::{Color, Rgb};

#[derive(Debug)]
pub struct HexColor {
    hex: String,
}

impl HexColor {
    pub fn new(hex: &str) -> Self {
        Self {
            hex: hex.to_string(),
        }
    }
}

fn to_percent(two_hex: &str) -> Result<f64, ParseIntError> {
    let value = i64::from_str_radix(two_hex, 16)?;
    Ok((value as f64) / 255.0)
}

impl From<HexColor> for Color {
    fn from(item: HexColor) -> Self {
        let r = to_percent(&item.hex[1..3]).unwrap_or(0.0);
        let g = to_percent(&item.hex[3..5]).unwrap_or(0.0);
        let b = to_percent(&item.hex[5..7]).unwrap_or(0.0);
        Color::Rgb(Rgb::new(r, g, b, None))
    }
}
