mod commands;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            load_clinic_data,
            save_clinic_data,
            get_data_path,
            create_backup,
            list_backups,
            restore_backup,
            open_data_folder,
            check_for_updates
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
