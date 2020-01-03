use tokio::net::TcpListener;

use crate::log;
use crate::node::Node;
use crate::peer::Peer;

pub struct Cluster {
    listener: TcpListener,
    nodes: Vec<Node>,
}

impl Cluster {
    pub async fn connect(local_addr: &str, remote_addrs: &[&str]) -> Self {
        let listener = TcpListener::bind(local_addr)
            .await
            .expect("listen local address");

        let (log_tx, log_rx) = log::channel();

        let nodes = remote_addrs
            .into_iter()
            .map(|addr| Node::connect(addr, log_tx.clone()))
            .collect();

        Self { listener, nodes }
    }

    pub async fn serve(&mut self) {
        while let Ok((stream, _addr)) = self.listener.accept().await {
            let peer = Peer::new(stream);
        }
    }

    pub fn quorum(&self) -> usize {
        self.nodes.len() / 2 + 1
    }

    pub async fn is_majority(&self) -> bool {
        let mut connected_num = 0;
        for node in self.nodes.iter() {
            if node.is_connected().await {
                connected_num += 1;
            }
        }

        connected_num >= self.quorum()
    }
}
