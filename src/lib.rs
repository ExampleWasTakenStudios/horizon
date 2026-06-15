use tokio::{self, net, runtime};

mod query_state;
mod ingres_egress_system;
mod protocol;
mod buffer;

pub fn entry() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .name("horizon-tokio-runtime")
        .thread_name("horizon-worker")
        .build()
        .unwrap();

    rt.block_on(async {
        let socket = net::UdpSocket::bind("0.0.0.0:1234").await.unwrap();
        println!("Bound to socket: {:?}", socket);

        loop {
            
        }
    })
}
