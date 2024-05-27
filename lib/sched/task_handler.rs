use {
    super::rules::SchedulingRule,
    chrono::{DateTime, Local},
    core::fmt,
    std::{
        fmt::{Debug, Formatter},
        sync::{
            atomic::{AtomicBool, AtomicPtr, Ordering},
            Arc,
        },
    },
};

/// 🧉 » task handler
///
/// returned by the `Scheduler::schedule` method,
/// this struct can be used to check and control
/// the status of the task.
#[derive(Clone)]
pub struct TaskHandler {
    pub(crate) name: String,
    pub(crate) rules: Arc<Vec<SchedulingRule>>,
    pub(crate) is_running: Arc<AtomicBool>,
    pub(crate) is_stopped: Arc<AtomicBool>,
    pub(crate) is_removed: Arc<AtomicBool>,
    pub(crate) last_run: Arc<AtomicPtr<DateTime<Local>>>,
}

impl Debug for TaskHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskHandler")
            .field("name", &self.name)
            .field("is_running", &self.is_running)
            .field("is_stopped", &self.is_stopped)
            .field("is_removed", &self.is_removed)
            .field("last_run", &self.get_last_run())
            .field("next_run", &self.get_next_run())
            .finish()
    }
}

impl TaskHandler {
    /// 🧉 » last run date
    ///
    /// returns a `DateTime<Local>` representing the last time the task was run or None if the task
    /// has never been run before.
    pub fn get_last_run(&self) -> Option<DateTime<Local>> {
        let last_run = self.last_run.load(Ordering::Relaxed);
        if last_run.is_null() {
            None
        } else {
            Some(unsafe { *last_run })
        }
    }

    /// 🧉 » next run date
    ///
    /// returns a `DateTime<Local>` representing the next time the task is scheduled to run
    pub fn get_next_run(&self) -> Option<DateTime<Local>> {
        if self.is_active() {
            return get_next_run_time(&self.rules, None);
        }

        None
    }

    /// 🧉 » is running?
    ///
    /// returns a `bool` indicating if the task is currently running in this moment
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    /// 🧉 » name
    ///
    /// returns the name of the task
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 🧉 » is active?
    ///
    /// returns a `bool` indicating if the task is active
    pub fn is_active(&self) -> bool {
        !self.is_stopped.load(Ordering::Relaxed) && !self.is_removed.load(Ordering::Relaxed)
    }

    /// 🧉 » is stopped?
    ///
    /// returns a `bool` indicating if the task has been stopped
    ///
    /// a task being stopped means that it has been paused, but not removed from the scheduler.
    /// So, although it's not running, it's still in the scheduler and can be resumed.
    pub fn is_stopped(&self) -> bool {
        self.is_stopped.load(Ordering::Relaxed)
    }

    /// 🧉 » is removed?
    ///
    /// returns a `bool` indicating if the task has been removed
    ///
    /// once a task is removed, it's no longer in the scheduler and can't be resumed.
    pub fn is_removed(&self) -> bool {
        self.is_removed.load(Ordering::Relaxed)
    }
}

/// **get next run time**
///
/// this function takes a list of scheduling rules and a base time, and returns the next time the
/// task should run.
///
/// to determine the next run time, it iterates over the list of rules and calculates the next run
/// time for each of them, returning the earliest of them all.
pub(crate) fn get_next_run_time(
    rules: &Vec<SchedulingRule>,
    from: Option<DateTime<Local>>,
) -> Option<DateTime<Local>> {
    let mut next_run_so_far: Option<DateTime<Local>> = None;

    let base = if let Some(from) = from { from } else { Local::now() };

    for rule in rules {
        let rule_next_run = rule.next_from(base);

        if let Some(next_run) = rule_next_run {
            if let Some(d) = next_run_so_far {
                if next_run < d {
                    next_run_so_far = Some(next_run);
                }
            } else {
                next_run_so_far = Some(next_run);
            }
        }
    }

    next_run_so_far
}
