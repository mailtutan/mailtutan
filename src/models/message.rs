use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub data: String,
}
