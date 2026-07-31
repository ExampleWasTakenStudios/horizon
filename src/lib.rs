mod constants;
mod init;
mod network;

use crate::init::{init, init_tokio_runtime};

pub fn entry() {
    let runtime = init_tokio_runtime();
    println!("Successfully created tokio runtime.");

    runtime.block_on(async move {
        let app_state = init();

        println!("Successfully initialized");
        app_state.global_join_set.join_all().await;
    });
}
