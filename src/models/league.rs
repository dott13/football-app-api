use serde::Serialize;

#[derive(Serialize)]
pub struct League {
    pub name: String,
    pub tm_id: String,
    pub slug: String
}