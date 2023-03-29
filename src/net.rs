use crate::core::NamedRecord;

use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

pub async fn launch_server(sender: Sender, addr: String) {
    let bind = SocketAddr::from_str(&addr).unwrap();
    let listener = TcpListener::bind(bind).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        handle_incoming_data(socket, sender.clone()).await;
    }
}

async fn handle_incoming_data(socket: TcpStream, sender: Sender) {
    let mut server = Framed::new(socket, LinesCodec::new_with_max_length(1024));
    while let Some(Ok(line)) = server.next().await {
        let record: Option<NamedRecord> = serde_json::from_str(&line).ok();
        if let Some(record) = record {
            sender.send(record).unwrap();
        }
    }
}

#[derive(Clone)]
pub struct Sender(UnboundedSender<NamedRecord>);

pub struct Receiver(UnboundedReceiver<NamedRecord>);

impl Receiver {
    pub async fn recv(&mut self) -> Option<NamedRecord> {
        self.0.recv().await
    }
}

impl Sender {
    pub fn send(
        &self,
        message: NamedRecord,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<NamedRecord>> {
        self.0.send(message)
    }
}

pub fn channel() -> (Sender, Receiver) {
    let (sender, receiver) = unbounded_channel::<NamedRecord>();
    (Sender(sender), Receiver(receiver))
}

pub struct Publisher {
    stream: TcpStream,
}

impl Publisher {
    pub async fn new<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self { stream })
    }

    pub async fn publish(&mut self, record: NamedRecord) -> std::io::Result<usize> {
        let mut serialized = serde_json::to_vec(&record).unwrap();
        serialized.push(b'\n');
        self.stream.write(&serialized).await
    }
}
