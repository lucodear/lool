use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use {
    super::SchedulingRule,
    crate::utils::threads::threadpool::ThreadPool,
    chrono::{DateTime, Local},
};

// TODO: add logging (always as debug)

type Action = Box<dyn FnMut() + Send + Sync + 'static>;

/// 🧉 » a scheduled task
///
/// this structs represents a task that has been scheduled in the scheduler.
///
/// this is returned by the `Scheduler::schedule` method, and can be used to check and control the
/// status of the task.
pub struct ScheduledTask {
    #[allow(dead_code)]
    index: usize,
    name: String,
    action: Action,
    rules: Vec<SchedulingRule>,
    is_running: AtomicBool,
    last_run: Arc<Mutex<Option<DateTime<Local>>>>, // TODO: remaining limits
}

impl ScheduledTask {
    fn run(&mut self) {
        let action = self.action.as_mut();
        action();
    }

    pub fn get_last_run(&self) -> Option<DateTime<Local>> {
        let last_run_lock = self.last_run.lock().unwrap();
        last_run_lock.as_ref().cloned()
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// 🧉 » a task scheduler.
///
/// this struct is responsible for scheduling tasks to be executed at specific times, depending on
/// the rules provided for each task.
///
/// Each task can have n rules, and the task will be executed when any of the rules is met.
pub struct Scheduler {
    pool: ThreadPool,
    tasks: Vec<Arc<Mutex<ScheduledTask>>>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// 🧉 » create a new scheduler
    ///
    /// default constructor, sets the internal thread pool to have 5 threads at most.
    pub fn new() -> Self {
        Self {
            tasks: vec![],
            pool: ThreadPool::create(5).unwrap(),
        }
    }

    /// 🧉 » create a new scheduler
    ///
    /// creates a new scheduler, just like `Scheduler::new`, but with a specific capacity for the
    /// internal thread pool.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tasks: vec![],
            pool: ThreadPool::create(capacity).unwrap(),
        }
    }

    /// 🧉 » schedule a task
    ///
    /// schedules a task to be executed at times determined by the provided rules.
    pub fn schedule<F>(&mut self, name: &str, action: F, rules: SchedulingRule) -> TaskHandler
    where
        F: FnMut() + Send + Sync + 'static,
    {
        self.schedule_many_rules(name, action, vec![rules])
    }

    /// 🧉 » schedule a task
    ///
    /// schedules a task to be executed at times determined by the provided rules.
    pub fn schedule_many_rules<F>(
        &mut self,
        name: &str,
        action: F,
        rules: Vec<SchedulingRule>,
    ) -> TaskHandler
    where
        F: FnMut() + Send + Sync + 'static,
    {
        let index = self.tasks.len();

        let task = Arc::new(Mutex::new(ScheduledTask {
            index,
            name: name.to_string(),
            action: Box::new(action),
            rules,
            is_running: AtomicBool::new(false),
            last_run: Arc::new(Mutex::new(None)),
        }));

        self.tasks.push(task.clone());

        run_in_pool(task.clone(), &self.pool);

        TaskHandler { task: task.clone() }
    }
}

/// 🧉 » task handler
///
/// returned by the `Scheduler::schedule` method,
/// this struct can be used to check and control
/// the status of the task.
pub struct TaskHandler {
    // HACK: holding the task in the TaskHandler is a temporal hack
    //       TaskHandler should hold Atomic references to the important parts of the task
    //       instead. e.g. is_running, last_run, etc.
    //       the problem is with last_run, as its a DateTime, and not a primitive type
    //       we could get around this by instead of holding the DateTime, holding the i64
    //       value (unix timestamp) and then converting it to a DateTime when needed.
    pub task: Arc<Mutex<ScheduledTask>>,
}

/// **main function to run the task in the thread pool**
///
/// it spawns a new job in the thread pool to run the task until the task is no longer scheduled to
/// run.
fn run_in_pool(task_mutex: Arc<Mutex<ScheduledTask>>, pool: &ThreadPool) {
    pool.execute(move || {
        let (mut maybe_next_run, name) = {
            let task = task_mutex.lock().unwrap();
            let rules = &task.rules;

            (get_next_run_time(rules, None), task.name.clone())
        };

        while let Some(run_date) = maybe_next_run {
            let now = Local::now();
            if run_date > now {
                // if the next run is in the future, go to bed until then
                let sleep_until = run_date - now;
                println!(
                    "task {} will run in {} seconds",
                    name,
                    sleep_until.num_seconds()
                );
                std::thread::sleep(sleep_until.to_std().unwrap());
            } else {
                // if the next run is in the past, run the task immediately, probably missed the
                // run time for a few nanos
                println!("task will run in 0 seconds");
            }

            let mut task = task_mutex.lock().unwrap();

            task.last_run = Arc::new(Mutex::new(Some(run_date)));
            task.is_running.store(true, Ordering::SeqCst);
            task.run();
            task.is_running.store(false, Ordering::SeqCst);
            maybe_next_run = get_next_run_time(&task.rules, Some(run_date));
        }
    });
}

/// **get next run time**
///
/// this function takes a list of scheduling rules and a base time, and returns the next time the
/// task should run.
///
/// to determine the next run time, it iterates over the list of rules and calculates the next run
/// time for each of them, returning the earliest of them all.
fn get_next_run_time(
    rules: &Vec<SchedulingRule>,
    from: Option<DateTime<Local>>,
) -> Option<DateTime<Local>> {
    let mut next_run_so_far: Option<DateTime<Local>> = None;

    let base = if let Some(from) = from {
        from
    } else {
        Local::now()
    };

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
