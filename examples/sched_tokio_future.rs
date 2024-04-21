use {
    eyre::{set_hook, DefaultHandler, Result},
    lool::{
        logger::ConsoleLogger,
        sched::{recur, ruleset, scheduler::tokio::Scheduler},
    },
    tokio::time::sleep,
};

fn setup_eyre() {
    let _ = set_hook(Box::new(DefaultHandler::default_with));
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    setup_eyre();
    ConsoleLogger::default_setup(log::Level::Trace, "lool::sched::recur")?;

    let mut sched = Scheduler::new();
    log::debug!("scheduler created");
    
    let now = chrono::Local::now();

    let handler = sched.schedule_fut("test-task", async move {
        println!("I'm running at {}", &now.format("%Y-%m-%d %H:%M:%S"));
    }, recur(ruleset().at_second(0))).await;

    sleep(std::time::Duration::from_secs(1)).await;

    loop {
        let name = handler.name();
        println!("{:?}", handler);

        handler.get_next_run();

        sleep(std::time::Duration::from_secs(60)).await;

        let result = sched.remove(&handler).await;

        if result.is_ok() {
            println!("task {name} removed");
        } else {
            println!("task {name} not present in the scheduler");
        }
    }
}
