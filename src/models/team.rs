use crate::models::money::MonetaryValue;

pub struct Team {
    pub name: String,
    pub logo: String,
    pub squad_size: Option<u8>,
    pub delta_age: Option<f32>,
    pub foreigners: Option<u8>,
    pub delta_value: Option<MonetaryValue>,
    pub market_value: Option<MonetaryValue>
}