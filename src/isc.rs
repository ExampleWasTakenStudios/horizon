use tokio::sync::{mpsc::{self, Receiver, Sender}, oneshot};

use crate::{MAX_PACKET_SIZE, protocol::packet::DnsPacket, query_state::QueryState};

/// Represents any  one-way (simplex) channel between two systems.
pub type IscChannel<T> = (Sender<T>, Receiver<T>);

/// `T`: The type of the payload that is sent by the sender of the message. <br>
/// `V`: The type of the payload of the returning message.
pub struct HalfDuplexMessage<T, V> {
    pub payload: T,
    pub return_channel: oneshot::Sender<V>
}

/// Holds all channels used for inter-system communications.
/// The naming scheme is in rx_tx indicating which system acts as the transmitting side and which system receives the channel.
pub struct InterSystemCommunicationChannels {
    pub ies_incoming_channel: IscChannel<[u8; MAX_PACKET_SIZE]>,
    pub ies_outgoing_channel: IscChannel<[u8; MAX_PACKET_SIZE]>,
    pub ies_dps: IscChannel<QueryState>,
    pub dps_srs: IscChannel<QueryState>,
    /// Used by the SRS to send a query to the IES to be forwarded to an upstream resolver.
    ///
    /// `K`: A reference to the DNS question <br>
    /// `V`: the owned `DnsPacket` of the upstream response
    pub srs_ies_resolving_channel: IscChannel<HalfDuplexMessage<DnsPacket, DnsPacket>>,
    /// Used by the IES to receive data it should send to the querying client.
    pub ies_answer_channel: IscChannel<QueryState>,
}

pub fn init_msg_channels() -> InterSystemCommunicationChannels {
    InterSystemCommunicationChannels {
        ies_incoming_channel: mpsc::channel(100),
        ies_outgoing_channel: mpsc::channel(100),
        ies_dps: mpsc::channel(100),
        dps_srs: mpsc::channel(100),
        srs_ies_resolving_channel: mpsc::channel(100),
        ies_answer_channel: mpsc::channel(100),
    }
}

