use tauri_plugin_background_tasks::{BackgroundTasksExt, ScheduleBackgroundTaskRequest};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_background_tasks::init())
        .setup(|app| {
            #[cfg(mobile)]
            let r =
                app.background_tasks()
                    .schedule_background_task(ScheduleBackgroundTaskRequest {
                        label: String::from("hi"),
                        interval: 1,
                    })?;
            #[cfg(mobile)]
            println!("{r:?}");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
