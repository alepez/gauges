use gauges::core::{Id, NamedRecord, Record, Value};
use std::cell::Cell;
use std::collections::HashMap;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::sleep;

struct RecordsGenerator {
    x: f64,
    step: f64,
    min: f64,
    max: f64,
}

impl RecordsGenerator {
    fn next(&mut self) -> Record {
        if self.x >= self.max || self.x <= self.min {
            self.step = -self.step;
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

    let mut generators: HashMap<u32, Cell<RecordsGenerator>> = HashMap::new();
    generators.insert(
        1,
        Cell::new(RecordsGenerator {
            x: 0.0,
            step: 1.0,
            min: 0.0,
            max: 100.0,
        }),
    );

    let mut err = false;

    while !err {
        for (&id, generator) in &mut generators {
            let record = generator.get_mut().next();
            let record = NamedRecord {
                id: Id::Num(id),
                record,
            };
            let mut serialized = serde_json::to_vec(&record).unwrap();
            serialized.push(b'\n');
            if stream.write(&serialized).await.is_err() {
                err = true;
                break;
            }
        }
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
