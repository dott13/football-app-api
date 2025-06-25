use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Unit {
    K,
    M,
    Bn,
    None
}

#[derive(Debug, Serialize)]
pub struct MonetaryValue {
    pub amount: f32,
    pub unit: Unit
}