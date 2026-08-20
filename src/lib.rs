mod constants;
mod error;
mod protocol;
mod query;
mod ring;
mod server;

use tokio_util::task::TaskTracker;

pub fn entry() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .name("main-runtime")
        .thread_name("horizon-tokio")
        .build()
        .unwrap();

    println!("Successfully created tokio runtime.");

    runtime.block_on(async move {
        let task_tracker = TaskTracker::new();

        server::start(task_tracker.clone()).await;

        println!(" ");
        println!("RUNNING...");
        println!(" ");
    });
}
