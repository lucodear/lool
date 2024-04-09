use {
    eyre::{Ok, Result},
    lool::utils::threads::threadpool::ThreadPool,
};

fn job(id: usize) {
    // wait for a while
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("job {}", id);
}

fn main() -> Result<()> {
    let pool = ThreadPool::create(4)?;

    for i in 0..10 {
        pool.execute(move || job(i));
    }

    // wait for all jobs to finish
    // another way to do this is to call `pool.join()` but I want to log stuff in the meantime
    loop {
        println!("{:?}", pool);
        if !pool.has_work() {
            break;
        }

        // wait for a while
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
