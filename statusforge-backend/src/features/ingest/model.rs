use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub id: String,
    pub project_id: String,
    pub level: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
    pub trace_id: Option<String>,
    pub source: Option<String>,
    pub environment: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLog {
    pub level: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
    pub trace_id: Option<String>,
    pub source: Option<String>,
    pub environment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListLogsQuery {
    pub level: Option<String>,
    pub trace_id: Option<String>,
    pub source: Option<String>,
    pub environment: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
