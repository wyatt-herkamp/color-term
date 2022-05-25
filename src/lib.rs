pub mod no_color;
pub mod styles;
pub mod color;

use std::borrow::{Cow};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::styles::Style;
pub use crate::color::{DefaultColor, Color, DisplayColor, EightBitColor, TrueColor};

pub static SET_GRAPHIC: &str = "\x1b[{}m";
pub static SET_GRAPHIC_MULTI: &str = "\x1b[{};{}m";


/// A Styled String.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct Styles {
    pub text_color: Option<Color>,
    pub background_color: Option<Color>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub styles: Vec<Style>,
}


#[derive(Debug)]
pub struct InvalidStyleError;

impl Display for InvalidStyleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "The Value provided is not compatible with ANSI escape code standards")
    }
}

impl Error for InvalidStyleError {}

/// A Styled String.
#[derive(Debug, Clone)]
pub struct StyledString<'content> {
    /// Content of the String
    content: &'content str,
    styles: Cow<'content, Styles>,
}

impl<'content> Display for StyledString<'content> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !no_color::show_styles() {
            return Display::fmt(&self.content, f);
        }
        for style in &self.styles.styles {
            Display::fmt(&style, f)?;
        }
        if let Some(background) = self.styles.background_color.as_ref() {
            background.fmt_background(f)?;
        }
        if let Some(color) = self.styles.text_color.as_ref() {
            color.fmt_color(f)?;
        }
        write!(f, "{}{}", self.content, styles::RESET)
    }
}

impl<'content> StyledString<'content> {
    pub(crate) fn new(content: &'content str) -> StyledString {
        StyledString {
            content,
            styles: Cow::Owned(Styles::default()),
        }
    }
    pub fn text_color<C: Into<Color>>(mut self, text: C) -> Self {
        self.styles.to_mut().text_color = Some(text.into());
        self
    }
    pub fn background_color<C: Into<Color>>(mut self, text: C) -> Self {
        self.styles.to_mut().background_color = Some(text.into());
        self
    }

    pub fn add_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.styles.to_mut().styles.push(style.into());
        self
    }
}

pub trait StyleString {
    /// Converts the current type into a Styled String
    fn style(&self) -> StyledString;

    /// Creates a styled string with the applied settings
    fn apply_styles<'content>(&'content self, styles: &'content Styles) -> StyledString<'content> {
        let mut styled = self.style();
        styled.styles = Cow::Borrowed(styles);
        styled
    }
}

impl StyleString for String {
    fn style(&self) -> StyledString {
        StyledString::new(self.as_str())
    }
}

impl<'content> StyleString for &'content String {
    fn style(&self) -> StyledString<'content> {
        StyledString::new(self.as_str())
    }
}

impl<'content> StyleString for &'content str {
    fn style(&self) -> StyledString {
        StyledString::new(self)
    }
}


#[cfg(test)]
pub mod test {
    use std::fs::File;
    use crate::{Color, DefaultColor, EightBitColor, StyleString, TrueColor};
    use std::io::Write;
    use crate::color::{DefaultColor, EightBitColor, FourBitColor, TrueColor};

    #[test]
    pub fn general_test() {
        let string = "Howdy".style().text_color(DefaultColor::BrightBlue).background_color(FourBitColor::Black).add_style(4);
        println!("{}", string);
    }

    #[test]
    pub fn serde_test() {
        let true_color = Color::TrueColor(TrueColor::from((5, 5, 5)));
        let eight_bit_color = Color::EightBitColor(EightBitColor::from(5));
        let four_bit_color = Color::FourBitColor(FourBitColor::Black);
        println!("true_color: {}", serde_json::to_string_pretty(&true_color).unwrap());
        println!("eight_bit_color: {}", serde_json::to_string_pretty(&eight_bit_color).unwrap());
        println!("for_bit_color: {}", serde_json::to_string_pretty(&four_bit_color).unwrap());
    }
}