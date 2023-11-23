use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<BackgroundTasks<R>> {
  Ok(BackgroundTasks(app.clone()))
}

/// Access to the background-tasks APIs.
pub struct BackgroundTasks<R: Runtime>(AppHandle<R>);

impl<R: Runtime> BackgroundTasks<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
