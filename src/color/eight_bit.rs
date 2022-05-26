use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::color::{Color, DisplayColor};
#[cfg(not(feature = "default_four_bit"))]
use crate::DefaultColor;

/// An 8 Bit color Uses values 0-255
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct EightBitColor(u8);


impl From<u8> for EightBitColor {
    fn from(v: u8) -> Self {
        EightBitColor(v)
    }
}

impl FromStr for EightBitColor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = u8::from_str(s)?;
        Ok(EightBitColor(result))
    }
}

impl Display for EightBitColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "5;{}", self.0)
    }
}

#[cfg(not(feature = "default_four_bit"))]
impl From<DefaultColor> for Color {
    fn from(value: DefaultColor) -> Self {
        let value = match value {
            DefaultColor::Black => { 0 }
            DefaultColor::Red => { 1 }
            DefaultColor::Green => { 2 }
            DefaultColor::Yellow => { 3 }
            DefaultColor::Blue => { 4 }
            DefaultColor::Magenta => { 5 }
            DefaultColor::Cyan => { 6 }
            DefaultColor::White => { 7 }
            DefaultColor::Gray => { 8 }
            DefaultColor::BrightRed => { 9 }
            DefaultColor::BrightGreen => { 10 }
            DefaultColor::BrightYellow => { 11 }
            DefaultColor::BrightBlue => { 12 }
            DefaultColor::BrightMagenta => { 13 }
            DefaultColor::BrightCyan => { 14 }
            DefaultColor::BrightWhite => { 15 }
        };
        Color::EightBitColor(EightBitColor(value))
    }
}


impl DisplayColor for EightBitColor {}


impl From<EightBitColor> for Color {
    fn from(color: EightBitColor) -> Self {
        Color::EightBitColor(color)
    }
}
