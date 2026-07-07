use tokio::sync::{
    mpsc::{self},
    oneshot,
};

use crate::{
    MAX_PACKET_SIZE, protocol::packet::DnsPacket, query_state::QueryState,
    systems::NetworkTrafficMessage,
};

pub struct MpscChannel<T> {
    pub tx: mpsc::Sender<T>,
    pub rx: mpsc::Receiver<T>,
}

pub struct OneshotChannel<T> {
    pub tx: oneshot::Sender<T>,
    pub rx: oneshot::Receiver<T>,
}

/// `T`: The type of the payload that is sent by the sender of the message. <br>
/// `V`: The type of the payload of the returning message.
pub struct HalfDuplexMessage<T, V> {
    pub payload: T,
    pub return_channel: oneshot::Sender<V>,
}

/// Holds all channels used for inter-system communications.
/// The naming scheme is in rx_tx indicating which system acts as the transmitting side and which system receives the channel.
pub struct InterSystemCommunicationChannels {
    /// Used by the IES to receive data directly from the socket (main-thread).
    pub ies_ingress_channel: MpscChannel<NetworkTrafficMessage>,
    /// Used by the IES to send data directly to the socket (main-thread).
    pub ies_egress_channel: MpscChannel<NetworkTrafficMessage>,
    /// Used by the IES to send a `QueyState` to the DPS.
    pub ies_to_dps: MpscChannel<QueryState>,
    /// Used by the DPS to send a `QueryState` to the SRS.
    pub dps_to_srs: MpscChannel<QueryState>,
    /// Used by the SRS to send a `DnsPacket` data structure to the IES to be forwarded to an upstream resolver.
    ///
    /// `K`: A reference to the DNS question <br>
    /// `V`: the owned `DnsPacket` of the upstream response
    pub ies_resolving_channel: MpscChannel<HalfDuplexMessage<DnsPacket, Option<DnsPacket>>>,
    /// Used by the IES to receive data it should send to the querying client.
    pub ies_answer_channel: MpscChannel<QueryState>,
}

pub fn init_msg_channels() -> InterSystemCommunicationChannels {
    let (ies_ingress_channel_tx, ies_ingress_channel_rx) = mpsc::channel(100);
    let (ies_egress_channel_tx, ies_egress_rx) = mpsc::channel(100);
    let (ies_to_dps_tx, ies_to_dps_rx) = mpsc::channel(100);
    let (dps_to_srs_tx, dps_to_srs_rx) = mpsc::channel(100);
    let (ies_resolving_channel_tx, ies_resolving_channel_rx) = mpsc::channel(100);
    let (ies_answer_channel_tx, ies_answer_channel_rx) = mpsc::channel(100);

    InterSystemCommunicationChannels {
        ies_ingress_channel: MpscChannel {
            tx: ies_ingress_channel_tx,
            rx: ies_ingress_channel_rx,
        },
        ies_egress_channel: MpscChannel {
            tx: ies_egress_channel_tx,
            rx: ies_egress_rx,
        },
        ies_to_dps: MpscChannel {
            tx: ies_to_dps_tx,
            rx: ies_to_dps_rx,
        },
        dps_to_srs: MpscChannel {
            tx: dps_to_srs_tx,
            rx: dps_to_srs_rx,
        },
        ies_resolving_channel: MpscChannel {
            tx: ies_resolving_channel_tx,
            rx: ies_resolving_channel_rx,
        },
        ies_answer_channel: MpscChannel {
            tx: ies_answer_channel_tx,
            rx: ies_answer_channel_rx,
        },
    }
}
