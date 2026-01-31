use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub id: i32,
    pub email: String,
    pub latest_session: Session,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub email: String,
    pub created_at: String,
}
