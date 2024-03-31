use eyre::{set_hook, DefaultHandler, Result};
use lool::cli::stylize::{stylize, Stylize};

fn setup_eyre() {
    let _ = set_hook(Box::new(DefaultHandler::default_with));
}

fn main() -> Result<()> {
    setup_eyre();

    let red_bold = stylize("[red+bold]", "red+bold");
    let alt_red_bold = stylize(stylize("alt [red+bold]", "red"), "+bold");

    let red_bold_italic = stylize("[red+bold|italic]", "red+bold|italic");
    let alt_red_bold_italic = stylize(stylize(stylize("alt [red+bold|italic]", "red"), "+bold"), "+italic");

    let red_on_blue = stylize("[white on blue]", "white on blue");
    let rgb = stylize("[#3a95ef]", "#3a95ef");
    let rgb_on_rgb = stylize("[#3a95ef on #c174dd]", "#3a95ef on #c174dd");
    let rgb_dim = stylize("[#3a95ef+dimmed]", "#3a95ef+dimmed");

    println!("pre {} post", red_bold);
    println!("pre {} post", alt_red_bold);
    println!("pre {} post", red_bold_italic);
    println!("pre {} post", alt_red_bold_italic);

    println!("pre {} post", red_on_blue);
    println!("pre {} post", rgb);
    println!("pre {} post", rgb_on_rgb);
    println!("pre {} post", rgb_dim);

    println!("pre {} post", "[green]".stl("green").stl("+bold"));
    println!("pre {} post", "[green+bold]".stl("green+bold"));
    
    println!("pre {} post", "[.blue()]".blue());
    println!("pre {} post", "[.blue().bold()]".blue().bold());
    println!("pre {} post", "[.blue().on_red().bold()]".blue().on_red().bold());

    println!("pre {} post", "[.dim()]".dim());
    println!("pre {} post", "[.blue().dim()]".blue().dim());

    Ok(())
}