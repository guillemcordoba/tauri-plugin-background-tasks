use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "studio.darksoil.tauripluginbackgroundtasks";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_background - tasks);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<BackgroundTasks<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "BackgroundTasksPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_background - tasks)?;
    Ok(BackgroundTasks(handle))
}

/// Access to the background-tasks APIs.
pub struct BackgroundTasks<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> BackgroundTasks<R> {
    pub fn schedule_background_task(
        &self,
        payload: ScheduleBackgroundTaskRequest,
    ) -> crate::Result<PingResponse> {
        self.0
            .run_mobile_plugin("scheduleBackgroundTask", payload)
            .map_err(Into::into)
    }
}
