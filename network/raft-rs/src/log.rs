use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::node::Node;

pub enum Log {
    NodeAppend(Node),
    NodeRemove(Node),
}

pub type Tx = UnboundedSender<Log>;
pub type Rx = UnboundedReceiver<Log>;

pub fn channel() -> (Tx, Rx) {
    unbounded_channel()
}
