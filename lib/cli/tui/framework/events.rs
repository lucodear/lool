use {
    crossterm::event::{KeyEvent, MouseEvent},
    std::fmt::{Display, Formatter, Result},
    strum::EnumString,
};

#[derive(Debug, PartialEq, Eq, Clone, EnumString)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,
    Version,
    AppAction(String),
    Key(String),
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let enum_str = write!(f, "{:?}", self);
        enum_str
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    Init,
    Quit,
    Error,
    Closed,
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}
