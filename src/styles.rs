use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

pub static RESET: Style = Style(0);


/// Refers to to a ANSI Style code
/// Rendered VIA `\x1b[{u8}m`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Style(u8);

impl Display for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m", self.0)
    }
}

impl From<u8> for Style {
    fn from(v: u8) -> Self {
        Style(v)
    }
}

impl FromStr for Style {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = u8::from_str(s)?;
        Ok(Style(result))
    }
}