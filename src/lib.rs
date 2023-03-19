pub mod app;
pub mod net;

pub struct GaugeInfo {
    pub id: GaugeId,
    pub style: GaugeStyle,
    pub range: Range,
}

pub struct Range {
    pub min: i64,
    pub max: i64,
}

pub enum GaugeStyle {
    Circle,
    Bar,
}

pub struct GaugeId(String);

impl From<String> for GaugeId {
    fn from(s: String) -> Self {
        GaugeId(s)
    }
}
