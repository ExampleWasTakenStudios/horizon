use tokio::{self, net, runtime};

pub mod protocol;
pub mod context;
pub mod buffer;

pub fn entry() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .name("horizon-tokio-runtime")
        .thread_name("horizon-worker")
        .build()
        .unwrap();

    rt.block_on(async {
        let socket = net::UdpSocket::bind("0.0.0.0:1234").await.unwrap();
        println!("Bound to socket: {:?}", socket.local_addr());
        let mut buffer: [u8; 512] = [0; 512];

        loop {
             let (_, source) = match socket.recv_from(&mut buffer).await {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Failure while receiving datagram...should send a SERVFAIL response code.\n{:#?}", e);
                    continue;
                }
             };

             tokio::spawn(async move {
                let mut buffer = buffer::PacketBuffer::from_raw_buffer(buffer);
                let packet = protocol::DnsPacket::parse_from(&mut buffer);

             });
        }
    })
}
