<p align="center"><img src="./../../.github/img/logo-macros.svg" width="200"></p>

<br>
<br>
<br>

<p align="center"><b>lool » <code>macros</code></b> is a collection of utility macros for rust.</p>
</p>

<br>
<br>
<br>

# Installation

This crate is for internal use. It's only published privately. 

```bash
cargo add lool --registry=lugit --features macros
```

# Macros

## `s`

A macro to create a `String` from a `&str`.

```rs
use lool::s;

fn main() {
    let s = s!("Hello, world!");
    // now `s` is a `String`
}
```

## `f` or `fail`

A macro to use with the [`eyre`](https://crates.io/crates/eyre) crate.
It basically creates an `Err`.

It's almost the same as `bail!` from the `anyhow` crate, but this one doesn't explicitly returns (the `bail!` macro creates a `return` statement).

This macro is equivalent to: `Err(eyre!(<args>))`.

```rs
use lool::f;

fn main() -> eyre::Result<()> {
    if (2 + 2) == 5 {
        f!("Oh no! I'm bad at math!");
    }

    Ok(())
}
```

which is equivalent to:

```rs
use eyre::eyre;

fn main() -> eyre::Result<()> {
    if (2 + 2) == 5 {
        return Err(eyre!("Oh no! I'm bad at math!"));
    }

    Ok(())
}
```

It also works as a final statement:

```rs
use lool::f;

fn fail_always() -> eyre::Result<()> {
    // no ; at the end
    f!("I'm a failure")
}
```

