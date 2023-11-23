use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::BackgroundTasks;
#[cfg(mobile)]
use mobile::BackgroundTasks;

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
        .setup(|app, api| {
            #[cfg(mobile)]
            let background_tasks = mobile::init(app, api)?;
            #[cfg(desktop)]
            let background_tasks = desktop::init(app, api)?;
            app.manage(background_tasks);

            Ok(())
        })
        .build()
}
