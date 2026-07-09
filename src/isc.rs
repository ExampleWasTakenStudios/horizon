use tokio::sync::{
    mpsc::{self},
    oneshot,
};

use crate::{protocol::packet::DnsPacket, query_state::QueryState, systems::IesResolveCommand};

pub struct MpscChannel<T> {
    pub tx: mpsc::Sender<T>,
    pub rx: mpsc::Receiver<T>,
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
    /// Used by the IES to send a `QueryState` to the DPS.
    pub ies_to_dps: MpscChannel<QueryState>,

    /// Used by the DPS to send a `QueryState` to the SRS.
    pub dps_to_srs: MpscChannel<QueryState>,

    /// Used by the IES to receive data it should send to the querying client.
    ///
    /// QueryStates sent through this channel are expected to have a populated `response` field. QueryState that don't fulfill this expectation, are silently dropped by the IES.
    pub ies_answer: MpscChannel<QueryState>,

    /// Data sent to the IES through this channel is directly sent to the upstream resolver with the specified `SocketAddr`.
    ///
    /// `K`: The `IesCommand` <br>
    /// `V`: The owned `DnsPacket` of the upstream response
    pub ies_upstream_resolve: MpscChannel<HalfDuplexMessage<IesResolveCommand, Option<DnsPacket>>>,
}

pub fn init_msg_channels() -> InterSystemCommunicationChannels {
    let (ies_to_dps_tx, ies_to_dps_rx) = mpsc::channel(100);
    let (dps_to_srs_tx, dps_to_srs_rx) = mpsc::channel(100);
    let (ies_answer_channel_tx, ies_answer_channel_rx) = mpsc::channel(100);
    let (ies_upstream_resolve_tx, ies_upstream_resolve_rx) = mpsc::channel(100);

    InterSystemCommunicationChannels {
        ies_to_dps: MpscChannel {
            tx: ies_to_dps_tx,
            rx: ies_to_dps_rx,
        },
        dps_to_srs: MpscChannel {
            tx: dps_to_srs_tx,
            rx: dps_to_srs_rx,
        },
        ies_answer: MpscChannel {
            tx: ies_answer_channel_tx,
            rx: ies_answer_channel_rx,
        },
        ies_upstream_resolve: MpscChannel {
            tx: ies_upstream_resolve_tx,
            rx: ies_upstream_resolve_rx,
        },
    }
}
