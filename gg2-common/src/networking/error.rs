pub type Result<T> = std::result::Result<T, NetworkError>;

#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
    #[error("An error occured when accepting a new connnection: {0}")]
    Accept(std::io::Error),
    #[error("Could not find connection")]
    ConnectionNotFound,
    #[error("Connection closed")]
    ChannelClosed,
    #[error("Not connected to any server")]
    NotConnected,
    #[error("An error occured when trying to start listening for new connections: {0}")]
    Listen(std::io::Error),
    #[error("An error occured when trying to connect to '{1}': {0}")]
    Connection(std::io::Error, String),
    #[error("No data was found in the packet")]
    PacketEmpty,
    #[error("Failed to parse packet kind: {0}")]
    PacketKind(u8),
    #[error("Reached end of packet")]
    UnexpectedEOF,
    #[error("Packet payload is improperly formated")]
    PacketPayload,
    #[error("Failed to serialize packet payload: {0}")]
    StringLength(std::num::TryFromIntError),
    #[error("Unsanitized string")]
    UnsanitizedString,
    #[error("{0}")]
    CommonError(#[from] crate::error::CommonError),
    #[error("Message not allowed at this time: {0:?}")]
    IncorrectMessage(crate::networking::PacketKind),
    #[error("Failed to send packet")]
    PacketSend,
    #[error("Could not initiate connection")]
    ConnectSend,
}
