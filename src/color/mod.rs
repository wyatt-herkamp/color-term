pub mod classic_term;
pub mod eight_bit;
pub mod true_color;

use std::fmt::{Display, Formatter};
pub use crate::color::classic_term::FourBitColor;
pub use crate::color::eight_bit::EightBitColor;
pub use crate::color::true_color::TrueColor;

/// Default for Colors.
/// By default they will convert into 8 Bit Color
/// If you would like them to auto convert into 4 bit color. Enable the feature `default_four_bit`
pub enum DefaultColor {
    /// 0 on 8 bit color
    Black,
    /// 0 on 8 bit color

    Red,
    /// 1 on 8 bit color

    Green,
    /// 2 on 8 bit color

    Yellow,
    /// 3 on 8 bit color

    Blue,
    /// 4 on 8 bit color

    Magenta,
    /// 5 on 8 bit color

    Cyan,
    /// 6 on 8 bit color

    White,
    /// 7 on 8 bit color

    Gray,
    /// 8 on 8 bit color

    BrightRed,
    /// 9 on 8 bit color

    BrightGreen,
    /// 10 on 8 bit color

    BrightYellow,
    /// 11 on 8 bit color

    BrightBlue,
    /// 12 on 8 bit color

    BrightMagenta,
    /// 13 on 8 bit color

    BrightCyan,
    /// 14 on 8 bit color

    BrightWhite,
}


/// Converts the type into a Color
/// For the default implementation to work. Display must be implemented with the return type that is compatible with `\x1b[{38 || 48};{VALUE}m`
///
/// **The functions below will not check for NO_COLOR automatically. It is recommended that you do this yourself**
pub trait DisplayColor: Display + Into<Color> {
    /// Formats the ansi escape code for the background
    /// On the True Color and Eight Bit it will be the Format `\x1b[48;{}m` on the four bit it will use the `\x1b[{}m`
    ///
    /// This is used when you want to push a direct write to the formatter. If you don't have a formatter. Use [background_color](DisplayColor::background_color)
    fn fmt_background(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[48;{}m", self)
    }
    /// Formats the ansi escape code for the normal color
    /// On the True Color and Eight Bit it will be the Format `\x1b[38;{}m` on the four bit it will use the `\x1b[{}m`
    ///
    /// This is used when you want to push a direct write to the formatter. If you don't have a formatter. Use [color](DisplayColor::color)
    fn fmt_color(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[38;{}m", self)
    }
    /// Creates the ANSI escape code as a string
    fn background_color(&self) -> String {
        format!("\x1b[48;{}m", self)
    }
    /// Creates the ANSI escape code as a string for the text color
    fn color(&self) -> String {
        format!("\x1b[38;{}m", self)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "color_type", content = "value"))]
/// The Color allowing for the different implementations where needed
pub enum Color {
    /// A True Color terminal color
    TrueColor(TrueColor),
    /// Eight Bit Color format
    EightBitColor(EightBitColor),
    /// Classic Four Bit Color format.
    FourBitColor(FourBitColor),
}


impl DisplayColor for Color {
    fn fmt_background(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::TrueColor(color) => { color.fmt_background(f) }
            Color::EightBitColor(color) => { color.fmt_background(f) }
            Color::FourBitColor(color) => { color.fmt_background(f) }
        }
    }
    fn fmt_color(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::TrueColor(color) => { color.fmt_color(f) }
            Color::EightBitColor(color) => { color.fmt_color(f) }
            Color::FourBitColor(color) => { color.fmt_color(f) }
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::TrueColor(color) => {
                color.fmt(f)
            }
            Color::EightBitColor(color) => {
                color.fmt(f)
            }
            Color::FourBitColor(color) => {
                color.fmt(f)
            }
        }
    }
}


