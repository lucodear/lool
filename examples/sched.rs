use lool::sched::{recur, ruleset, Scheduler};

fn my_action() {
    let now = chrono::Local::now();
    println!("I'm running at {}", now.format("%Y-%m-%d %H:%M:%S"));

    std::thread::sleep(std::time::Duration::from_secs(15));
}

fn main() {
    let mut sched = Scheduler::new();

    let handler = sched.schedule("test-task", my_action, recur(ruleset().at_second(0)));

    std::thread::sleep(std::time::Duration::from_secs(1));

    loop {
        let is_running = handler.is_running();
        let last_run = handler.get_last_run();
        let next_run = handler.get_next_run();
        let name = handler.name();

        println!(
            "task {name} |--> running: {is_running}, last: {last_run:?}, next: {next_run:?}"
        );
        

        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
