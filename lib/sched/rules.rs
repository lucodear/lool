#[cfg(feature = "sched-rule-cron")]
mod cron;
#[cfg(feature = "sched-rule-recurrent")]
mod recurrent;
#[cfg(feature = "sched-rule-recurrent")]
pub use self::recurrent::{many, range, ranges, ruleset, val, RecurrenceRuleSet, Rule};

use chrono::{DateTime, Local};

/// 🧉 » a scheduling rule
///
/// can be:
///   - `Once`: runs only at a specific `chrono::DateTime`
///   - `Repeat`: runs at specific intervals defined by a `RecurrenceRule`
///   - `Cron`: runs at specific intervals defined by a cron expression
pub enum SchedulingRule {
    /// 🧉 » a scheduling rule that makes the task run only once at a specific `chrono::DateTime`
    Once(chrono::DateTime<Local>),

    /// 🧉 » a scheduling rule expressed with a `RecurrenceRule` structure
    #[cfg(feature = "sched-rule-recurrent")]
    Repeat(RecurrenceRuleSet),

    /// 🧉 » a scheduling rule expressed in cron format
    #[cfg(feature = "sched-rule-cron")]
    Cron(String),
}

impl SchedulingRule {
    /// 🧉 » get the next execution time from now
    pub fn next(&self) -> Option<DateTime<Local>> {
        self.next_from(Local::now())
    }

    /// 🧉 » get the next execution time from now
    pub fn next_from(&self, _date: DateTime<Local>) -> Option<DateTime<Local>> {
        match self {
            SchedulingRule::Once(_dt) => {
                unimplemented!()
            }
            #[cfg(feature = "sched-rule-recurrent")]
            SchedulingRule::Repeat(_rule) => {
                unimplemented!()
            }
            #[cfg(feature = "sched-rule-cron")]
            SchedulingRule::Cron(_cron) => {
                unimplemented!()
            }
        }
    }
}
