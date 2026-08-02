mod downstream;

pub use downstream::*;

pub struct AppState {
    pub downstream: DownstreamAppState,
}

pub fn init() -> AppState {
    AppState {
        downstream: downstream::init(),
    }
}

pub fn init_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .name("horizon-runtime")
        .thread_name("horizon-worker")
        .build()
        .unwrap()
}
