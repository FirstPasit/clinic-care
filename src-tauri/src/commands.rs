use std::fs;
use std::path::PathBuf;

// Data file path helper
fn get_data_dir() -> PathBuf {
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ClinicCare");
    
    // Create directory if it doesn't exist
    if !data_dir.exists() {
        let _ = fs::create_dir_all(&data_dir);
    }
    
    data_dir
}

fn get_data_file_path() -> PathBuf {
    get_data_dir().join("clinic_data.json")
}

// Backup directory
fn get_backup_dir() -> PathBuf {
    let backup_dir = get_data_dir().join("backups");
    if !backup_dir.exists() {
        let _ = fs::create_dir_all(&backup_dir);
    }
    backup_dir
}

// ============ Tauri Commands ============

/// Load all clinic data from file
#[tauri::command]
pub fn load_clinic_data() -> Result<String, String> {
    let file_path = get_data_file_path();
    
    if file_path.exists() {
        fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read data file: {}", e))
    } else {
        // Return empty object if file doesn't exist yet
        Ok("{}".to_string())
    }
}

/// Save all clinic data to file
#[tauri::command]
pub fn save_clinic_data(data: String) -> Result<(), String> {
    let file_path = get_data_file_path();
    
    fs::write(&file_path, &data)
        .map_err(|e| format!("Failed to save data file: {}", e))?;
    
    log::info!("Data saved to {:?}", file_path);
    Ok(())
}

/// Get the data file path for display
#[tauri::command]
pub fn get_data_path() -> String {
    get_data_file_path().to_string_lossy().to_string()
}

/// Create a backup with timestamp
#[tauri::command]
pub fn create_backup() -> Result<String, String> {
    let source = get_data_file_path();
    
    if !source.exists() {
        return Err("No data file to backup".to_string());
    }
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = get_backup_dir().join(format!("backup_{}.json", timestamp));
    
    fs::copy(&source, &backup_path)
        .map_err(|e| format!("Failed to create backup: {}", e))?;
    
    Ok(backup_path.to_string_lossy().to_string())
}

/// List available backups
#[tauri::command]
pub fn list_backups() -> Result<Vec<String>, String> {
    let backup_dir = get_backup_dir();
    
    let mut backups: Vec<String> = fs::read_dir(&backup_dir)
        .map_err(|e| format!("Failed to read backup directory: {}", e))?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if name.ends_with(".json") {
                    Some(name)
                } else {
                    None
                }
            })
        })
        .collect();
    
    backups.sort();
    backups.reverse(); // Most recent first
    
    Ok(backups)
}

/// Restore from a backup file
#[tauri::command]
pub fn restore_backup(backup_name: String) -> Result<String, String> {
    let backup_path = get_backup_dir().join(&backup_name);
    
    if !backup_path.exists() {
        return Err("Backup file not found".to_string());
    }
    
    let data = fs::read_to_string(&backup_path)
        .map_err(|e| format!("Failed to read backup: {}", e))?;
    
    // Save as current data
    save_clinic_data(data.clone())?;
    
    Ok(data)
}

/// Open data folder in file explorer
#[tauri::command]
pub fn open_data_folder() -> Result<(), String> {
    let data_dir = get_data_dir();
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    
    Ok(())
}
