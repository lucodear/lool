#[cfg(feature = "sched.rule-recurrence")]
mod recurrent;
#[cfg(feature = "sched.rule-recurrence")]
pub use self::recurrent::{many, range, ranges, ruleset, val, RecurrenceRuleSet, Rule};

#[cfg(feature = "sched.rule-cron")]
mod cron;
#[cfg(feature = "sched.rule-cron")]
pub use self::cron::Cron;

use {
    chrono::{DateTime, Local},
    eyre::Result,
    std::fmt::Debug,
};

/// 🧉 » a scheduling rule
///
/// can be:
///   - `Once`: runs only at a specific `chrono::DateTime`
///   - `Repeat`: runs at specific intervals defined by a `RecurrenceRule`
///   - `Cron`: runs at specific intervals defined by a cron expression
#[derive(Clone, Debug)]
pub enum SchedulingRule {
    /// 🧉 » a scheduling rule that makes the task run only once at a specific `chrono::DateTime`
    Once(chrono::DateTime<Local>),

    /// 🧉 » a scheduling rule expressed with a `RecurrenceRule` structure
    #[cfg(feature = "sched.rule-recurrence")]
    Repeat(RecurrenceRuleSet),

    /// 🧉 » a scheduling rule expressed in cron format
    #[cfg(feature = "sched.rule-cron")]
    Cron(Cron),
}

impl SchedulingRule {
    /// 🧉 » get the next execution time from now
    pub fn next(&self) -> Option<DateTime<Local>> {
        self.next_from(Local::now())
    }

    /// 🧉 » get the next execution time from now
    pub fn next_from(&self, base: DateTime<Local>) -> Option<DateTime<Local>> {
        match self {
            SchedulingRule::Once(date) => {
                if date > &base {
                    Some(*date)
                } else {
                    None
                }
            }

            #[cfg(feature = "sched.rule-recurrence")]
            SchedulingRule::Repeat(rule) => rule.next_match_from(base),

            #[cfg(feature = "sched.rule-cron")]
            SchedulingRule::Cron(pattern) => {
                let next = pattern.find_next_occurrence(&base, false);
                next.ok()
            }
        }
    }
}

/// 🧉 » create a new `SchedulingRule` that runs at specific intervals defined by a cron expression
#[cfg(feature = "sched.rule-cron")]
pub fn cron(pattern: &str) -> Result<SchedulingRule> {
    let cron = Cron::new(pattern)?;
    Ok(SchedulingRule::Cron(cron))
}

/// 🧉 » create a new `SchedulingRule` that runs at specific intervals defined by a `RecurrenceRule`
#[cfg(feature = "sched.rule-recurrence")]
pub fn recur(rule: &RecurrenceRuleSet) -> SchedulingRule {
    SchedulingRule::Repeat(rule.clone())
}

/// 🧉 » create a new `SchedulingRule` that runs only once at a specific `chrono::DateTime`
pub fn once(datetime: DateTime<Local>) -> SchedulingRule {
    SchedulingRule::Once(datetime)
}
