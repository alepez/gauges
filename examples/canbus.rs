use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};

use gauges::core::{NamedRecord, Record, SignalId, Value};
use gauges::net::Publisher;

fn raw_to_percent(raw: u8) -> f64 {
    (raw as f64) / 250.0 * 100.0
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut socket_rx = CANSocket::open("can0")?;

    let mut publisher = Publisher::new("127.0.0.1:9999").await?;

    while let Some(Ok(frame)) = socket_rx.next().await {
        let data = frame.data();
        let id = frame.id();
        let value: Option<f64> = match id {
            0x0CFF08D0 => {
                // Current position
                let pos = raw_to_percent(data[0]);
                Some(pos)
            }
            0x0CFF0181 => {
                // Requested position
                let pos = raw_to_percent(data[1]);
                Some(pos)
            }
            _ => {
                None
            }
        };

        if let Some(value) = value {
            let record = NamedRecord {
                id: SignalId::Num(id),
                record: Record {
                    value: Value::Percent(value),
                },
            };
            if publisher.publish(record).await.is_err() {
                break;
            }
        }
    }
    Ok(())
}
