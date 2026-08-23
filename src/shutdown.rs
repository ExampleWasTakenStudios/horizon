pub async fn wait_for_shutdown_signal() {
    use tokio::signal::unix::{SignalKind, signal};

    let mut sig_int = signal(SignalKind::interrupt()).expect("Failed to bind SIGINT");
    let mut sig_term = signal(SignalKind::terminate()).expect("Failed to bind SIGTERM");

    tokio::select! {
        _ = sig_int.recv() => {
            println!("[MAIN] Received SIGINT. Shutting down...");
        }

        _ = sig_term.recv() => {
            println!("[MAIN] Received SIGTERM. Shutting down...")
        }
    }
}
