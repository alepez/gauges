use std::collections::HashMap;
use std::time::Duration;

use tokio::time::sleep;

use gauges::core::{NamedRecord, Record, SignalId, Value};
use gauges::net::Publisher;

trait RecordGenerator {
    fn next(&mut self) -> Record;
}

struct OnOffGenerator {
    t: std::time::Instant,
    on_ratio: f64,
    period: std::time::Duration,
}

impl RecordGenerator for OnOffGenerator {
    fn next(&mut self) -> Record {
        let time_since_start = std::time::Instant::now() - self.t;
        let x = time_since_start.as_secs_f64() % self.period.as_secs_f64();
        let y = (x as f64) / (self.period.as_secs_f64());
        let on = y < self.on_ratio;
        Record {
            value: Value::OnOff(on),
        }
    }
}

impl Into<Generator> for OnOffGenerator {
    fn into(self) -> Generator {
        Generator::OnOff(self)
    }
}

struct FloatGenerator {
    x: f64,
    step: f64,
    min: f64,
    max: f64,
}

impl RecordGenerator for FloatGenerator {
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

impl Into<Generator> for FloatGenerator {
    fn into(self) -> Generator {
        Generator::Float(self)
    }
}

enum Generator {
    Float(FloatGenerator),
    OnOff(OnOffGenerator),
}

impl RecordGenerator for Generator {
    fn next(&mut self) -> Record {
        match self {
            Generator::Float(x) => x.next(),
            Generator::OnOff(x) => x.next(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut generators: HashMap<u32, Generator> = HashMap::new();
    generators.insert(
        0,
        FloatGenerator {
            x: 0.0,
            step: 1.0,
            min: -20.0,
            max: 40.0,
        }
        .into(),
    );
    generators.insert(
        1,
        FloatGenerator {
            x: 9.0,
            step: 5.0,
            min: 0.0,
            max: 100.0,
        }
        .into(),
    );
    generators.insert(
        2,
        FloatGenerator {
            x: -90.0,
            step: 5.0,
            min: -90.0,
            max: 360.0,
        }
        .into(),
    );
    generators.insert(
        3,
        OnOffGenerator {
            t: std::time::Instant::now(),
            on_ratio: 0.4,
            period: std::time::Duration::from_secs(2),
        }
        .into(),
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
