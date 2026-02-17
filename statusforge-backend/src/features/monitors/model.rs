use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub id: String,
    pub project_id: String,
    pub kind: String,
}
