use {
    crate::sched::{
        rules::SchedulingRule,
        task_handler::{get_next_run_time, TaskHandler},
    },
    chrono::{DateTime, Local},
    eyre::{eyre, Result},
    log::debug,
    std::{
        collections::HashMap,
        sync::{
            atomic::{AtomicBool, AtomicPtr, Ordering},
            Arc, Mutex,
        },
        thread::{self, JoinHandle},
    },
};

type Action = Box<dyn FnMut() + Send + Sync + 'static>;

/// 🧉 » a scheduled task
///
/// this structs represents a task that has been scheduled in the scheduler.
///
/// this is returned by the `Scheduler::schedule` method, and can be used to check and control the
/// status of the task.
pub struct ScheduledTask {
    #[allow(dead_code)]
    name: String,
    action: Action,
    rules: Arc<Vec<SchedulingRule>>,
    is_running: Arc<AtomicBool>,
    is_stopped: Arc<AtomicBool>,
    is_removed: Arc<AtomicBool>,
    last_run: Arc<AtomicPtr<DateTime<Local>>>,
}

impl ScheduledTask {
    fn run(&mut self) {
        let action = self.action.as_mut();
        action();
    }

    fn make_handler(&self) -> TaskHandler {
        TaskHandler {
            name: self.name.clone(),
            rules: self.rules.clone(),
            is_running: self.is_running.clone(),
            is_stopped: self.is_stopped.clone(),
            is_removed: self.is_removed.clone(),
            last_run: self.last_run.clone(),
        }
    }

    fn is_active(&self) -> bool {
        !self.is_stopped.load(Ordering::Relaxed) && !self.is_removed.load(Ordering::Relaxed)
    }

    fn is_removed(&self) -> bool {
        self.is_removed.load(Ordering::Relaxed)
    }
}

/// 🧉 » a task scheduler.
///
/// this struct is responsible for scheduling tasks to be executed at specific times, depending on
/// the rules provided for each task.
///
/// Each task can have n rules, and the task will be executed when any of the rules is met.
pub struct Scheduler {
    tasks: HashMap<String, Arc<Mutex<ScheduledTask>>>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// 🧉 » create a new scheduler
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
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
        let task = Arc::new(Mutex::new(ScheduledTask {
            name: name.to_string(),
            action: Box::new(action),
            rules: Arc::new(rules),
            is_running: Arc::new(AtomicBool::new(false)),
            is_stopped: Arc::new(AtomicBool::new(false)),
            is_removed: Arc::new(AtomicBool::new(false)),
            last_run: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
        }));

        self.tasks.insert(name.to_string(), task.clone());

        // launch the task in its own thread
        spawn_task(task.clone());

        let handler: TaskHandler = {
            let task = task.lock().unwrap();
            task.make_handler()
        };

        handler
    }

    /// 🧉 » stop a task
    pub fn stop(&mut self, handler: &TaskHandler) -> Result<()> {
        // get the task from the tasks map
        let task = self.tasks.get(handler.name());

        if let Some(task) = task {
            if let Ok(task) = task.lock() {
                task.is_stopped.store(true, Ordering::Relaxed);
                debug!("task {} has been stopped", handler.name());
                Ok(())
            } else {
                Err(eyre!("error stopping task {}", handler.name()))
            }
        } else {
            Err(eyre!("task {} was not found", handler.name()))
        }
    }

    /// 🧉 » resume a task
    pub fn resume(&mut self, handler: &TaskHandler) -> Result<()> {
        // get the task from the tasks map
        let task = self.tasks.get(handler.name());

        if let Some(task) = task {
            if let Ok(task) = task.lock() {
                task.is_stopped.store(false, Ordering::Relaxed);
                debug!("task {} has been resumed", handler.name());
                Ok(())
            } else {
                Err(eyre!("error resuming task {}", handler.name()))
            }
        } else {
            Err(eyre!("task {} was not found", handler.name()))
        }
    }

    /// 🧉 » remove a task
    pub fn remove(&mut self, handler: &TaskHandler) -> Result<()> {
        // get the task from the tasks map
        let task = self.tasks.remove(handler.name());

        if let Some(task) = task {
            if let Ok(task) = &mut task.lock() {
                task.is_removed.store(true, Ordering::Relaxed);
                debug!("task {} has been removed", handler.name());

                Ok(())
            } else {
                Err(eyre!("error removing task {}", handler.name()))
            }
        } else {
            handler.is_removed.store(true, Ordering::Relaxed);
            Err(eyre!("task {} was not found", handler.name()))
        }
    }
}

/// **main function to run the task in its own thread**
///
/// it creates a new thread and runs the task according to its scheduling rules.
fn spawn_task(task_mutex: Arc<Mutex<ScheduledTask>>) -> JoinHandle<()> {
    let thread = thread::spawn(move || {
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
                debug!(
                    "task {} will run in {} seconds",
                    name,
                    sleep_until.num_seconds()
                );

                std::thread::sleep(sleep_until.to_std().unwrap());
            } else {
                // if the next run is in the past, run the task immediately, probably missed the
                // run time for a few nanos
                debug!("task will run in 0 seconds");
            }

            let mut task = task_mutex.lock().unwrap();

            if task.is_active() {
                let run_date_box = Box::new(run_date);
                let run_date_raw = Box::into_raw(run_date_box);

                task.last_run.store(run_date_raw, Ordering::Relaxed);
                task.is_running.store(true, Ordering::SeqCst);
                task.run();
                task.is_running.store(false, Ordering::SeqCst);
            }

            if !task.is_removed() {
                maybe_next_run = get_next_run_time(&task.rules, Some(run_date));
            } else {
                maybe_next_run = None;
            }
        }

        debug!("task {} has finished", name);
    });

    thread
}
