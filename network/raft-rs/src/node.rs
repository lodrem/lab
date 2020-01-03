use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::log;
use crate::peer::Peer;

const MIN_DELAY_MS: u64 = 100;
const MAX_DELAY_MS: u64 = 300;

#[derive(Eq, PartialEq)]
enum Status {
    Disconnected,
    Connected,
    Terminated,
}

struct Inner {
    status: Status,
    addr: SocketAddr,
    peer: Option<Peer>,
    log_tx: log::Tx,
}

impl Inner {
    pub fn is_connected(&self) -> bool {
        self.status == Status::Connected
    }

    pub fn set_peer(&mut self, mut peer: Peer) {
        if self.status == Status::Connected {
            let _ = peer.shutdown();
        } else {
            self.peer = Some(peer);
            self.status = Status::Connected;
        }
    }

    pub fn terminate(&mut self) {
        if self.status == Status::Connected {
            self.status = Status::Disconnected;
            let _ = self.peer.as_mut().unwrap().shutdown();
        }
    }
}

#[derive(Clone)]
pub struct Node {
    inner: Arc<Mutex<Inner>>,
}

impl Node {
    pub fn connect(addr: &str, log_tx: log::Tx) -> Self {
        let inner = Arc::new(Mutex::new(Inner {
            status: Status::Disconnected,
            addr: addr.parse().unwrap(),
            peer: None,
            log_tx,
        }));

        let node = Self { inner };

        tokio::spawn(node.clone().check_connection());

        node
    }

    pub async fn is_connected(&self) -> bool {
        self.inner.lock().await.is_connected()
    }

    pub async fn terminate(&mut self) {
        let mut inner = self.inner.lock().await;
        inner.terminate();
    }

    pub async fn set_peer(&mut self, peer: Peer) {
        let mut inner = self.inner.lock().await;
        inner.set_peer(peer);
    }

    async fn check_connection(self) {
        use std::time::Duration;
        use tokio::time::delay_for;

        loop {
            let delay = {
                use rand::Rng;
                let mut rng = rand::thread_rng(); // bad performance for repeatedly consturction
                Duration::from_millis(rng.gen_range(MIN_DELAY_MS, MAX_DELAY_MS))
            };
            delay_for(delay).await;

            let mut inner = self.inner.lock().await;

            match inner.status {
                Status::Disconnected => {
                    if let Ok(stream) = TcpStream::connect(inner.addr).await {
                        let peer = Peer::new(stream);
                        inner.set_peer(peer);
                    }
                }
                Status::Connected => {
                    // TODO Send heartbeat
                }
                Status::Terminated => {
                    break;
                }
            }
        }
    }
}
