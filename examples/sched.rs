use {
    eyre::{set_hook, DefaultHandler},
    lool::sched::{recur, ruleset, Scheduler},
};

fn setup_eyre() {
    let _ = set_hook(Box::new(DefaultHandler::default_with));
}

fn my_action() {
    let now = chrono::Local::now();
    println!("I'm running at {}", now.format("%Y-%m-%d %H:%M:%S"));

    std::thread::sleep(std::time::Duration::from_secs(15));
}

fn main() {
    setup_eyre();

    let mut sched = Scheduler::new();

    let handler = sched.schedule("test-task", my_action, recur(ruleset().at_second(0)));

    std::thread::sleep(std::time::Duration::from_secs(1));

    loop {
        let is_running = handler.is_running();
        let last_run = handler.get_last_run();
        let next_run = handler.get_next_run();
        let name = handler.name();

        println!("task {name} |--> running: {is_running}, last: {last_run:?}, next: {next_run:?}");

        std::thread::sleep(std::time::Duration::from_secs(60));

        let result = sched.remove(&handler);

        if result.is_ok() {
            println!("task {name} removed");
        } else {
            println!("task {name} not present in the scheduler");
        }
    }
}
