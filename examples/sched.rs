use {
    eyre::{set_hook, DefaultHandler, Result},
    lool::{
        logger::ConsoleLogger,
        sched::{recur, ruleset, Scheduler},
    },
};

fn setup_eyre() {
    let _ = set_hook(Box::new(DefaultHandler::default_with));
}

fn my_action() {
    let now = chrono::Local::now();
    println!("I'm running at {}", now.format("%Y-%m-%d %H:%M:%S"));

    std::thread::sleep(std::time::Duration::from_secs(15));
}

fn main() -> Result<()> {
    setup_eyre();
    ConsoleLogger::default_setup(log::Level::Trace, "lool::sched::recur")?;

    let mut sched = Scheduler::new();
    log::debug!("scheduler created");

    let handler = sched.schedule("test-task", my_action, recur(ruleset().at_second(0)));

    std::thread::sleep(std::time::Duration::from_secs(1));

    loop {
        let name = handler.name();
        println!("{:?}", handler);

        std::thread::sleep(std::time::Duration::from_secs(60));

        let result = sched.remove(&handler);

        if result.is_ok() {
            println!("task {name} removed");
        } else {
            println!("task {name} not present in the scheduler");
        }
    }
}
