use {eyre::Result, palette::rgb::Rgb, ratatui::style::Color, std::str::FromStr};

pub mod framework {
    pub mod app;
    pub mod component;
    pub mod events;
    pub mod keyboard;
    pub mod tui;
}

#[macro_export]
macro_rules! components {
    ( $( $x:expr $( => $t:ty )* ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(
                    Box::new($x)
                        as Box<dyn lool::cli::tui::framework::component::Component $( $t + )* >
                );
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! children {
    ( $( $name:expr => $value:expr ),* ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert(
                    $name.to_string(),
                    Box::new($value) as Box<dyn lool::cli::tui::framework::component::Component>
                );
            )*
            map
        }
    };
}

pub fn rgb(hex: &str) -> Result<Color> {
    let rgb: Rgb<u8, u8> = Rgb::from_str(hex)?;
    Ok(Color::Rgb(rgb.red, rgb.green, rgb.blue))
}
