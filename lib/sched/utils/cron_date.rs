use chrono::{
    DateTime, Datelike, Duration, Local, Months, NaiveDate, TimeZone, Timelike, Utc, Weekday,
};

/// 🧉 » `LoolDate`
/// --
///
/// wrapper around `chrono::DateTime` to provide more date manipulation methods
pub struct LoolDate<Tz: TimeZone> {
    date: DateTime<Tz>,
}

pub enum TimeUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
}

impl<Tz: TimeZone> LoolDate<Tz> {
    /// creates a new `LoolDate` from a `DateTime`
    pub fn new(date: DateTime<Tz>) -> Self {
        Self { date }
    }

    // creates a new `LoolDate` in the `Local` timezone
    pub fn now() -> LoolDate<Local> {
        LoolDate { date: Local::now() }
    }

    /// creates a new `LoolDate` in the `Utc` timezone
    pub fn utc_now() -> LoolDate<Utc> {
        LoolDate { date: Utc::now() }
    }

    /// returns a clone of the inner `DateTime`
    pub fn date(&self) -> DateTime<Tz> {
        self.date.clone()
    }

    /// adds `1` year to the current date
    pub fn add_year(&mut self) {
        self.date = self.date.clone() + Months::new(12);
        self.set_start_of(TimeUnit::Year);
    }

    /// adds `1` month to the current date
    pub fn add_month(&mut self) {
        self.date = self.date.clone() + Months::new(1);
        self.set_start_of(TimeUnit::Month);
    }

    /// adds `1` day to the current date
    pub fn add_day(&mut self) {
        self.date += Duration::days(1);
        self.set_start_of(TimeUnit::Day);
    }

    /// adds `1` hour to the current date
    pub fn add_hour(&mut self) {
        self.date += Duration::hours(1);
        self.set_start_of(TimeUnit::Hour);
    }

    /// adds `1` minute to the current date
    pub fn add_minute(&mut self) {
        self.date += Duration::minutes(1);
        self.set_start_of(TimeUnit::Minute);
    }

    /// adds `1` second to the current date
    pub fn add_second(&mut self) {
        self.date += Duration::seconds(1);
        self.set_start_of(TimeUnit::Second);
    }

    /// subtracts `1` year from the current date
    pub fn subs_year(&mut self) {
        self.date = self.date.clone() - Months::new(12);
    }

    /// subtracts `1` month from the current date
    pub fn subs_month(&mut self) {
        self.date = self.date.clone() - Months::new(1);
    }

    /// subtracts `1` day from the current date
    pub fn subs_day(&mut self) {
        self.date -= Duration::days(1);
    }

    /// subtracts `1` hour from the current date
    pub fn subs_hour(&mut self) {
        self.date -= Duration::hours(1);
    }

    /// subtracts `1` minute from the current date
    pub fn subs_minute(&mut self) {
        self.date -= Duration::minutes(1);
    }

    /// subtracts `1` second from the current date
    pub fn subs_second(&mut self) {
        self.date -= Duration::seconds(1);
    }

    /// returns the day of the month starting from `1`
    pub fn day(&self) -> u32 {
        self.date.day()
    }

    /// returns the day of the week
    pub fn weekday(&self) -> Weekday {
        self.date.weekday()
    }

    /// returns the day of the week starting from `0` (monday)
    pub fn weekday_from_monday(&self) -> u32 {
        self.date.weekday().num_days_from_monday()
    }

    /// returns the day of the week starting from `0` (sunday)
    pub fn weekday_from_sunday(&self) -> u32 {
        self.date.weekday().num_days_from_sunday()
    }

    /// returns the month starting from 1
    pub fn month(&self) -> u32 {
        self.date.month()
    }

    /// returns the year number in the [calendar date](./naive/struct.NaiveDate.html#calendar-date).
    pub fn year(&self) -> i32 {
        self.date.year()
    }

    /// returns the hour `(0..23)`
    pub fn hour(&self) -> u32 {
        self.date.hour()
    }

    /// returns the minute `(0..59)`
    pub fn minute(&self) -> u32 {
        self.date.minute()
    }

    /// returns the second number `(0..59)`
    pub fn second(&self) -> u32 {
        self.date.second()
    }

    /// returns the number of milliseconds since the last second boundary
    ///
    /// In event of a [leap second](https://en.wikipedia.org/wiki/Leap_second) this may exceed
    /// `999`.
    pub fn millis(&self) -> u64 {
        self.date.timestamp_subsec_millis() as u64
    }

    /// Returns the number of microseconds since the last second boundary.
    ///
    /// In event of a [leap second](https://en.wikipedia.org/wiki/Leap_second) this may exceed
    /// `999999`.
    pub fn micros(&self) -> u32 {
        self.date.timestamp_subsec_micros()
    }

    /// Returns the number of nanoseconds since the last second boundary.
    ///
    /// In event of a [leap second](https://en.wikipedia.org/wiki/Leap_second) this may exceed
    /// `999999999`.
    pub fn nanos(&self) -> u32 {
        self.date.timestamp_subsec_nanos()
    }

    /// sets the nanoseconds since the last second change
    ///
    /// values greater than `2000,000,000` will be clamped to `1999,999,999`
    pub fn set_nanos(&mut self, nanos: u32) {
        // avoid `whith_nanosecond` returning None for > 2_000_000_000 values
        // so we can safely unwrap the result
        let nanos = if nanos > 2_000_000_000 {
            1_999_999_999
        } else {
            nanos
        };
        self.date = self.date.with_nanosecond(nanos).unwrap();
    }

    /// sets the microseconds since the last second change
    ///
    /// values greater than `2,000,000` will be clamped to `1,999,999`
    pub fn set_micros(&mut self, micros: u32) {
        let micros = if micros > 2_000_000 {
            1_999_999
        } else {
            micros
        };
        self.date = self.date.with_nanosecond(micros * 1_000).unwrap();
    }

    /// sets the milliseconds since the last second change
    ///
    /// values greater than `2,000` will be clamped to `1,999`
    pub fn set_millis(&mut self, millis: u64) {
        let millis = if millis > 2_000 { 1_999 } else { millis };
        self.date = self.date.with_nanosecond(millis as u32 * 1_000_000).unwrap();
    }

    /// sets the second number of the date
    ///
    /// values >= `60` will be clamped to `0`
    pub fn set_second(&mut self, second: u32) {
        let second = if second >= 60 { 0 } else { second };
        self.date = self.date.with_second(second).unwrap();
    }

    /// sets the minute number of the date
    ///
    /// values >= `60` will be clamped to `0`
    pub fn set_minute(&mut self, minute: u32) {
        let minute = if minute >= 60 { 0 } else { minute };
        self.date = self.date.with_minute(minute).unwrap();
    }

    /// sets the hour number of the date `(0..23)`
    ///
    /// values >= `24` will be clamped to `0`
    pub fn set_hour(&mut self, hour: u32) {
        let hour = if hour >= 24 { 0 } else { hour };
        self.date = self.date.with_hour(hour).unwrap();
    }

    /// sets the hour, minute and second of the date at once
    ///
    /// - `hour` must be in the range `(0..23)`
    /// - `minute` must be in the range `(0..59)`
    /// - `second` must be in the range `(0..59)`
    ///
    /// values greater than the maximum will be clamped (see `set_hour`, `set_minute`, `set_second`)
    /// documentation for more information
    pub fn set_hms(&mut self, hour: u32, minute: u32, second: u32) {
        self.set_hour(hour);
        self.set_minute(minute);
        self.set_second(second);
    }

    /// sets the day of the month `(1..31)`
    ///
    /// values greater than the maximum day of the month will be clamped to the last day of the
    /// month, depending on the current year and month and taking leap years into account.
    pub fn set_day(&mut self, day: u32) {
        let day = if day == 0 { 1 } else { day };
        let days_in_month = get_days_from_month(self.date.year(), self.date.month());

        if day > days_in_month {
            self.date = self.date.with_day(days_in_month).unwrap();
        } else {
            self.date = self.date.with_day(day).unwrap();
        }
    }

    /// sets the month of the year `(1..12)`
    ///
    /// - values greater than the maximum month will be clamped to `12`
    /// - values less than `1` will be clamped to `1`
    ///
    /// if the current day is greater than the last day of the new month, the day will be clamped
    /// to the last day of the month, depending on the current year and month and taking leap years
    /// into account. So, be aware that changing the month may imply a change in the day.
    /// This is done to avoid invalid dates like `2024-02-30` which would cause an error in the
    /// `chrono` library.
    pub fn set_month(&mut self, month: u32) {
        let month = if month > 12 {
            12
        } else if month == 0 {
            1
        } else {
            month
        };

        let day = self.date.day();
        let days_in_month = get_days_from_month(self.date.year(), month);

        if day > days_in_month {
            self.date = self.date.with_day(days_in_month).unwrap();
            self.date = self.date.with_month(month).unwrap();
        } else {
            self.date = self.date.with_month(month).unwrap();
        }
    }

    /// sets the month and day of the date at once
    ///
    /// check the `set_month` and `set_day` documentation for more information about the clamping
    /// behavior.
    pub fn set_md(&mut self, month: u32, day: u32) {
        self.set_month(month);
        self.set_day(day);
    }

    /// sets the year of the date
    ///
    /// **warning**: changing the year may imply a change in the day.
    /// For example, if the current date is `2024-02-29` and you set the year to `2023`, the date
    /// date `2023-02-29` will be invalid, because `2023` is not a leap year. In this case, the day
    /// will be clamped to `2023-02-28`.
    ///
    /// check the `set_day` documentation for more information about the clamping behavior.
    pub fn set_year(&mut self, year: i32) {
        let month = self.date.month();
        let day = self.date.day();
        let days_in_month = get_days_from_month(year, month);

        if day > days_in_month {
            self.date = self.date.with_day(days_in_month).unwrap();
            self.date = self.date.with_year(year).unwrap();
        } else {
            self.date = self.date.with_year(year).unwrap();
        }
    }

    /// sets the year, month and day of the date at once
    ///
    /// check the `set_year`, `set_month` and `set_day` documentation for more information about the
    /// clamping behavior.
    pub fn set_ymd(&mut self, year: i32, month: u32, day: u32) {
        self.set_year(year);
        self.set_month(month);
        self.set_day(day);
    }

    /// returns true when the current date is the last day of the month.
    pub fn is_last_day_of_month(&self) -> bool {
        self.date.day() == get_days_from_month(self.date.year(), self.date.month())
    }

    /// returns true when the current weekday is the last occurrence of this weekday
    /// for the present month.
    pub fn is_last_weekday_of_month(&self) -> bool {
        // check this by adding 7 days to the current date and seeing if it's
        // a different month
        let next_weekday = self.date.clone() + Duration::days(7);
        self.date.month() != next_weekday.month()
    }

    pub fn set_start_of(&mut self, unit: TimeUnit) {
        match unit {
            TimeUnit::Year => {
                self.set_month(1);
                self.set_day(1);
                self.set_hour(0);
                self.set_minute(0);
                self.set_second(0);
                self.set_nanos(0);
            }
            TimeUnit::Month => {
                self.set_day(1);
                self.set_hour(0);
                self.set_minute(0);
                self.set_second(0);
                self.set_nanos(0);
            }
            TimeUnit::Day => {
                self.set_hour(0);
                self.set_minute(0);
                self.set_second(0);
                self.set_nanos(0);
            }
            TimeUnit::Hour => {
                self.set_minute(0);
                self.set_second(0);
                self.set_nanos(0);
            }
            TimeUnit::Minute => {
                self.set_second(0);
                self.set_nanos(0);
            }
            TimeUnit::Second => {
                self.set_nanos(0);
            }
        };
    }
}

fn get_days_from_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days() as u32
}

#[cfg(test)]
mod tests {
    use {super::*, chrono::Utc};

    #[test]
    fn test_get_days_from_month() {
        assert_eq!(get_days_from_month(2024, 1), 31);
        assert_eq!(get_days_from_month(2024, 2), 29);
        assert_eq!(get_days_from_month(2024, 3), 31);
        assert_eq!(get_days_from_month(2024, 4), 30);
        assert_eq!(get_days_from_month(2024, 5), 31);
        assert_eq!(get_days_from_month(2024, 6), 30);
        assert_eq!(get_days_from_month(2024, 7), 31);
        assert_eq!(get_days_from_month(2024, 8), 31);
        assert_eq!(get_days_from_month(2024, 9), 30);
        assert_eq!(get_days_from_month(2024, 10), 31);
        assert_eq!(get_days_from_month(2024, 11), 30);
        assert_eq!(get_days_from_month(2024, 12), 31);
    }

    #[test]
    fn test_set_day() {
        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        date.set_day(31);
        assert_eq!(date.day(), 31);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2024);

        date.set_day(1);
        assert_eq!(date.day(), 1);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2024);
    }

    #[test]
    fn test_set_day_invalid_values() {
        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        date.set_day(35);
        assert_eq!(date.day(), 31);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2024);

        date.set_day(0);
        assert_eq!(date.day(), 1);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2024);

        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap());
        date.set_day(30);
        assert_eq!(date.day(), 29);
        assert_eq!(date.month(), 2);
        assert_eq!(date.year(), 2024);
    }

    #[test]
    fn test_set_month() {
        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        date.set_month(12);
        assert_eq!(date.month(), 12);

        date.set_month(13);
        assert_eq!(date.month(), 12);

        date.set_month(1);
        assert_eq!(date.month(), 1);
    }

    #[test]
    fn test_set_month_invalid_values() {
        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap());
        date.set_month(13);
        assert_eq!(date.month(), 12);
        assert_eq!(date.year(), 2024);
        assert_eq!(date.day(), 10);

        date.set_month(0);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2024);
        assert_eq!(date.day(), 10);
    }

    #[test]
    fn test_set_month_leap_years() {
        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2023, 1, 29, 0, 0, 0).unwrap());
        date.set_month(2);
        assert_eq!(date.month(), 2);
        assert_eq!(date.year(), 2023);
        assert_eq!(date.day(), 28); // 2023 is not a leap year so, can't keep the original 29

        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 1, 29, 0, 0, 0).unwrap());
        date.set_month(2);
        assert_eq!(date.month(), 2);
        assert_eq!(date.year(), 2024);
        assert_eq!(date.day(), 29); // 2024 is a leap year so, 29 is a valid day
    }

    #[test]
    fn test_set_month_month_overflow() {
        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 1, 31, 0, 0, 0).unwrap());
        date.set_month(4);
        assert_eq!(date.month(), 4);
        assert_eq!(date.year(), 2024);
        assert_eq!(date.day(), 30); // since april has 30 days, it cant keep the original 31

        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 1, 31, 0, 0, 0).unwrap());
        date.set_month(8);
        assert_eq!(date.month(), 8);
        assert_eq!(date.year(), 2024);
        assert_eq!(date.day(), 31); // august has 31 days, so it can keep the original 31

        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 1, 31, 0, 0, 0).unwrap());
        date.set_month(11);
        assert_eq!(date.month(), 11);
        assert_eq!(date.year(), 2024);
        assert_eq!(date.day(), 30); // november has 30 days, so it cant keep the original 31
    }

    #[test]
    fn test_set_year() {
        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        date.set_year(2021);
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);

        date.set_year(2050);
        assert_eq!(date.year(), 2050);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);

        date.set_year(-5);
        assert_eq!(date.year(), -5);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_set_year_leap_years() {
        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2024, 2, 29, 0, 0, 0).unwrap());
        date.set_year(2023);
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 2);
        assert_eq!(date.day(), 28); // 2023 is not a leap year so, can't keep the original 29

        let mut date = LoolDate::new(Utc.with_ymd_and_hms(2023, 2, 28, 0, 0, 0).unwrap());
        date.set_year(2024);
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 2);
        assert_eq!(date.day(), 28);
    }
}
