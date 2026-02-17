use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub kind: String,
    pub url: String,
    pub keyword: Option<String>,
    pub interval_seconds: i32,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorResult {
    pub id: String,
    pub monitor_id: String,
    pub region: String,
    pub status: String,
    pub response_time_ms: Option<i32>,
    pub http_status: Option<i32>,
    pub ssl_valid: Option<bool>,
    pub ssl_expires_at: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMonitor {
    pub name: String,
    pub kind: String,
    pub url: String,
    pub keyword: Option<String>,
    pub interval_seconds: Option<i32>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMonitor {
    pub name: Option<String>,
    pub kind: Option<String>,
    pub url: Option<String>,
    pub keyword: Option<String>,
    pub interval_seconds: Option<i32>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMonitorResult {
    pub region: String,
    pub status: String,
    pub response_time_ms: Option<i32>,
    pub http_status: Option<i32>,
    pub ssl_valid: Option<bool>,
    pub ssl_expires_at: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListMonitorResultsQuery {
    pub region: Option<String>,
    pub status: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CheckMonitorRequest {
    pub region: Option<String>,
}
