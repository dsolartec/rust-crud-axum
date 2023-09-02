use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip)]
    pub password: String,
}
