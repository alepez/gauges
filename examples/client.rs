use std::collections::HashMap;
use std::time::Duration;

use tokio::time::sleep;

use gauges::core::{NamedRecord, Record, SignalId, Value};
use gauges::net::Publisher;

struct RecordsGenerator {
    x: f64,
    step: f64,
    min: f64,
    max: f64,
}

impl RecordsGenerator {
    fn next(&mut self) -> Record {
        if self.x > self.max || self.x < self.min {
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
    let mut generators: HashMap<u32, RecordsGenerator> = HashMap::new();
    generators.insert(
        0,
        RecordsGenerator {
            x: 0.0,
            step: 1.0,
            min: -20.0,
            max: 40.0,
        },
    );
    generators.insert(
        1,
        RecordsGenerator {
            x: 9.0,
            step: 5.0,
            min: 0.0,
            max: 100.0,
        },
    );
    generators.insert(
        2,
        RecordsGenerator {
            x: -90.0,
            step: 5.0,
            min: -90.0,
            max: 360.0,
        },
    );

    let mut err = false;

    let mut publisher = Publisher::new("127.0.0.1:9999").await?;

    while !err {
        for (&id, generator) in &mut generators {
            let record = generator.next();
            let record = NamedRecord {
                id: SignalId::Num(id),
                record,
            };

            if publisher.publish(record).await.is_err() {
                err = true;
                break;
            }
        }
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
