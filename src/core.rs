use std::collections::HashMap;
use std::fmt::Display;

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

impl Display for SignalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalId::Num(n) => write!(f, "{}", n),
        }
    }
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

#[derive(Debug, Default, PartialEq, Clone)]
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

    pub fn insert(&mut self, id: SignalId, info: SignalInfo) {
        self.items.insert(
            id,
            Signal {
                id,
                info,
                current_record: None,
            },
        );
    }

    pub fn get(&self, id: &SignalId) -> Option<&Signal> {
        self.items.get(id)
    }

    pub fn iter(&self) -> std::collections::hash_map::Values<'_, SignalId, Signal> {
        self.items.values()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GaugeInfo {
    pub id: SignalId,
    pub style: GaugeStyle,
    pub range: Range,
    pub signal: SignalInfo,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CircleGaugeStyle {
    pub radius: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArcGaugeStyle {
    pub radius: f64,
    pub begin_angle: f64,
    pub full_width: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GaugeStyle {
    Arc(ArcGaugeStyle),
    Circle(CircleGaugeStyle),
}

#[derive(PartialEq, Clone)]
pub struct DashboardConfig {
    pub items: Vec<GaugeInfo>,
}

impl DashboardConfig {
    pub fn new(items: Vec<GaugeInfo>) -> Self {
        DashboardConfig { items }
    }
}

impl Into<Signals> for DashboardConfig {
    fn into(self) -> Signals {
        let mut signals = Signals::default();
        for item in self.items.into_iter() {
            signals.insert(
                item.id,
                SignalInfo {
                    name: Some(item.id.to_string()),
                },
            );
        }
        signals
    }
}
