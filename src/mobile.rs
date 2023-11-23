use serde::{de::DeserializeOwned, Serialize};
use tauri::{
    ipc::{Channel, InvokeBody},
    plugin::{PluginApi, PluginHandle},
    AppHandle, Manager, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "studio.darksoil.tauripluginbackgroundtasks";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_background - tasks);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventHandler {
    pub handler: Channel,
}

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    app_handle: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<BackgroundTasks<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "BackgroundTasksPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_background - tasks)?;

    let app_handle = app_handle.clone();

    handle.run_mobile_plugin::<()>(
        "setupChannel",
        EventHandler {
            handler: Channel::new(move |event| {
                let label = match event {
                    InvokeBody::Json(payload) => payload
                        .get("label")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_owned()),
                    _ => None,
                };

                let _r = app_handle.emit("run-background-task", label.expect("Label is empty"));
                // let _ = app_handle.emit_all("deep-link://new-url", payload);
                Ok(())
            }),
        },
    )?;

    Ok(BackgroundTasks(handle))
}

/// Access to the background-tasks APIs.
pub struct BackgroundTasks<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> BackgroundTasks<R> {
    pub fn schedule_background_task<F>(
        &self,
        payload: ScheduleBackgroundTaskRequest,
        task: F,
    ) -> crate::Result<()>
    where
        F: Fn() + Send + 'static,
    {
        let cloned_label = payload.label.clone();
        self.0
            .app()
            .listen_global("run-background-task", move |event| {
                if event.payload().to_string() == format!(r#""{cloned_label}""#) {
                    task();
                }
            });

        self.0
            .run_mobile_plugin("scheduleBackgroundTask", payload)
            .map_err(Into::into)
    }
}
