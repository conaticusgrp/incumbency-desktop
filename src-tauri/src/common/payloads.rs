use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PayloadNewDay {
    pub day: i32,
}