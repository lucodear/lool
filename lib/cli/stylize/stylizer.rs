use super::style::{Color, StyleAttributes as StyleBitflags};
use eyre::Result;

pub mod instructions {
    use super::{Result, StyleBitflags, Color};
    use bitflags::parser::{from_str, ParseError};
    
    pub struct StyledString {
        pub fg: Option<Color>,
        pub bg: Option<Color>,
        pub attrs: StyleBitflags,
    }

    impl Default for StyledString {
        fn default() -> Self {
            StyledString {
                fg: None,
                bg: None,
                attrs: StyleBitflags::empty(),
            }
        }
    }

    /// parses the instruction into a StyledString
    pub fn parse(instructions: &str) -> Result<StyledString> {
        let mut styled_string = StyledString::default();

        if instructions.starts_with("+") {
            // only attributes
            styled_string.attrs = attributes_from_str(instructions.trim_start_matches('+')).map_err(|e| eyre::eyre!(e))?;
            return Ok(styled_string);    
        }

        // try to separate the colors from the attributes
        let parts = instructions.split('+').collect::<Vec<_>>();

        match parts.len() {
            1 => {
                // no attributes
                let (fg, bg) = parse_color_instruction(parts[0].trim())?;
                styled_string.fg = fg;
                styled_string.bg = bg;
            }
            2 => {
                // attributes
                let (fg, bg) = parse_color_instruction(parts[0].trim())?;
                styled_string.fg = fg;
                styled_string.bg = bg;
                styled_string.attrs = attributes_from_str(parts[1]).map_err(|e| eyre::eyre!(e))?;
            }
            _ => {
                return Err(eyre::eyre!("Invalid instruction: {}", instructions));
            }
        };

        Ok(styled_string)
    }

    /// Parses the color instruction into a tuple of fg and bg colors.
    pub fn parse_color_instruction(instruction: &str) -> Result<(Option<Color>, Option<Color>)> {
        if instruction.starts_with("on ") {
            // If instruction starts with "on ", bg color is provided
            return Ok((None, Some(Color::from_str(&instruction[3..])?)));
        }

        let colors: Vec<&str> = instruction.split(" on ").collect();
        match colors.len() {
            1 => {
                // Only fg color is provided
                Ok((Some(Color::from_str(colors[0])?), None))
            }
            2 => {
                // Both fg and bg colors are provided
                Ok((Some(Color::from_str(colors[0])?), Some(Color::from_str(colors[1])?)))
            }
            _ => Err(eyre::eyre!("Invalid color instruction: {}", instruction)),
        }
    }

    fn attributes_from_str (s: &str) -> Result<StyleBitflags, ParseError> {
        from_str(s.to_uppercase().as_str())    
    }
}

/// 🧉 » Stylize Trait
/// --
/// 
/// `Trait` that extends `String` and `str` with the ability to stylize them
/// with ANSI color and attrs, using methods that return a new string with the given style.
pub trait Stylize {
    /// Basic styling method, receives a styling instruction
    /// see the `stylize` function for more information
    fn stl(&self, instruction: &str) -> String;
    fn black(&self) -> String { self.stl("black") }
    fn red(&self) -> String { self.stl("red") }
    fn green(&self) -> String { self.stl("green") }
    fn yellow(&self) -> String { self.stl("yellow") }
    fn blue(&self) -> String { self.stl("blue") }
    fn magenta(&self) -> String { self.stl("magenta") }
    fn cyan(&self) -> String { self.stl("cyan") }
    fn white(&self) -> String { self.stl("white") }
    fn bright_black(&self) -> String { self.stl("bright-black") }
    fn bright_red(&self) -> String { self.stl("bright-red") }
    fn bright_green(&self) -> String { self.stl("bright-green") }
    fn bright_yellow(&self) -> String { self.stl("bright-yellow") }
    fn bright_blue(&self) -> String { self.stl("bright-blue") }
    fn bright_magenta(&self) -> String { self.stl("bright-magenta") }
    fn bright_cyan(&self) -> String { self.stl("bright-cyan") }
    fn bright_white(&self) -> String { self.stl("bright-white") }
    fn rgb(&self, r: u8, g: u8, b: u8) -> String { self.stl(&format!("#{:02X}{:02X}{:02X}", r, g, b)) }
    fn on_black(&self) -> String { self.stl("on black") }
    fn on_red(&self) -> String { self.stl("on red") }
    fn on_green(&self) -> String { self.stl("on green") }
    fn on_yellow(&self) -> String { self.stl("on yellow") }
    fn on_blue(&self) -> String { self.stl("on blue") }
    fn on_magenta(&self) -> String { self.stl("on magenta") }
    fn on_cyan(&self) -> String { self.stl("on cyan") }
    fn on_white(&self) -> String { self.stl("on white") }
    fn on_bright_black(&self) -> String { self.stl("on bright-black") }
    fn on_bright_red(&self) -> String { self.stl("on bright-red") }
    fn on_bright_green(&self) -> String { self.stl("on bright-green") }
    fn on_bright_yellow(&self) -> String { self.stl("on bright-yellow") }
    fn on_bright_blue(&self) -> String { self.stl("on bright-blue") }
    fn on_bright_magenta(&self) -> String { self.stl("on bright-magenta") }
    fn on_bright_cyan(&self) -> String { self.stl("on bright-cyan") }
    fn on_bright_white(&self) -> String { self.stl("on bright-white") }
    fn on_rgb(&self, r: u8, g: u8, b: u8) -> String { self.stl(&format!("on #{:02X}{:02X}{:02X}", r, g, b)) }
    fn bold(&self) -> String { self.stl("+bold") }
    fn dim(&self) -> String { self.stl("+dim") }
    fn italic(&self) -> String { self.stl("+italic") }
    fn underline(&self) -> String { self.stl("+underline") }
    fn blink(&self) -> String { self.stl("+blink") }
    fn reverse(&self) -> String { self.stl("+reverse") }
    fn hidden(&self) -> String { self.stl("+hidden") }
    fn strikethrough(&self) -> String { self.stl("+strikethrough") }
} 

impl Stylize for str {
    fn stl(&self, instruction: &str) -> String {
        stylize(self, instruction)
    }
}

impl Stylize for String {
    fn stl(&self, instruction: &str) -> String {
        stylize(self, instruction)
    }
}

/// 🧉 » stylize fn
/// --
/// 
/// Stylizes a string with optional ANSI color and attributes.
/// 
pub fn stylize<S: AsRef<str>>(s: S, instructions: &str) -> String {
    let styled_string = instructions::parse(instructions);

    if styled_string.is_err() {
        return s.as_ref().to_string();
    }

    let styled_string = styled_string.unwrap();

    let mut formatted = String::new();

    if let Some(fg) = styled_string.fg {
        formatted.push_str(&format!("\x1b[{}m", fg.to_fg_str()));
    }

    if let Some(bg) = styled_string.bg {
        formatted.push_str(&format!("\x1b[{}m", bg.to_bg_str()));
    }

    if !styled_string.attrs.is_empty() {
        let ansi_codes = styled_string.attrs.to_ansi_codes();

        if !ansi_codes.is_empty() {
            formatted.push_str(&format!("\x1b[{}m", ansi_codes.join(";")));
        }
    }

    // Append the original string and clear the style
    formatted.push_str(s.as_ref());
    formatted.push_str("\x1b[0m");

    formatted
}
