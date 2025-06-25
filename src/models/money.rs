#[derive(Debug)]
pub enum Unit {
    K,
    M,
    Bn,
    None
}

#[derive(Debug)]
pub struct MonetaryValue {
    pub amount: f32,
    pub unit: Unit
}