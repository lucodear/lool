<p align="center"><img src="../../../.github/img/logo-utils-threads.svg" width="200"></p>

<br>
<br>
<br>

<p align="center">
<b>lool » <code>utils.threads</code></b> contains utilities for working with threads in Rust.
</p>

<br>
<br>
<br>

# Installation

This library is for internal use. And as such, it's only published privately.

```bash
cargo add lool --registry=lugit --features utils.threads
```

# Utilities

## Thread Pool

This is a super basic and lightweight thread pool implementation to use when there's not need for
a more complex solution.

This is basically, the `ThreadPool` implementation from the book ["The Rust Programming Language" (chapter 20)](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
by Steve Klabnik and Carol Nichols, but with some modifications and additions, inspired by the
[threadpool](https://crates.io/crates/threadpool) crate.

The `ThreadPool` implemented by the book waits for all threads to finish when the pool is dropped.

This behavior is ok for some cases, for example, for a web server that needs to wait for all
currently active requests to finish before shutting down. But it might not be the desired behavior
for other cases (e.g. long-running tasks when we can't wait for a thread to finish).

To keep this library generic enough, the `ThreadPool` implementation here exposes a `join` method
that explicitly waits for all threads to finish.

### More robust solutions

-   [rayon](https://crates.io/crates/rayon): a data parallelism library for Rust.
-   [threadpool](https://crates.io/crates/threadpool): a simple thread pool implementation (quite
    abandoned).

# Usage

```rust
use lool::utils::threads::ThreadPool;

fn main() {
    // create a thread pool with 4 threads
    let pool = ThreadPool::new(4);

    // spawn a bunch of tasks
    for i in 0..8 {
        pool.execute(move || {
            println!("task {}", i);
        });
    }

    // wait for all tasks to finish
    // unlike the book's implementation, this doesn't happen automatically when the pool is dropped
    // e.g. when the program ends, so we need to join explicitly
    pool.join();
}
```

Also, see the [`threadpool.rs`](../../../examples/threadpool.rs) example.
