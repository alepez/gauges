use std::f64::consts::PI;

use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};

use gauges::core::{NamedRecord, Record, SignalId, Value};
use gauges::net::Publisher;

fn decode_u16_with_offset(data: &[u8], offset: usize) -> f64 {
    let x = data[offset] as u32;
    let y = data[offset + 1] as u32;
    let v = (y << 8) | x;
    v as f64
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut socket_rx = CANSocket::open("can0")?;

    let mut publisher = Publisher::new("127.0.0.1:9999").await?;

    while let Some(Ok(frame)) = socket_rx.next().await {
        let data = frame.data();
        let id = frame.id();

        let records = match id {
            0x09FD0217 => {
                // PGN=130306 SRC=23 PRIO=2 (Wind Data)
                let speed = decode_u16_with_offset(data, 1) * (1.94384 / 100.0);
                let angle = decode_u16_with_offset(data, 3) * (180.0 / PI / 10000.0);
                let angle = if angle > 180.0 { angle - 360.0 } else { angle };
                let is_apparent = data[5] & 0x07 == 0x02;
                if is_apparent {
                    vec![(1, Value::Float(speed)), (2, Value::Float(angle))]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        };

        for (id, value) in records {
            let record = NamedRecord {
                id: SignalId::Num(id),
                record: Record { value },
            };
            if publisher.publish(record).await.is_err() {
                break;
            }
        }
    }
    Ok(())
}
