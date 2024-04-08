use eyre::{ensure, eyre, Result};

pub mod cron_date;

const NO_SIGN_ERR: &str = "invalid timezone offset";
const INVALID_OFFSET_ERR: &str = "invalid timezone offset (format should be `+hh{{:mm}}?`)";
const INVALID_OFFSET_HS_ERR: &str = "invalid timezone offset (hour should be between 0 and 14)";
const INVALID_OFFSET_MIN_ERR: &str = "invalid timezone offset (minute should be between 0 and 59)";

/// 🧉 » converts `hours` and `minutes` durations to total seconds
///
/// e.g. `h=1, m=30` should return `5400`
///
/// meaning `1 hour and 30 minutes` is `5400` seconds
pub fn hm_to_s(h: i32, m: i32) -> i32 {
    h * 3600 + m * 60
}

/// 🧉 » converts timezone offset to seconds
///
/// eg:
/// - `+01:00` -> `3600`
/// - `-03:00` -> `-10800`
/// - `+03` -> `10800`
/// - `+00:00` -> `0`
/// - `-03:30` -> `-12600`
///
/// returns an error if the offset is invalid or badly formatted
pub fn tz_to_s(offset: &str) -> Result<i32> {
    // if it doesn't start with '+' or '-', it's invalid
    ensure!(
        offset.starts_with('+') || offset.starts_with('-'),
        NO_SIGN_ERR
    );

    let sign = if offset.starts_with('+') { 1 } else { -1 };
    let parts: Vec<&str> = offset[1..].split(':').collect();

    // it should have at least one part and at most two parts
    ensure!(!parts.is_empty(), INVALID_OFFSET_ERR);
    ensure!(parts.len() <= 2, INVALID_OFFSET_ERR);

    let hours = parts[0].parse::<u32>().map_err(|_| eyre!(INVALID_OFFSET_ERR))?;

    let minutes = if parts.len() == 2 {
        parts[1].parse::<u32>().map_err(|_| eyre!(INVALID_OFFSET_ERR))?
    } else {
        0
    };

    // offset hours cannot be greater than 14, minutes cannot be greater than 59 and
    // seconds cannot be greater than 59
    ensure!(hours <= 14, INVALID_OFFSET_HS_ERR);
    ensure!(minutes <= 59, INVALID_OFFSET_MIN_ERR);

    Ok(sign * hm_to_s(hours as i32, minutes as i32))
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        eyre::{set_hook, DefaultHandler},
    };

    fn setup_eyre() {
        let _ = set_hook(Box::new(DefaultHandler::default_with));
    }

    #[test]
    fn test_h_m_to_seconds() {
        assert_eq!(hm_to_s(1, 30), 5400);
        assert_eq!(hm_to_s(2, 0), 7200);
        assert_eq!(hm_to_s(3, 0), 10800);
        assert_eq!(hm_to_s(0, 0), 0);
    }

    #[test]
    fn test_timezone_offset_to_seconds() -> Result<()> {
        assert_eq!(tz_to_s("+01:00")?, 3600);
        assert_eq!(tz_to_s("-03:00")?, -10800);
        assert_eq!(tz_to_s("+03")?, 10800);
        assert_eq!(tz_to_s("+00:00")?, 0);
        assert_eq!(tz_to_s("-00:00")?, 0);
        assert_eq!(tz_to_s("-3")?, -10800);
        assert_eq!(tz_to_s("-3:30")?, -12600);

        Ok(())
    }

    #[test]
    fn test_timezone_offset_to_seconds_missing_sign_err() {
        setup_eyre();
        let err = tz_to_s("01:00").unwrap_err().to_string();
        assert_eq!(err, NO_SIGN_ERR);
    }

    #[test]
    fn test_timezone_offset_to_seconds_invalid_format_errs() {
        setup_eyre();
        let err = tz_to_s("+01:00:00").unwrap_err().to_string();
        assert_eq!(err, INVALID_OFFSET_ERR);

        let err = tz_to_s("+01:").unwrap_err().to_string();
        assert_eq!(err, INVALID_OFFSET_ERR);

        let err = tz_to_s("--01:00").unwrap_err().to_string();
        assert_eq!(err, INVALID_OFFSET_ERR);

        let err = tz_to_s("+01:-01").unwrap_err().to_string();
        assert_eq!(err, INVALID_OFFSET_ERR);

        let err = tz_to_s("+15:01").unwrap_err().to_string();
        assert_eq!(err, INVALID_OFFSET_HS_ERR);

        let err = tz_to_s("+01:60").unwrap_err().to_string();
        assert_eq!(err, INVALID_OFFSET_MIN_ERR);
    }
}
