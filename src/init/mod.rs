mod downstream;
mod upstream;

pub use downstream::*;
pub use upstream::*;

pub struct AppState {
    pub downstream: DownstreamAppState,
    pub upstream: UpstreamAppState,
}

pub fn init() -> AppState {
    AppState {
        downstream: downstream::init(),
        upstream: upstream::init(),
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
