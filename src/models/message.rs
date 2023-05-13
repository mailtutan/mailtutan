use serde::Serialize;

#[derive(Serialize, Debug, Default, Clone)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub data: String,
}
