mod downstream;

use tokio::task::JoinSet;
pub use downstream::*;

pub struct AppState {
    pub downstream: DownstreamAppState,
    pub join_set: JoinSet<()>,
}

pub fn init() -> AppState {
    AppState {
        downstream: downstream::init(),
        join_set: JoinSet::new(),
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
