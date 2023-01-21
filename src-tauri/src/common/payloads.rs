use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PayloadNewDay {
    pub date: String,
}

#[derive(Clone, Serialize)]
pub struct NewGame {
    pub population: i32,
}