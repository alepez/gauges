use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

pub fn launch_server(sender: Sender) {
    tokio::spawn(async move {
        let bind = SocketAddr::from_str("127.0.0.1:9999").unwrap();
        let listener = TcpListener::bind(bind).await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            handle_incoming_data(socket, sender.clone()).await;
        }
    });
}

async fn handle_incoming_data(socket: TcpStream, sender: Sender) {
    let mut server = Framed::new(socket, LinesCodec::new_with_max_length(1024));
    while let Some(Ok(line)) = server.next().await {
        let parsed: Option<f64> = line.parse().ok();
        if let Some(x) = parsed {
            let x = Value::Float(x);
            sender.0.send(x).unwrap();
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Float(f64),
}

#[derive(Clone)]
pub struct Sender(UnboundedSender<Value>);

pub struct Receiver(UnboundedReceiver<Value>);

impl Receiver {
    pub async fn recv(&mut self) -> Option<Value> {
        self.0.recv().await
    }
}

pub fn channel() -> (Sender, Receiver) {
    let (sender, receiver) = unbounded_channel::<Value>();
    (Sender(sender), Receiver(receiver))
}
