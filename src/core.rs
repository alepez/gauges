#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    None,
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub value: Value,
}
