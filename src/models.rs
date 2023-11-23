use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleBackgroundTaskRequest {
    pub label: String,
    pub interval: u64,
}
