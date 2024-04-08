pub mod builder;
    
use {
    super::Rule,
    crate::sched::utils::cron_date::LoolDate,
    chrono::{DateTime, Datelike, Local},
};

/// 🧉 » a recurrence rule-set
///
/// sets rules that define a certain recurrence behavior
/// 
/// use the builder pattern to create a new `RecurrenceRuleSet`
pub struct RecurrenceRuleSet {
    /// second of the minute (0..59)
    second: Option<Rule<u32>>,
    /// minute of the hour (0..59)
    minute: Option<Rule<u32>>,
    /// hour of the day (0..23)
    hour: Option<Rule<u32>>,
    /// day of the week starting from sunday (`0=Sunday`, `1=Monday`, ..., `6=Saturday`)
    dow: Option<Rule<u32>>,
    /// day of the month (1..31)
    day: Option<Rule<u32>>,
    /// month of the year (1..12)
    month: Option<Rule<u32>>,
    /// year
    year: Option<Rule<i32>>,
}

impl RecurrenceRuleSet {
    /// 🧉 » returns the next match of the rule set from `now`
    pub fn next_match(&self) -> Option<DateTime<Local>> {
        self.next_match_from(Local::now())
    }

    /// 🧉 » returns the next match of the rule set from a given `DateTime`
    pub fn next_match_from(&self, from: DateTime<Local>) -> Option<DateTime<Local>> {
        let next = self._next_match(from);
        match next {
            Some(date) => Some(date.date()),
            None => None,
        }
    }

    /// 🚧 internal
    fn _next_match(&self, from: DateTime<Local>) -> Option<LoolDate<Local>> {
        if !self.is_valid() {
            return None;
        }

        // check year
        if let Some(Rule::Val(year)) = self.year {
            if year < from.year().into() {
                return None;
            }
        }

        let mut next = LoolDate::new(from.clone());
        next.add_second();

        loop {
            // check other possible year values
            if let Some(year_unit) = &self.year {
                if let Rule::Val(year) = year_unit {
                    if *year < next.year() {
                        return None;
                    }
                }

                if !year_unit.matches(next.year()) {
                    next.add_year();
                    next.set_md(1, 1);
                    next.set_hms(0, 0, 0);
                    continue;
                }
            }

            if let Some(month_unit) = &self.month {
                if !month_unit.matches(next.month()) {
                    next.add_month();
                    continue;
                }
            }

            if let Some(day_unit) = &self.day {
                if !day_unit.matches(next.day()) {
                    next.add_day();
                    continue;
                }
            }

            if let Some(dow_unit) = &self.dow {
                if !dow_unit.matches(next.weekday_from_sunday()) {
                    next.add_day();
                    continue;
                }
            }

            if let Some(hour_unit) = &self.hour {
                if !hour_unit.matches(next.hour()) {
                    next.add_hour();
                    continue;
                }
            }

            if let Some(minute_unit) = &self.minute {
                if !minute_unit.matches(next.minute()) {
                    next.add_minute();
                    continue;
                }
            }

            if let Some(second_unit) = &self.second {
                if !second_unit.matches(next.second()) {
                    next.add_second();
                    continue;
                }
            }

            // finally, everything matches, so we get out of the loop
            break;
        }

        Some(next)
    }
}

