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
    pub id: SignalId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum SignalId {
    Num(u32),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignalInfo {
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Signal {
    pub id: SignalId,
    pub info: SignalInfo,
    pub current_record: Option<Record>,
}

#[derive(Default, PartialEq, Clone)]
pub struct Signals {
    items: HashMap<SignalId, Signal>,
}

impl Signals {
    pub fn insert_named_record(&mut self, record: NamedRecord) {
        let NamedRecord { id, record } = record;
        if let Some(signal) = self.items.get_mut(&id) {
            signal.current_record = Some(record);
        }
    }

    pub fn get(&self, id: &SignalId) -> Option<&Signal> {
        self.items.get(id)
    }

    pub fn iter(&self) -> std::collections::hash_map::Values<'_, SignalId, Signal> {
        self.items.values()
    }
}
