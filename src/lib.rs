pub mod app;
pub mod core;
pub mod net;

use crate::core::SignalId;
use crate::core::SignalInfo;
use crate::core::Signals;

#[derive(Debug, PartialEq, Clone)]
pub struct GaugeInfo {
    pub id: SignalId,
    pub style: GaugeStyle,
    pub range: Range,
    pub signal: SignalInfo,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Range {
    pub min: i64,
    pub max: i64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CircleGaugeStyle {
    pub radius: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GaugeStyle {
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
