use lool::sched::{recur, ruleset, Scheduler};

fn my_action() {
    let now = chrono::Local::now();
    println!("I'm running at {}", now.format("%Y-%m-%d %H:%M:%S"));
}

fn main() {
    let mut sched = Scheduler::new();

    let handler = sched.schedule("test-task", my_action, recur(ruleset().at_second(0)));

    std::thread::sleep(std::time::Duration::from_secs(1));

    loop {
        {
            let task = handler.task.lock().unwrap();

            let is_running = task.is_running();
            let last_run = task.get_last_run();
            let name = task.name();

            println!(
                "task {} |--> is running: {}, last run: {:?}",
                name, is_running, last_run
            );
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
