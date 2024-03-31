<p align="center"><img src="../../../.github/img/logo-cli-colors.svg" height="256"></p>

<br>
<br>
<br>

<p align="center"><b>lool » <code>cli/stylize</code></b> is a set of utilities for colorizing console outputs.
</p>

<br>
<br>
<br>

# Installation

This crate is for internal use. It's only published privately. 

```bash
cargo add lool --registry=lugit --features cli-stylize
```

# Usage

## Styling-Instructions API

This library provides a set of functions to colorize and stylize console outputs. The "low-level" API works by parsing a string with special instructions to apply styles to the text.

```rs
use lool::cli::stylize::stylize;

fn main() {
    let text = "Hello, World!";
    let styled_text = stylize(text, "blue on red+bold|underline");
    println!("{}", styled_text);
}
```

The above code will print "Hello, World!" with the following styles:

- Foreground color: **blue**
- Background color: **red**
- Attributes: **bold** and **underline**

### Instructions

The instructions are a string with the following format:

```plaintext
<fg-color> on <bg-color>+<attr1>|<attr2>|<attr...>
```

All instructions are optional. Which means that the following are all valid instructions:

- `red`
- `on green`
- `+bold`
- `red+bold`
- `on green+bold`
- `on green+bold|underline`

### Colors
It accepts all standard ANSI colors:

- black
- red
- green
- yellow
- blue
- magenta
- cyan
- white
- bright-black
- bright-red
- bright-green
- bright-yellow
- bright-blue
- bright-magenta
- bright-cyan
- bright-white

It also supports TrueColor by providing a hex code:

e.g.

```plaintext
#ff0000 on #00ff00
```

Which means, basically, red on green.

### Attributes

The following attributes are supported:

- bold
- dim
- italic
- underline
- blink
- reverse
- hidden
- strikethrough

Attributes can be combined with the `|` character.

## High-Level API

The library also provides an abstraction over the low-level API. The high-level API provides a set of functions to colorize and stylize console outputs.

It builds on top of `str` and `String` types, providing a set of methods to colorize and stylize text.

```rs
// import the Stylish trait
use lool::cli::stylize::{Stylize};

fn main() {
    let styled_text = "Hello, World!".blue().on_red().bold().underline();
    println!("{}", styled_text);
}

```

The above code will print "Hello, World!" with the following styles:

- Foreground color: **blue**
- Background color: **red**
- Attributes: **bold** and **underline**