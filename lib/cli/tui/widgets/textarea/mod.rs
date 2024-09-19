pub(super) mod behaviour {
    pub(super) mod cursor;
    pub(super) mod highlight;
    pub(super) mod input;
    pub(super) mod scroll;
    pub(super) mod util;
}

mod textarea;

pub use {
    behaviour::input::{Input, Key},
    textarea::{
        validation::{validators, ValidationResult},
        TextArea,
    },
};
