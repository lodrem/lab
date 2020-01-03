use std::net::Shutdown;

use bytes::BytesMut;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_util::codec::{Decoder, Encoder, Framed};

use crate::log;

pub enum Message {
    Metadata(),
    Log(log::Log),
}

pub type Tx = UnboundedSender<Message>;
pub type Rx = UnboundedReceiver<Message>;

pub struct FrameCodec {}
impl FrameCodec {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder for FrameCodec {
    type Item = Message;
    type Error = std::io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Decoder for FrameCodec {
    type Item = Message;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }
}

pub struct Peer {
    transport: Framed<TcpStream, FrameCodec>,
    tx: Tx,
    rx: Rx,
}

impl Peer {
    pub fn new(stream: TcpStream) -> Self {
        let (tx, rx) = unbounded_channel();

        let transport = Framed::new(stream, FrameCodec::new());

        Self { transport, tx, rx }
    }

    pub fn shutdown(&mut self) {
        let _ = self.transport.get_ref().shutdown(Shutdown::Both);
    }

    pub fn tx(&self) -> &Tx {
        &self.tx
    }

    pub fn take_tx(&self) -> Tx {
        self.tx.clone()
    }

    pub fn rx(&self) -> &Rx {
        &self.rx
    }
}
