use chrono::Weekday;

use {super::RecurrenceRuleSet, crate::sched::rules::Rule};

pub fn ruleset() -> RecurrenceRuleSet {
    RecurrenceRuleSet::recurring()
}

impl RecurrenceRuleSet {
    /// 🧉 » create a new `RecurrenceRuleSet
    pub fn recurring() -> Self {
        Self {
            second: None,
            minute: None,
            hour: None,
            dow: None,
            day: None,
            month: None,
            year: None,
        }
    }

    /// 🧉 » set the second rule
    pub fn seconds_rule(&mut self, rule: Rule<u32>) -> &mut Self {
        self.second = Some(rule);
        self
    }

    /// 🧉 » set the minute rule
    pub fn minutes_rule(&mut self, rule: Rule<u32>) -> &mut Self {
        self.minute = Some(rule);
        self
    }

    /// 🧉 » set the hour rule
    pub fn hours_rule(&mut self, rule: Rule<u32>) -> &mut Self {
        self.hour = Some(rule);
        self
    }

    /// 🧉 » set the full time rule
    pub fn time_rule(
        &mut self,
        hour: Rule<u32>,
        minute: Rule<u32>,
        second: Rule<u32>,
    ) -> &mut Self {
        self.hours_rule(hour).minutes_rule(minute).seconds_rule(second)
    }

    /// 🧉 » set the day of the week rule
    pub fn dow_rule(&mut self, rule: Rule<u32>) -> &mut Self {
        self.dow = Some(rule);
        self
    }

    /// 🧉 » set the day of the month rule
    pub fn day_ryle(&mut self, rule: Rule<u32>) -> &mut Self {
        self.day = Some(rule);
        self
    }

    /// 🧉 » set the month rule
    pub fn month_rule(&mut self, rule: Rule<u32>) -> &mut Self {
        self.month = Some(rule);
        self
    }

    /// 🧉 » set the year rule
    pub fn year_rule(&mut self, rule: Rule<i32>) -> &mut Self {
        self.year = Some(rule);
        self
    }

    /// 🧉 » set the second rule as a single value from primitive
    pub fn at_second(&mut self, value: u32) -> &mut Self {
        self.seconds_rule(Rule::Val(value))
    }

    /// 🧉 » set the minute rule as a single value from primitive
    pub fn at_minute(&mut self, value: u32) -> &mut Self {
        self.minutes_rule(Rule::Val(value))
    }

    /// 🧉 » set the hour rule as a single value from primitive
    pub fn at_hour(&mut self, value: u32) -> &mut Self {
        self.hours_rule(Rule::Val(value))
    }

    /// 🧉 » set the full time rule as a single value from primitives
    pub fn at_time(&mut self, hour: u32, minute: u32, second: u32) -> &mut Self {
        self.time_rule(Rule::Val(hour), Rule::Val(minute), Rule::Val(second))
    }

    /// 🧉 » set the day of the week rule as a single value
    pub fn on_weekday(&mut self, value: Weekday) -> &mut Self {
        self.dow_rule(Rule::Val(value.num_days_from_sunday()))
    }

    /// 🧉 » set the day of the week rule as a single value (`dow` from Sunday 0)
    pub fn on_dow(&mut self, value: u32) -> &mut Self {
        self.dow_rule(Rule::Val(value))
    }

    /// 🧉 » set the day of the week rule as a range between two `Weekday`
    pub fn from_to_weekdays(&mut self, from: Weekday, to: Weekday) -> &mut Self {
        if from == to {
            return self;
        }

        self.dow_rule(Rule::Range(
            from.num_days_from_sunday(),
            to.num_days_from_sunday(),
            1,
        ))
    }

    /// 🧉 » set the day of the week rule as a range between two values (`dow` from Sunday 0)
    pub fn from_to_dow(&mut self, from: u32, to: u32) -> &mut Self {
        if from == to {
            return self;
        }

        self.dow_rule(Rule::Range(from, to, 1))
    }

    /// 🧉 » set the day of the month rule as a single value from primitive
    pub fn on_day(&mut self, value: u32) -> &mut Self {
        self.day_ryle(Rule::Val(value))
    }

    /// 🧉 » set the month rule as a single value from primitive
    pub fn in_month(&mut self, value: u32) -> &mut Self {
        self.month_rule(Rule::Val(value))
    }

    /// 🧉 » set the year rule as a single value from primitive
    pub fn in_year(&mut self, value: i32) -> &mut Self {
        self.year_rule(Rule::Val(value))
    }

    /// 🧉 » set the full date as single values from primitives
    pub fn on_date(&mut self, year: i32, month: u32, day: u32) -> &mut Self {
        self.in_year(year).in_month(month).on_day(day)
    }

    /// 🧉 » set the full datetime as single values from primitives
    pub fn on_datetime(
        &mut self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> &mut Self {
        self.on_date(year, month, day).at_time(hour, minute, second)
    }

    /// 🧉 » check if the rule set is valid
    pub fn is_valid(&self) -> bool {
        // at least one of the rules must be set
        let mut valid = self.second.is_some()
            || self.minute.is_some()
            || self.hour.is_some()
            || self.dow.is_some()
            || self.day.is_some()
            || self.month.is_some()
            || self.year.is_some();

        // month/s should be between 1 and 12
        if let Some(month) = &self.month {
            valid = valid && month.value_is_between(1, 12);
        }

        // day/s of week should be between 0 and 6
        if let Some(dow) = &self.dow {
            valid = valid && dow.value_is_between(0, 6);
        }

        // day/s of month should be between 1 and 31
        if let Some(day) = &self.day {
            // check month overflows if month is also set
            // HACK: I'm already handling day overflows and even leap years at `CronDate`
            //       this might not be necessary anymore... we should test it and see how it goes
            //       without this check (we might want to check for 1..31 as a minimum and that's
            //       all)
            match &self.month {
                Some(month) => {
                    if month.matches(2) {
                        valid = valid && day.value_is_between(1, 29);
                    } else if month.matches(4)
                        || month.matches(6)
                        || month.matches(9)
                        || month.matches(11)
                    {
                        valid = valid && day.value_is_between(1, 30);
                    } else {
                        valid = valid && day.value_is_between(1, 31);
                    }
                }
                None => valid = valid && day.value_is_between(1, 31),
            }
        }

        // hour/s should be between 0 and 23
        if let Some(hour) = &self.hour {
            valid = valid && hour.value_is_between(0, 23);
        }

        // minute/s should be between 0 and 59
        if let Some(minute) = &self.minute {
            valid = valid && minute.value_is_between(0, 59);
        }

        // second/s should be between 0 and 59
        if let Some(second) = &self.second {
            valid = valid && second.value_is_between(0, 59);
        }

        valid
    }
}
