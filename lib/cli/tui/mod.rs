mod framework {
    pub mod app;
    pub mod component;
    pub mod events;
    pub mod keyboard;
    pub mod tui;
}

use {eyre::Result, palette::rgb::Rgb, ratatui::style::Color, std::str::FromStr};

pub use framework::{
    app::{App, Kb},
    component::{Children, Component},
    events::{Action, Event},
    keyboard::KeyBindings,
    tui::{Frame, Tui, IO},
};

pub mod utils {
    pub mod component {
        pub use super::super::framework::component::{
            child_downcast, child_downcast_mut, init_children, pass_action_handler_to_children,
            pass_message_to_children, set_active_on_children, update_children,
        };
    }

    pub mod keyboard {
        pub use super::super::framework::keyboard::{key_event_to_string, parse_key_sequence};
    }
}

#[cfg(feature = "cli.tui.widgets")]
pub mod widgets {
    pub mod gridselector {
        mod selector;
        mod state;
        mod widget;

        pub use {selector::*, state::*};
    }

    pub mod textarea;

    pub mod switch {
        mod widget;
        pub use widget::*;
    }
}

// ratatui prelude
pub mod ratatui {
    pub use ratatui::{prelude::*, *};
}

#[macro_export]
macro_rules! components {
    ( $( $x:expr $( => $t:ty )* ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(
                    Box::new($x)
                        as Box<dyn lool::tui::Component $( $t + )* >
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
                    Box::new($value) as Box<dyn lool::tui::Component>
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
