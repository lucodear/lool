#[cfg(test)]
mod parse_instruction {
    use {
        crate::cli::stylize::{
            style::{Color, StyleAttributes},
            stylizer::instructions::{parse, parse_color_instruction},
        },
        eyre::{set_hook, DefaultHandler},
    };

    fn setup_eyre() {
        let _ = set_hook(Box::new(DefaultHandler::default_with));
    }

    #[test]
    fn test_parse_color_instruction() -> eyre::Result<()> {
        setup_eyre();

        assert_eq!(parse_color_instruction("red")?, (Some(Color::Red), None));
        assert_eq!(
            parse_color_instruction("#FF0000")?,
            (Some(Color::TrueColor { r: 255, g: 0, b: 0 }), None)
        );
        assert_eq!(
            parse_color_instruction("on blue")?,
            (None, Some(Color::Blue))
        );
        assert_eq!(
            parse_color_instruction("on #0000FF")?,
            (None, Some(Color::TrueColor { r: 0, g: 0, b: 255 }))
        );
        assert_eq!(
            parse_color_instruction("red on blue")?,
            (Some(Color::Red), Some(Color::Blue))
        );
        assert_eq!(
            parse_color_instruction("#FF0000 on #0000FF")?,
            (
                Some(Color::TrueColor { r: 255, g: 0, b: 0 }),
                Some(Color::TrueColor { r: 0, g: 0, b: 255 })
            )
        );
        assert!(parse_color_instruction("red on blue on green").is_err());
        assert!(parse_color_instruction("red on").is_err());
        assert!(parse_color_instruction("on").is_err());
        assert!(parse_color_instruction("on red blue").is_err());
        assert!(parse_color_instruction("red on blue on green").is_err());
        assert!(parse_color_instruction("invalid").is_err());
        assert!(parse_color_instruction("red on invalid").is_err());
        assert!(parse_color_instruction("invalid on blue").is_err());
        Ok(())
    }

    #[test]
    fn test_parse_instructions() -> eyre::Result<()> {
        setup_eyre();

        let styled_string = parse("red on blue")?;
        assert_eq!(styled_string.fg, Some(Color::Red));
        assert_eq!(styled_string.bg, Some(Color::Blue));
        assert_eq!(styled_string.attrs, StyleAttributes::empty());

        let styled_string = parse("on blue")?;
        assert_eq!(styled_string.fg, None);
        assert_eq!(styled_string.bg, Some(Color::Blue));
        assert_eq!(styled_string.attrs, StyleAttributes::empty());

        let styled_string = parse("red")?;
        assert_eq!(styled_string.fg, Some(Color::Red));
        assert_eq!(styled_string.bg, None);
        assert_eq!(styled_string.attrs, StyleAttributes::empty());

        // ##RRGGBB
        let styled_string = parse("#FF0000 on #0000FF")?;
        assert_eq!(
            styled_string.fg,
            Some(Color::TrueColor { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            styled_string.bg,
            Some(Color::TrueColor { r: 0, g: 0, b: 255 })
        );
        assert_eq!(styled_string.attrs, StyleAttributes::empty());

        let styled_string = parse("red on blue+bold")?;
        assert_eq!(styled_string.fg, Some(Color::Red));
        assert_eq!(styled_string.bg, Some(Color::Blue));
        assert_eq!(styled_string.attrs, StyleAttributes::BOLD);

        // ##RRGGBB
        let styled_string = parse("#FF0000 on #0000FF+bold")?;
        assert_eq!(
            styled_string.fg,
            Some(Color::TrueColor { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            styled_string.bg,
            Some(Color::TrueColor { r: 0, g: 0, b: 255 })
        );
        assert_eq!(styled_string.attrs, StyleAttributes::BOLD);

        let styled_string = parse("red on blue+bold|underline")?;
        assert_eq!(styled_string.fg, Some(Color::Red));
        assert_eq!(styled_string.bg, Some(Color::Blue));
        assert_eq!(
            styled_string.attrs,
            StyleAttributes::BOLD | StyleAttributes::UNDERLINE
        );

        let styled_string = parse("red on #0000FF+bold|underline|italic")?;
        assert_eq!(styled_string.fg, Some(Color::Red));
        assert_eq!(
            styled_string.bg,
            Some(Color::TrueColor { r: 0, g: 0, b: 255 })
        );
        assert_eq!(
            styled_string.attrs,
            StyleAttributes::BOLD | StyleAttributes::UNDERLINE | StyleAttributes::ITALIC
        );

        let styled_string = parse("+bold")?;
        assert_eq!(styled_string.fg, None);
        assert_eq!(styled_string.bg, None);
        assert_eq!(styled_string.attrs, StyleAttributes::BOLD);

        let styled_string = parse("on red+bold")?;
        assert_eq!(styled_string.fg, None);
        assert_eq!(styled_string.bg, Some(Color::Red));
        assert_eq!(styled_string.attrs, StyleAttributes::BOLD);

        let styled_string = parse("red+bold")?;
        assert_eq!(styled_string.fg, Some(Color::Red));
        assert_eq!(styled_string.bg, None);
        assert_eq!(styled_string.attrs, StyleAttributes::BOLD);

        let styled_string = parse("red on blue+bold")?;
        assert_eq!(styled_string.fg, Some(Color::Red));
        assert_eq!(styled_string.bg, Some(Color::Blue));

        Ok(())
    }
}
