use serde::{Deserialize, Serialize};

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Serialize)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct Robot {
    pub serial: String,
    pub name: String,
    pub organization: String,
    pub robot_type: String
}

#[derive(Deserialize)]
pub struct UpdateRobot {
    pub name: Option<String>,
    pub organization: Option<String>,
    pub robot_type: Option<String>,
}