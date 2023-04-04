use std::collections::HashMap;
use std::fmt::Display;
use std::time::Duration;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Value {
    None,
    Float(f64),
    Percent(f64),
    OnOff(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Float(x) => write!(f, "{:8.3}", x),
            Value::Percent(x) => write!(f, "{:8.1}%", x),
            Value::None => write!(f, "-"),
            Value::OnOff(false) => write!(f, "Off"),
            Value::OnOff(true) => write!(f, "On"),
        }
    }
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

#[derive(Debug, PartialEq, Clone)]
pub struct ExtendedSignal {
    signal: Signal,
    instant: std::time::Instant,
}

impl ExtendedSignal {
    pub fn signal(&self) -> &Signal {
        &self.signal
    }
    pub fn age(&self) -> Duration {
        std::time::Instant::now() - self.instant
    }
}

impl From<Signal> for ExtendedSignal {
    fn from(signal: Signal) -> Self {
        let instant = std::time::Instant::now();
        ExtendedSignal { signal, instant }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Signals {
    items: HashMap<SignalId, ExtendedSignal>,
}

impl Signals {
    pub fn insert_named_record(&mut self, record: NamedRecord) {
        let NamedRecord { id, record } = record;
        if let Some(ext) = self.items.get_mut(&id) {
            ext.signal.current_record = Some(record);
            ext.instant = std::time::Instant::now();
        }
    }

    pub fn insert(&mut self, id: SignalId, info: SignalInfo) {
        self.items.insert(
            id,
            Signal {
                id,
                info,
                current_record: None,
            }
            .into(),
        );
    }

    pub fn get(&self, id: &SignalId) -> Option<&ExtendedSignal> {
        self.items.get(id)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct GaugeTextFormat {
    #[serde(default)]
    pub precision: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GaugeInfo {
    pub id: SignalId,
    pub style: GaugeStyle,
    pub range: Range,
    pub signal: SignalInfo,
    pub format: GaugeTextFormat,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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
    pub begin_angle: angle::Rad<f64>,
    pub full_width: angle::Rad<f64>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ProtractorGaugeStyle {
    pub radius: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GaugeStyle {
    Arc(ArcGaugeStyle),
    Circle(CircleGaugeStyle),
    Protractor(ProtractorGaugeStyle),
}

#[derive(PartialEq, Clone)]
pub struct DashboardConfig {
    pub addr: String,
    pub items: Vec<GaugeInfo>,
    pub age_indicator: bool,
}

impl From<DashboardConfig> for Signals {
    fn from(value: DashboardConfig) -> Self {
        let mut signals = Signals::default();
        for item in value.items.into_iter() {
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Age {
    New,
    Valid,
    Expired,
    Unknown,
}
