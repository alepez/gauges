use std::time::Duration;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:9999").await?;
    let mut x = 0;
    let mut step = 1;
    loop {
        if x == 100 {
            step = -1
        }
        if x == 0 {
            step = 1
        }
        x += step;
        let s = format!("{}\n", x);
        if stream.write(&s.as_bytes()).await.is_err() {
            break;
        }
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
