//! The server module is the implementation of the network layer. It owns the sockets that communicate with clients but is unaware of DNS.
//!
//! **Rules:** It depends on the [`protocol`](crate::protocol) module to which is hands the data it receives.
mod udp;

use tokio::net::UdpSocket;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
pub use udp::*;

pub async fn start(tracker: TaskTracker, cancel_token: CancellationToken) {
    // Since we don't want to start a server where this value is unknown we intentionally panic here.
    // This applies to all .unwrap() calls inside this function.
    let avail_para = std::thread::available_parallelism().unwrap().get();

    // UDP listeners
    for _ in 0..avail_para {
        let tracker = tracker.clone();
        let cancel_token = cancel_token.clone();

        tracker.clone().spawn(async move {
            let socket = socket2::Socket::new(
                socket2::Domain::IPV4,
                socket2::Type::DGRAM,
                Some(socket2::Protocol::UDP),
            )
            .unwrap();

            socket.set_nonblocking(true).unwrap();
            socket.set_reuse_port(true).unwrap();

            let socket = UdpSocket::from_std(socket.into()).unwrap();

            let listener = UdpListener::new(socket, tracker.clone(), cancel_token.clone());
            listener.listen().await;
        });
    }
}
