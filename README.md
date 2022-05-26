# style-term

A simple-to-use ANSI terminal style tool for Rust

## Features

Supports [No Color](https://no-color.org/) on the high level display methods

## How to use

```rust
use style_term::{Color, DefaultColor, EightBitColor, StyleString, TrueColor};

fn main() {
    println!("{}", "How are you doing".style().text_color(DefaultColor::Red))
}
```