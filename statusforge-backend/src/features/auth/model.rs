use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub access_token: String,
    pub user_id: String,
}
