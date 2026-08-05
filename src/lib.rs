mod constants;
mod error;
mod ring;
mod server;
mod worker;

use std::time::Instant;

pub fn entry() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .name("main-runtime")
        .thread_name("horizon-tokio")
        .build()
        .unwrap();

    println!("Successfully created tokio runtime.");

    runtime.block_on(async move {
        let start_init = Instant::now();
        let end_init = Instant::duration_since(&Instant::now(), start_init);

        println!("Successfully initialized in {:?}", end_init);

        println!(" ");
        println!("RUNNING...");
        println!(" ");
    });
}
