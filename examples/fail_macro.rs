use {
    eyre::{set_hook, DefaultHandler, Result},
    lool::f,
};

fn setup_eyre() {
    let _ = set_hook(Box::new(DefaultHandler::default_with));
}

fn main() -> Result<()> {
    setup_eyre();

    if (2 + 2) == 5 {
        return f!("Oh no! I'm bad at math!");
    }

    Ok(())
}
