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

#[derive(Debug, Clone)]
pub struct TrueColor(u8, u8, u8);

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
#[derive(Debug, Clone)]
pub struct FourBitColor(u8);

impl TryFrom<u8> for FourBitColor {
    type Error = InvalidStyleError;


    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (40..=47).contains(&value) || (100..=107).contains(&value) {
            // Background Color
            Ok(FourBitColor(value - 40))
        } else if (30..=37).contains(&value) || (90..=97).contains(&value) {
            // Normal Color
            Ok(FourBitColor(value - 30))
        } else if !value >= 7 {
            // Format wanted
            Ok(FourBitColor(value))
        } else {
            // Invalid Value
            Err(InvalidStyleError)
        }
    }
}

impl FromStr for FourBitColor {
    type Err = InvalidStyleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = u8::from_str(s).map_err(|_| InvalidStyleError)?;
        FourBitColor::try_from(result)
    }
}

impl Into<Color> for FourBitColor {
    fn into(self) -> Color {
        Color::FourBitColor(self)
    }
}

impl DisplayColor for FourBitColor {
    fn fmt_background(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{}m", 1, self.0 + 40)
    }
    fn fmt_color(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{}m", 1, self.0 + 30)
    }
}

impl Display for FourBitColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //Due to numbering differences. This doesn't work
        write!(f, "1;{}", self.0)
    }
}


