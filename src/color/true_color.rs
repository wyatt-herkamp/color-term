use std::fmt::{Display, Formatter};
use crate::color::{Color, DisplayColor};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
/// Uses a R G B value
/// Stored a tuple of (u8, u8, u8)
///
/// You can create one via TrueColor::from((r,g,b))
pub struct TrueColor(u8, u8, u8);

impl From<(u8, u8, u8)> for TrueColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        TrueColor(r, g, b)
    }
}

impl Display for TrueColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "2;{};{};{}", self.0, self.1, self.2)
    }
}


impl DisplayColor for TrueColor {}

impl From<TrueColor> for Color {
    fn from(color: TrueColor) -> Self {
        Color::TrueColor(color)
    }
}