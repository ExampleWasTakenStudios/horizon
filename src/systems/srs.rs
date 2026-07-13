use std::time::Duration;

use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        oneshot,
    },
    task::JoinSet,
};

use crate::{
    buffer::PacketBuffer, isc::HalfDuplexMessage, protocol::{DnsHeader, packet::DnsPacket}, query_state::QueryState,
};

/// An IES Resolve Command is sent to the IES to forward a packet to the specified resolver.
///
/// It contains the timeout after which the IES should clear the memory associated with the query and the `DnsPacket` representing the query.
pub struct IesResolveCommand {
    pub timeout: Duration,
    pub packet: DnsPacket,
}

pub struct StubResolverSystem {
    dps_to_srs_rx: Receiver<QueryState>,
    upstream_resolver_tx: Sender<HalfDuplexMessage<IesResolveCommand, Option<DnsPacket>>>,
    ies_answer_tx: Sender<QueryState>,
}

impl StubResolverSystem {
    pub async fn run(mut self) -> JoinSet<()> {
        let mut join_set = JoinSet::<()>::new();

        join_set.spawn(async move {
            loop {
                let mut query_state = match self.dps_to_srs_rx.recv().await {
                    None => panic!("DPS-SRS channel closed unexpectedly."),
                    Some(query_state) => query_state,
                };

                println!("[SRS] RECEIVED QUERY FROM DPS");

                // Create return channel
                let (return_channel_tx, return_channel_rx) =
                    oneshot::channel::<Option<DnsPacket>>();

                let ies_command = IesResolveCommand {
                    packet: query_state.get_packet().clone(),
                    timeout: Duration::from_secs(10),
                };

                let ies_message = HalfDuplexMessage {
                    payload: ies_command,
                    return_channel: return_channel_tx,
                };

                // Send `DnsPacket` to IES to forward it to the upstream resolver
                println!("[SRS] SENDING QUERY TO IES UPSTREAM EGRESS TASK");
                if let Err(e) = self.upstream_resolver_tx.try_send(ies_message) {
                    match e {
                        tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                            panic!("IES Upstream Resolver channel closed unexpectedly.")
                        }
                        tokio::sync::mpsc::error::TrySendError::Full(_) => {
                            eprintln!("IES Upstream Resolver channel overloaded. Dropping...");
                            return;
                        }
                    }
                }

                // Await response from the IES with the resolved
                let resolved_packet = match return_channel_rx.await {
                    Err(e) => {
                        eprintln!("[SRS] IES->SRS return channel closed before sending a message. Error: {e}");
                        return;
                    }
                    Ok(packet) => match packet {
                        None => {
                            println!("IES was unable to resolve query. Dropping.");
                            return;
                        }
                        Some(packet) => packet,
                    },
                };

                println!("[SRS] RECEIVED RESOLVED PACKET -> ATTACHING TO QUERY STATE AND SENDING TO IES ANSWER CHANNEL");

                // Attach response to `QueryState`
                query_state.set_response(resolved_packet);

                // Send query state to IES answer channel
                if let Err(e) = self.ies_answer_tx.try_send(query_state) {
                    match e {
                        tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                            panic!("IES Answer channel closed unexpectedly.");
                        }
                        tokio::sync::mpsc::error::TrySendError::Full(_) => {
                            eprintln!("IES answer channel overloaded. Dropping.");
                            return;
                        }
                    }
                };
            }
        });

        join_set
    }

    pub fn new(
        dps_to_srs_rx: Receiver<QueryState>,
        upstream_resolver_tx: Sender<HalfDuplexMessage<IesResolveCommand, Option<DnsPacket>>>,
        ies_answer_tx: Sender<QueryState>,
    ) -> Self {
        Self {
            dps_to_srs_rx,
            upstream_resolver_tx,
            ies_answer_tx,
        }
    }
}
