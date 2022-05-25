pub static NO_COLOR: &str = "NO_COLOR";
pub static TERM: &str = "TERM";
pub static NO_COLOR_TERMS: [&str; 2] = ["dumb", "xterm"];

/// Should the Display push display color
/// The default method is checking for the env variable or checking if the terminal supports it
#[cfg(not(feature = "disable_color"))]
pub fn show_styles() -> bool {
    if std::env::var(NO_COLOR).is_ok() {
        false
    } else if let Ok(value) = std::env::var(TERM) {
        !NO_COLOR_TERMS.contains(&value.as_str())
    } else {
        true
    }
}

/// Should the Display push display color
/// This version always says false.
#[cfg(feature = "disable_color")]
pub fn show_styles() -> bool {
    false
}