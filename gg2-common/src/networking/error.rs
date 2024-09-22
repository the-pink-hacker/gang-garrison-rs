pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
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
    #[error("An error occured when trying to connect: {0}")]
    Connection(std::io::Error),
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
}
