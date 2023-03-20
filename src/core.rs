use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Value {
    None,
    Float(f64),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Record {
    pub value: Value,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NamedRecord {
    pub record: Record,
    pub id: Id,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Id {
    Num(u32),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Signal {
    pub id: Id,
    pub current_record: Option<Record>,
}

#[derive(Default)]
pub struct Signals {
    items: HashMap<Id, Signal>,
}

impl Signals {
    pub fn insert_named_record(&mut self, record: NamedRecord) {
        let NamedRecord { id, record } = record;
        self.items
            .entry(id)
            .and_modify(|signal| signal.current_record = Some(record))
            .or_insert(Signal {
                id,
                current_record: None,
            });
    }

    pub fn get(&self, id: &Id) -> Option<&Signal> {
        self.items.get(id)
    }
}
