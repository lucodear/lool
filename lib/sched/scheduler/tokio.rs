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
        future::Future,
        pin::Pin,
        sync::{
            atomic::{AtomicBool, AtomicPtr, Ordering},
            Arc,
        },
    },
    tokio::{spawn, sync::Mutex, task::JoinHandle, time::sleep},
};

struct ScheduledTask {
    name: String,
    action: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    rules: Arc<Vec<SchedulingRule>>,
    is_running: Arc<AtomicBool>,
    is_stopped: Arc<AtomicBool>,
    is_removed: Arc<AtomicBool>,
    last_run: Arc<AtomicPtr<DateTime<Local>>>,
}

impl ScheduledTask {
    async fn run(&mut self) {
        self.action.as_mut().await;
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

    /// 🧉 » schedule an async task
    ///
    /// schedules an async function to be executed as a task at time intervals determined by the
    /// provided     /// rules.
    pub async fn schedule<F, Fut, Str>(
        &mut self,
        name: Str,
        func: F,
        rules: SchedulingRule,
    ) -> TaskHandler
    where
        F: FnMut() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
        Str: AsRef<str>,
    {
        let name = name.as_ref();
        let mut func = func;
        self.schedule_many_rules(name, func(), vec![rules]).await
    }

    /// 🧉 » schedule a future
    ///
    /// schedules a future to be executed as a task at time intervals determined by the provided
    /// rules.
    pub async fn schedule_fut<Fut, Str>(
        &mut self,
        name: Str,
        future: Fut,
        rules: SchedulingRule,
    ) -> TaskHandler
    where
        Fut: Future<Output = ()> + Send + 'static,
        Str: AsRef<str>,
    {
        let name = name.as_ref();
        self.schedule_many_rules(name, future, vec![rules]).await
    }

    /// 🧉 » schedule a task
    ///
    /// schedules a task to be executed at times determined by the provided rules.
    pub async fn schedule_many_rules<Fut>(
        &mut self,
        name: &str,
        future: Fut,
        rules: Vec<SchedulingRule>,
    ) -> TaskHandler
    where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Mutex::new(ScheduledTask {
            name: name.to_string(),
            action: Box::pin(future),
            rules: Arc::new(rules),
            is_running: Arc::new(AtomicBool::new(false)),
            is_stopped: Arc::new(AtomicBool::new(false)),
            is_removed: Arc::new(AtomicBool::new(false)),
            last_run: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
        }));

        self.tasks.insert(name.to_string(), task.clone());

        spawn_task(task.clone());

        let handler: TaskHandler = {
            let task = task.lock().await;
            task.make_handler()
        };

        handler
    }

    /// 🧉 » stop a task
    pub async fn stop(&mut self, handler: &TaskHandler) -> Result<()> {
        // get the task from the tasks map
        let task = self.tasks.get(handler.name());

        if let Some(task) = task {
            let task = task.lock().await;
            task.is_stopped.store(true, Ordering::Relaxed);
            debug!("task {} has been stopped", handler.name());
            Ok(())
        } else {
            Err(eyre!("task {} was not found", handler.name()))
        }
    }

    /// 🧉 » resume a task
    pub async fn resume(&mut self, handler: &TaskHandler) -> Result<()> {
        // get the task from the tasks map
        let task = self.tasks.get(handler.name());

        if let Some(task) = task {
            let task = task.lock().await;
            task.is_stopped.store(false, Ordering::Relaxed);
            debug!("task {} has been resumed", handler.name());
            Ok(())
        } else {
            Err(eyre!("task {} was not found", handler.name()))
        }
    }

    /// 🧉 » remove a task
    pub async fn remove(&mut self, handler: &TaskHandler) -> Result<()> {
        // get the task from the tasks map
        let task = self.tasks.remove(handler.name());

        if let Some(task) = task {
            let task = task.lock().await;
            task.is_removed.store(true, Ordering::Relaxed);
            debug!("task {} has been removed", handler.name());
            Ok(())
        } else {
            handler.is_removed.store(true, Ordering::Relaxed);
            Err(eyre!("task {} was not found", handler.name()))
        }
    }
}

/// **main function to spawn a task in tokio**
///
/// it spawns a new tokio task and runs the task according to its scheduling rules.
fn spawn_task(task_mutex: Arc<Mutex<ScheduledTask>>) -> JoinHandle<()> {
    spawn(async move {
        let (mut maybe_next_run, name) = {
            let task = task_mutex.lock().await;
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

                sleep(sleep_until.to_std().unwrap()).await;
            } else {
                // if the next run is in the past, run the task immediately, probably missed the
                // run time for a few nanos
                debug!("task will run in 0 seconds");
            }

            let mut task = task_mutex.lock().await;

            if task.is_active() {
                let run_date_box = Box::new(run_date);
                let run_date_raw = Box::into_raw(run_date_box);

                task.last_run.store(run_date_raw, Ordering::Relaxed);
                task.is_running.store(true, Ordering::SeqCst);
                task.run().await;
                task.is_running.store(false, Ordering::SeqCst);
            }

            if !task.is_removed() {
                maybe_next_run = get_next_run_time(&task.rules, Some(run_date));
            } else {
                maybe_next_run = None;
            }
        }

        debug!("task {} has finished", name);
    })
}
