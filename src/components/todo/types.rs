use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}
