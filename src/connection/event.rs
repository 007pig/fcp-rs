use std::io;
use ::message::Message;

/// Event of command which is sent TO reader thread
pub enum EventCmd {
    /// Close connection and terminate the thread
    Close,
}

/// Event of result which is sent FROM reader thread
pub enum EventResult {
    /// Connection was manually closed. The string is the reason.
    Closed(&'static str),
    /// Connection has dropped.
    Disconnected,
    /// Message from the server.
    Message(Box<Message>),
    /// Error parsing a message from the server.
    ///
    /// This can probably be ignored, and it shouldn't ever happen, really.
    /// If you catch this you should probably open an issue on GitHub.
    //ParseError(ParseError),
    /// Connection was sucessfully restored.
    Reconnected,
    /// Attempting to restore connection.
    Reconnecting,
    /// An error occured trying to restore the connection.
    ///
    /// This is normal in poor network conditions. It might take
    /// a few attempts before the connection can be restored.
    ReconnectionError(io::Error),
}
