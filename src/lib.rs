pub mod no_color;
pub mod styles;
pub mod color;

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::color::{Color, DisplayColor};
use crate::styles::Style;

pub static SET_GRAPHIC: &str = "\x1b[{}m";
pub static SET_GRAPHIC_MULTI: &str = "\x1b[{};{}m";

#[derive(Debug)]
pub struct InvalidStyleError;

impl Display for InvalidStyleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "The Value provided is not compatiable with ANSI escape code standards")
    }
}

impl Error for InvalidStyleError {}

/// A Styled String.
#[derive(Debug, Clone)]
pub struct StyledString<'content> {
    /// Content of the String
    pub content: Cow<'content, str>,
    pub text_color: Option<Color>,
    pub background_color: Option<Color>,
    pub styles: Vec<Style>,
}

impl<'content> Display for StyledString<'content> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !no_color::show_styles() {
            return Display::fmt(&self.content, f);
        }
        for style in &self.styles {
            Display::fmt(&style, f)?;
        }
        if let Some(background) = self.background_color.as_ref() {
            background.fmt_background(f)?;
        }
        if let Some(color) = self.text_color.as_ref() {
            color.fmt_color(f)?;
        }
        write!(f, "{}{}", self.content, styles::RESET)
    }
}

impl<'content> StyledString<'content> {
    pub fn text_color<C: Into<Color>>(mut self, text: C) -> Self {
        self.text_color = Some(text.into());
        self
    }
    pub fn background_color<C: Into<Color>>(mut self, text: C) -> Self {
        self.background_color = Some(text.into());
        self
    }

    pub fn add_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.styles.push(style.into());
        self
    }
}

pub trait StyleString {
    fn style(&self) -> StyledString;
}

impl StyleString for String {
    fn style(&self) -> StyledString {
        StyledString {
            content: Cow::Borrowed(self),
            text_color: None,
            background_color: None,
            styles: vec![],
        }
    }
}

impl<'content> StyleString for &'content str {
    fn style(&self) -> StyledString {
        StyledString {
            content: Cow::Borrowed(self),
            text_color: None,
            background_color: None,
            styles: vec![],
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::fs::File;
    use crate::color::{EightBitColor, FourBitColor};
    use crate::StyleString;
    use std::io::Write;

    #[test]
    pub fn general_test() {
        let string = "Howdy".style().text_color(EightBitColor::from(9)).background_color(FourBitColor::try_from(97).unwrap()).add_style(4);
        println!("{}", string);
    }
}