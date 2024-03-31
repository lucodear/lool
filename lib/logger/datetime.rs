use std::time::{SystemTime, UNIX_EPOCH};

pub fn noop_datetime() -> String {
    "".to_string()
}

/// Get the current time in the format yyyy-mm-dd hh:mm:ss
pub fn utc_current_time() -> String {
    // Get the current system time
    let now = SystemTime::now();

    // Convert the adjusted time to a formatted string
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let days = secs / (24 * 3600);
            let mut years = 1970;
            let mut days_left = days;

            // Adjusting for leap years
            loop {
                let days_in_year = if is_leap_year(years) { 366 } else { 365 };
                if days_left < days_in_year {
                    break;
                }
                days_left -= days_in_year;
                years += 1;
            }

            let (month, day) = days_to_date(days_left, is_leap_year(years));

            let hours = (secs / 3600) % 24;
            let minutes = (secs / 60) % 60;
            let seconds = secs % 60;

            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                years,
                month + 1,
                day,
                hours,
                minutes,
                seconds
            )
        }
        Err(_) => "0".to_string(),
    }
}

// Check if a year is a leap year
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// Calculate the month and day from the number of days since UNIX epoch
fn days_to_date(days: u64, leap: bool) -> (u64, u64) {
    let days_in_month = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days_left = days;
    let mut month = 0;

    while days_left >= days_in_month[month as usize] {
        days_left -= days_in_month[month as usize];
        month += 1;
    }

    (month, days_left + 1) // Adding 1 to day to make it 1-based
}
