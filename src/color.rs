use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::{InvalidStyleError};

/// Converts the type into a Color
/// For the default implementation to work. Display must be implemented with the return type that is compatible with \x1b[{38 || 48};{VALUE}m
pub trait DisplayColor: Display + Into<Color> {
    fn fmt_background(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[48;{}m", self)
    }
    fn fmt_color(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[38;{}m", self)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "color_type", content = "value"))]
pub enum Color {
    TrueColor(TrueColor),
    EightBitColor(EightBitColor),
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct TrueColor(u8, u8, u8);

impl From<(u8, u8, u8)> for TrueColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        TrueColor(r, g, b)
    }
}

impl Into<String> for TrueColor {
    fn into(self) -> String {
        format!("{};{};{}", self.0, self.1, self.2)
    }
}

impl Display for TrueColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "2;{};{};{}", self.0, self.1, self.2)
    }
}

impl Into<Color> for TrueColor {
    fn into(self) -> Color {
        Color::TrueColor(self)
    }
}

impl DisplayColor for TrueColor {}

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

impl DisplayColor for EightBitColor {}

impl Into<Color> for EightBitColor {
    fn into(self) -> Color {
        Color::EightBitColor(self)
    }
}

/// It is the data type as [EightBitColor] however, it renders a bit differently
/// Only use this if you want older system support
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct FourBitColor(u8);

impl From<u8> for FourBitColor {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl FromStr for FourBitColor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = u8::from_str(s)?;
        Ok(FourBitColor::from(result))
    }
}

impl Into<Color> for FourBitColor {
    fn into(self) -> Color {
        Color::FourBitColor(self)
    }
}

impl DisplayColor for FourBitColor {
    fn fmt_background(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[1;{}m", self.0)
    }
    fn fmt_color(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[1;{}m", self.0)
    }
}

impl Display for FourBitColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[1;{}m", self.0)
    }
}


