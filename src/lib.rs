mod constants;
mod error;
mod protocol;
mod query;
mod server;
mod shutdown;

use std::process;

use tokio::signal;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::shutdown::wait_for_shutdown_signal;

pub fn entry() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .name("main-runtime")
        .thread_name("horizon-tokio")
        .build()
        .unwrap();

    println!("Successfully created tokio runtime.");

    runtime.block_on(async move {
        let cancel_token = CancellationToken::new();
        let tracker = TaskTracker::new();

        server::start(tracker.clone(), cancel_token.clone()).await;

        println!(" ");
        println!("RUNNING...");
        println!(" ");

        wait_for_shutdown_signal().await;
        println!("[MAIN] Received Ctrl+C");
        tracker.close();
        cancel_token.cancel();

        tracker.wait().await;
        println!("[MAIN] All systems shutdown down. Exiting...");
        process::exit(0);
    });
}
