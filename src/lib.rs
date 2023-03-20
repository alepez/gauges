pub mod app;
pub mod core;
pub mod net;

use crate::core::SignalInfo;
use crate::core::SignalId;

#[derive(PartialEq, Clone)]
pub struct GaugeInfo {
    pub id: SignalId,
    pub style: GaugeStyle,
    pub range: Range,
    pub signal: SignalInfo,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Range {
    pub min: i64,
    pub max: i64,
}

#[derive(PartialEq, Clone)]
pub enum GaugeStyle {
    Circle,
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
