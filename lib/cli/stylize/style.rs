use bitflags::bitflags;
use eyre::Context;
use std::borrow::Cow;

/// The 8 standard colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    TrueColor { r: u8, g: u8, b: u8 },
}

impl Color {
    pub fn to_fg_str(&self) -> Cow<'static, str> {
        match *self {
            Color::Black => "30".into(),
            Color::Red => "31".into(),
            Color::Green => "32".into(),
            Color::Yellow => "33".into(),
            Color::Blue => "34".into(),
            Color::Magenta => "35".into(),
            Color::Cyan => "36".into(),
            Color::White => "37".into(),
            Color::BrightBlack => "90".into(),
            Color::BrightRed => "91".into(),
            Color::BrightGreen => "92".into(),
            Color::BrightYellow => "93".into(),
            Color::BrightBlue => "94".into(),
            Color::BrightMagenta => "95".into(),
            Color::BrightCyan => "96".into(),
            Color::BrightWhite => "97".into(),
            Color::TrueColor { r, g, b } => format!("38;2;{};{};{}", r, g, b).into(),
        }
    }

    pub fn to_bg_str(&self) -> Cow<'static, str> {
        match *self {
            Color::Black => "40".into(),
            Color::Red => "41".into(),
            Color::Green => "42".into(),
            Color::Yellow => "43".into(),
            Color::Blue => "44".into(),
            Color::Magenta => "45".into(),
            Color::Cyan => "46".into(),
            Color::White => "47".into(),
            Color::BrightBlack => "100".into(),
            Color::BrightRed => "101".into(),
            Color::BrightGreen => "102".into(),
            Color::BrightYellow => "103".into(),
            Color::BrightBlue => "104".into(),
            Color::BrightMagenta => "105".into(),
            Color::BrightCyan => "106".into(),
            Color::BrightWhite => "107".into(),
            Color::TrueColor { r, g, b } => format!("48;2;{};{};{}", r, g, b).into(),
        }
    }

    pub fn from_str(s: &str) -> eyre::Result<Color> {
        let color = match s {
            "" => None,
            "black" => Some(Color::Black),
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "yellow" => Some(Color::Yellow),
            "blue" => Some(Color::Blue),
            "magenta" => Some(Color::Magenta),
            "cyan" => Some(Color::Cyan),
            "white" => Some(Color::White),
            "bright-black" => Some(Color::BrightBlack),
            "bright-red" => Some(Color::BrightRed),
            "bright-green" => Some(Color::BrightGreen),
            "bright-yellow" => Some(Color::BrightYellow),
            "bright-blue" => Some(Color::BrightBlue),
            "bright-magenta" => Some(Color::BrightMagenta),
            "bright-cyan" => Some(Color::BrightCyan),
            "bright-white" => Some(Color::BrightWhite),
            s if s.starts_with("#") => {
                let s = &s[1..];
                let r = u8::from_str_radix(&s[0..2], 16).context("Error parsing RGB color")?;
                let g = u8::from_str_radix(&s[2..4], 16).context("Error parsing RGB color")?;
                let b = u8::from_str_radix(&s[4..6], 16).context("Error parsing RGB color")?;
                Some(Color::TrueColor { r, g, b })
            }
            _ => None,
        };

        color.ok_or_else(|| eyre::eyre!("Invalid color: '{}'", s))
    }
}

bitflags! {
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct StyleAttributes: u8 {
        const BOLD          = 0b00000001;
        const DIM           = 0b00000010;
        const ITALIC        = 0b00000100;
        const UNDERLINE     = 0b00001000;
        const BLINK         = 0b00010000;
        const REVERSED      = 0b00100000;
        const HIDDEN        = 0b01000000;
        const STRIKETHROUGH = 0b10000000;
    }
}

impl StyleAttributes {
    pub fn to_ansi_codes(&self) -> Vec<&'static str> {
        let mut v = Vec::new();
        if self.contains(StyleAttributes::BOLD) {
            v.push("1");
        }
        if self.contains(StyleAttributes::DIM) {
            v.push("2");
        }
        if self.contains(StyleAttributes::ITALIC) {
            v.push("3");
        }
        if self.contains(StyleAttributes::UNDERLINE) {
            v.push("4");
        }
        if self.contains(StyleAttributes::BLINK) {
            v.push("5");
        }
        if self.contains(StyleAttributes::REVERSED) {
            v.push("7");
        }
        if self.contains(StyleAttributes::HIDDEN) {
            v.push("8");
        }
        if self.contains(StyleAttributes::STRIKETHROUGH) {
            v.push("9");
        }
        v
    }
}
