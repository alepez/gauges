use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

pub fn launch_server(sender: UnboundedSender<f64>) {
    tokio::spawn(async move {
        let bind = SocketAddr::from_str("127.0.0.1:9999").unwrap();
        let listener = TcpListener::bind(bind).await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            handle_incoming_data(socket, sender.clone()).await;
        }
    });
}

pub async fn handle_incoming_data(socket: TcpStream, sender: UnboundedSender<f64>) {
    let mut server = Framed::new(socket, LinesCodec::new_with_max_length(1024));
    while let Some(Ok(line)) = server.next().await {
        let parsed: Option<f64> = line.parse().ok();
        if let Some(x) = parsed {
            sender.send(x).unwrap();
        }
    }
}
