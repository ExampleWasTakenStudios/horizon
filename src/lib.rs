use std::io::Write;

use tokio::{self, net, runtime};

use crate::{buffer::PacketBuffer, protocol::DnsPacket};

pub mod protocol;
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
        println!("Bound to socket: {:?}", socket);
        let mut buffer: [u8; 512] = [0; 512];

        loop {
             let (bytes, source) = match socket.recv_from(&mut buffer).await {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Failure while receiving datagram...should send a SERVFAIL response code.\n{:#?}", e);
                    continue;
                }
             };

             tokio::spawn(async move {
                let mut buffer = PacketBuffer::from_raw_buffer(buffer);
                let packet = DnsPacket::parse_from(&mut buffer).unwrap();

                println!("Received {} bytes from {} in packet: {:#?}", bytes, source, packet);
                let labels = &packet.questions[0].name.labels;

                println!("Query:");
                let mut stdout = std::io::stdout();
                for label in labels {
                    stdout.write_all(label).unwrap();
                    stdout.flush().unwrap();
                }

             });
        }
    })
}