use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::BackgroundTasks;
#[cfg(mobile)]
use mobile::BackgroundTasks;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the background-tasks APIs.
pub trait BackgroundTasksExt<R: Runtime> {
    fn background_tasks(&self) -> &BackgroundTasks<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BackgroundTasksExt<R> for T {
    fn background_tasks(&self) -> &BackgroundTasks<R> {
        self.state::<BackgroundTasks<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("background-tasks")
        .invoke_handler(tauri::generate_handler![commands::execute])
        .setup(|app, api| {
            #[cfg(mobile)]
            let background_tasks = mobile::init(app, api)?;
            #[cfg(desktop)]
            let background_tasks = desktop::init(app, api)?;
            app.manage(background_tasks);

            // manage state so it is accessible by the commands
            // app.manage(MyState::default());
            Ok(())
        })
        .build()
}
