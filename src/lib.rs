mod constants;
mod init;
mod network;
mod query;

use std::time::Duration;

use tokio::{io::AsyncReadExt as _, net::TcpStream, time::timeout};

use crate::{
    init::{AppState, init_tokio_runtime},
    network::{Firewall, TransmissionProtocol},
    query::Query,
};

pub fn entry() {
    let runtime = init_tokio_runtime();
    println!("Successfully created tokio runtime.");

    runtime.block_on(async move {
        let mut app_state = init::init();
        println!("Successfully initialized");

        run_downstream(&mut app_state);
        println!(" ");
        println!("RUNNING...");
        println!(" ");

        app_state.join_set.join_all().await;
    });
}

fn run_downstream(app_state: &mut AppState) {
    run_udp(app_state);
    run_tcp(app_state);

    fn run_udp(app_state: &mut AppState) {
        for downstream_socket in &app_state.downstream.udp_sockets {
            let semaphore = app_state.downstream.semaphore.clone();
            let downstream_socket = downstream_socket.clone();

            app_state.join_set.spawn(async move {
                loop {
                    let mut buf = vec![0; constants::MAX_PACKET_SIZE];
                    let (length, origin) = match downstream_socket.recv_from(&mut buf).await {
                        Err(e) => {
                            eprintln!("  error while receiving downstream UDP traffic: {e}");
                            continue;
                        }
                        Ok(v) => v,
                    };
                    buf.truncate(length);

                    if !Firewall::verify_query(&buf) {
                        eprintln!("  warning: firewall rejected DGRAM from {}", &origin.ip());
                        continue;
                    }

                    let permit = match semaphore.clone().try_acquire_owned() {
                        Err(e) => {
                            match e {
                                tokio::sync::TryAcquireError::Closed => {
                                    panic!("Query Semaphore is closed. No new permits can be offered. Unrecoverable state.");
                                }
                                tokio::sync::TryAcquireError::NoPermits => {
                                    eprintln!("  warning: max. number of concurrent queries reached. Dropping query...");
                                    continue;
                                }
                            }
                        }
                        Ok(permit ) => permit,
                    };

                    let query = Query::new(permit, TransmissionProtocol::Udp(downstream_socket.clone()), origin, buf);
                    query.process().await;
                }
            });
        }
    }

    fn run_tcp(app_state: &mut AppState) {
        for downstream_listener in &app_state.downstream.tcp_listeners {
            let semaphore = app_state.downstream.semaphore.clone();
            let downstream_listener = downstream_listener.clone();

            app_state.join_set.spawn(async move {
                loop {
                    let (mut stream, origin) = match downstream_listener.accept().await {
                        Err(e) => {
                            eprintln!("  error while accepting TCP connection: {e}");
                            continue;
                        }
                        Ok(v) => v,
                    };

                    let permit = match semaphore.clone().try_acquire_owned() {
                        Err(e) => {
                            match e {
                                tokio::sync::TryAcquireError::Closed => {
                                    panic!("Query Semaphore is closed. No new permits can be offered. Unrecoverable state.");
                                }
                                tokio::sync::TryAcquireError::NoPermits => {
                                    eprintln!("  warning: max. number of concurrent queries reached. Dropping query...");
                                    continue;
                                }
                            }
                        }
                        Ok(permit ) => permit,
                    };

                    let dns_buf = match read_packet(&mut stream).await {
                        None => continue,
                        Some(v) => v,
                    };

                    if !Firewall::verify_query(&dns_buf) {
                        eprintln!(
                            "  warning: received invalid TCP stream from {}",
                            origin.ip()
                        );
                        return;
                    }

                    let query = Query::new(permit, TransmissionProtocol::Tcp(stream), origin, dns_buf);
                    query.process().await;
                }
            });
        }

        /// Read an entire DNS packet from a [`TcpStream`].
        ///
        /// This method also enforces that the packet be sent within 2 seconds to prevent slowloris attacks.
        ///
        /// # Return
        /// The returned vector has the exact size of the number of bytes read.
        async fn read_packet(stream: &mut TcpStream) -> Option<Vec<u8>> {
            // Read the length prefix that TCP DNS messages carry as defined in
            // RFC 1035 Section 4.2.2 <https://datatracker.ietf.org/doc/html/rfc1035#section-4.2.2>
            let mut prefix_buf = [0_u8; 2];

            // The length of the DNS packet announced by `prefix_buf`
            let mut length: usize = 0;

            // This is the buffer that will contain the actual DNS data
            let mut dns_buf = Vec::<u8>::new();

            if timeout(Duration::from_secs(2), async {
                if let Err(e) = stream.read_exact(&mut prefix_buf).await {
                    eprintln!("  error while reading TCP length prefix: {e}");
                    return;
                };

                length = ((prefix_buf[0] as usize) << 8) | prefix_buf[1] as usize;
                dns_buf = vec![0; length];

                if let Err(e) = stream.read_exact(&mut dns_buf).await {
                    eprintln!("  error while reading TCP DNS buffer: {e}");
                }
            })
            .await
            .is_err()
            {
                eprintln!(
                    "  error: full TCP DNS packet was not received in the legal time frame (2 seconds)"
                );
                return None;
            }

            Some(dns_buf)
        }
    }
}
