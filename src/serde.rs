use serde::{Deserialize, Serialize};
use crate::{Color, Style, StyledString, StyleString};

/// A Styled String.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Styles {
    pub text_color: Option<Color>,
    pub background_color: Option<Color>,
    #[serde(default)]
    pub styles: Vec<Style>,
}


impl<'content> StyledString<'content> {
    /// Sets the styles in the config to the current value
    /// Consumes the config
    pub fn set_styles_from_config(mut self, value: Styles) -> StyledString<'content> {
        self.styles = value.styles;
        self.background_color = value.background_color;
        self.text_color = value.text_color;
        self
    }
}