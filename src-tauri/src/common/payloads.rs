use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PayloadNewDay {
    pub day: i32,
}

#[derive(Clone, Serialize)]
pub struct NewGame {
    pub population: i32,
}