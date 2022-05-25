use std::fmt::{Display, Formatter};
use crate::color::{Color, DisplayColor};

/// Only use this if you want older system support
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum FourBitColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

#[cfg(feature = "default_four_bit")]
use crate::color::DefaultColor;
#[cfg(feature = "default_four_bit")]
impl From<DefaultColor> for Color {
    fn from(value: DefaultColor) -> Self {
        let color = match value {
            DefaultColor::Black => { FourBitColor::Black }
            DefaultColor::Red => { FourBitColor::Black }
            DefaultColor::Green => { FourBitColor::Black }
            DefaultColor::Yellow => { FourBitColor::Black }
            DefaultColor::Blue => { FourBitColor::Black }
            DefaultColor::Magenta => { FourBitColor::Black }
            DefaultColor::Cyan => { FourBitColor::Black }
            DefaultColor::White => { FourBitColor::Black }
            DefaultColor::Gray => { FourBitColor::Black }
            DefaultColor::BrightRed => { FourBitColor::BrightRed }
            DefaultColor::BrightGreen => { FourBitColor::BrightGreen }
            DefaultColor::BrightYellow => { FourBitColor::BrightYellow }
            DefaultColor::BrightBlue => { FourBitColor::BrightBlue }
            DefaultColor::BrightMagenta => { FourBitColor::BrightMagenta }
            DefaultColor::BrightCyan => { FourBitColor::BrightCyan }
            DefaultColor::BrightWhite => { FourBitColor::BrightWhite }
        };
        Color::FourBitColor(color)
    }
}

impl From<FourBitColor> for Color {
    fn from(color: FourBitColor) -> Self {
        Color::FourBitColor(color)
    }
}

impl DisplayColor for FourBitColor {
    fn fmt_background(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color_code: u8 =
            match self {
                FourBitColor::Black => { 40 }
                FourBitColor::Red => { 41 }
                FourBitColor::Green => { 42 }
                FourBitColor::Yellow => { 43 }
                FourBitColor::Blue => { 44 }
                FourBitColor::Magenta => { 45 }
                FourBitColor::Cyan => { 46 }
                FourBitColor::White => { 47 }
                FourBitColor::Gray => { 100 }
                FourBitColor::BrightRed => { 101 }
                FourBitColor::BrightGreen => { 102 }
                FourBitColor::BrightYellow => { 103 }
                FourBitColor::BrightBlue => { 104 }
                FourBitColor::BrightMagenta => { 105 }
                FourBitColor::BrightCyan => { 106 }
                FourBitColor::BrightWhite => { 107 }
            };
        write!(f, "\x1b[1;{}m", color_code)
    }
    fn fmt_color(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color_code: u8 =
            match self {
                FourBitColor::Black => { 30 }
                FourBitColor::Red => { 31 }
                FourBitColor::Green => { 32 }
                FourBitColor::Yellow => { 33 }
                FourBitColor::Blue => { 34 }
                FourBitColor::Magenta => { 35 }
                FourBitColor::Cyan => { 36 }
                FourBitColor::White => { 37 }
                FourBitColor::Gray => { 90 }
                FourBitColor::BrightRed => { 91 }
                FourBitColor::BrightGreen => { 92 }
                FourBitColor::BrightYellow => { 93 }
                FourBitColor::BrightBlue => { 94 }
                FourBitColor::BrightMagenta => { 95 }
                FourBitColor::BrightCyan => { 96 }
                FourBitColor::BrightWhite => { 97 }
            };
        write!(f, "\x1b[1;{}m", color_code)
    }
}

impl Display for FourBitColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_color(f)
    }
}


