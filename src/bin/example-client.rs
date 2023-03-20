use gauges::core::{Record, Value};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::sleep;

struct RecordsGenerator {
    x: f64,
    step: f64,
}

impl RecordsGenerator {
    fn next(&mut self) -> Record {
        if self.x >= 100.0 {
            self.step = -1.0;
        }
        if self.x <= 0.0 {
            self.step = 1.0;
        }
        self.x += self.step;

        Record {
            value: Value::Float(self.x),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:9999").await?;
    let mut generator = RecordsGenerator { x: 0.0, step: 1.0 };
    loop {
        let x = generator.next();
        let mut serialized = serde_json::to_vec(&x).unwrap();
        serialized.push(b'\n');
        if stream.write(&serialized).await.is_err() {
            break;
        }
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
