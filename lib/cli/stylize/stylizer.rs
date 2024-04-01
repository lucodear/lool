use {
    super::style::{Color, StyleAttributes as StyleBitflags},
    eyre::Result,
};

pub mod instructions {
    use {
        super::{Color, Result, StyleBitflags},
        bitflags::parser::{from_str, ParseError},
    };

    /// struct that holds the style information
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

    /// parses the instruction into a `StyledString`
    pub fn parse(instructions: &str) -> Result<StyledString> {
        let mut styled_string = StyledString::default();

        if instructions.starts_with('+') {
            // only attributes
            styled_string.attrs = attributes_from_str(instructions.trim_start_matches('+'))
                .map_err(|e| eyre::eyre!(e))?;
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

    /// parses the color part of an instruction into a tuple of fg and bg colors.
    pub fn parse_color_instruction(instruction: &str) -> Result<(Option<Color>, Option<Color>)> {
        if let Some(prefix) = instruction.strip_prefix("on ") {
            return Ok((None, Some(Color::from_str(prefix)?)));
        }

        let colors: Vec<&str> = instruction.split(" on ").collect();
        match colors.len() {
            1 => {
                // Only fg color is provided
                Ok((Some(Color::from_str(colors[0])?), None))
            }
            2 => {
                // Both fg and bg colors are provided
                Ok((
                    Some(Color::from_str(colors[0])?),
                    Some(Color::from_str(colors[1])?),
                ))
            }
            _ => Err(eyre::eyre!("Invalid color instruction: {}", instruction)),
        }
    }

    fn attributes_from_str(s: &str) -> Result<StyleBitflags, ParseError> {
        from_str(s.to_uppercase().as_str())
    }
}

/// 🧉 » Stylize Trait
/// --
///
/// `Trait` that extends `String` and `str` with the ability to stylize them
/// with ANSI color and attrs, using methods that return a new string with the given style.
#[rustfmt::skip]
pub trait Stylize {
    /// Basic styling method, receives a styling instruction
    /// see the `stylize` function for more information
    fn stl(&self, instruction: &str) -> String;
    /// 🧉 » makes the text **black**
    fn black(&self) -> String { self.stl("black") }
    /// 🧉 » makes the text **red**
    fn red(&self) -> String { self.stl("red") }
    /// 🧉 » makes the text **green**
    fn green(&self) -> String { self.stl("green") }
    /// 🧉 » makes the text **yellow**
    fn yellow(&self) -> String { self.stl("yellow") }
    /// 🧉 » makes the text **blue**
    fn blue(&self) -> String { self.stl("blue") }
    /// 🧉 » makes the text **magenta**
    fn magenta(&self) -> String { self.stl("magenta") }
    /// 🧉 » makes the text **cyan**
    fn cyan(&self) -> String { self.stl("cyan") }
    /// 🧉 » makes the text **white**
    fn white(&self) -> String { self.stl("white") }
    /// 🧉 » makes the text **bright black**
    fn bright_black(&self) -> String { self.stl("bright-black") }
    /// 🧉 » makes the text **bright red**
    fn bright_red(&self) -> String { self.stl("bright-red") }
    /// 🧉 » makes the text **bright green**
    fn bright_green(&self) -> String { self.stl("bright-green") }
    /// 🧉 » makes the text **bright yellow**
    fn bright_yellow(&self) -> String { self.stl("bright-yellow") }
    /// 🧉 » makes the text **bright blue**
    fn bright_blue(&self) -> String { self.stl("bright-blue") }
    /// 🧉 » makes the text **bright magenta**
    fn bright_magenta(&self) -> String { self.stl("bright-magenta") }
    /// 🧉 » makes the text **bright cyan**
    fn bright_cyan(&self) -> String { self.stl("bright-cyan") }
    /// 🧉 » makes the text **bright white**
    fn bright_white(&self) -> String { self.stl("bright-white") }
    /// 🧉 » makes the text colored after the **`rgb`** param (`#RRGGBB` format)
    fn rgb(&self, rgb: &str) -> String { self.stl(rgb) }
    /// 🧉 » makes the background of the text **black**
    fn on_black(&self) -> String { self.stl("on black") }
    /// 🧉 » makes the background of the text **red**
    fn on_red(&self) -> String { self.stl("on red") }
    /// 🧉 » makes the background of the text **green**
    fn on_green(&self) -> String { self.stl("on green") }
    /// 🧉 » makes the background of the text **yellow**
    fn on_yellow(&self) -> String { self.stl("on yellow") }
    /// 🧉 » makes the background of the text **blue**
    fn on_blue(&self) -> String { self.stl("on blue") }
    /// 🧉 » makes the background of the text **magenta**
    fn on_magenta(&self) -> String { self.stl("on magenta") }
    /// 🧉 » makes the background of the text **cyan**
    fn on_cyan(&self) -> String { self.stl("on cyan") }
    /// 🧉 » makes the background of the text **white**
    fn on_white(&self) -> String { self.stl("on white") }
    /// 🧉 » makes the background of the text **bright black**
    fn on_bright_black(&self) -> String { self.stl("on bright-black") }
    /// 🧉 » makes the background of the text **bright red**
    fn on_bright_red(&self) -> String { self.stl("on bright-red") }
    /// 🧉 » makes the background of the text **bright green**
    fn on_bright_green(&self) -> String { self.stl("on bright-green") }
    /// 🧉 » makes the background of the text **bright yellow**
    fn on_bright_yellow(&self) -> String { self.stl("on bright-yellow") }
    /// 🧉 » makes the background of the text **bright blue**
    fn on_bright_blue(&self) -> String { self.stl("on bright-blue") }
    /// 🧉 » makes the background of the text **bright magenta**
    fn on_bright_magenta(&self) -> String { self.stl("on bright-magenta") }
    /// 🧉 » makes the background of the text **bright cyan**
    fn on_bright_cyan(&self) -> String { self.stl("on bright-cyan") }
    /// 🧉 » makes the background of the text **bright white**
    fn on_bright_white(&self) -> String { self.stl("on bright-white") }
    /// 🧉 » makes the background color = **`rgb`** param (`#RRGGBB` format)
    fn on_rgb(&self, rgb: &str) -> String { self.stl(&format!("on {}", rgb)) }
    /// 🧉 » makes the text **bold**
    fn bold(&self) -> String { self.stl("+bold") }
    /// 🧉 » makes the text **dim**
    fn dim(&self) -> String { self.stl("+dim") }
    /// 🧉 » makes the text **italic**
    fn italic(&self) -> String { self.stl("+italic") }
    /// 🧉 » makes the text **underline**
    fn underline(&self) -> String { self.stl("+underline") }
    /// 🧉 » makes the text **blink**
    fn blink(&self) -> String { self.stl("+blink") }
    /// 🧉 » makes the text **reverse**
    fn reverse(&self) -> String { self.stl("+reverse") }
    /// 🧉 » makes the text **hidden**
    fn hidden(&self) -> String { self.stl("+hidden") }
    /// 🧉 » makes the text **strikethrough**
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

// alias for stylize: stl

/// 🧉 » stl fn
/// --
///
/// Stylizes a string with optional ANSI color and attributes.
/// 
/// This is an alias for the `stylize` function.
pub fn stl<S: AsRef<str>>(s: S, instructions: &str) -> String {
    stylize(s, instructions)
}