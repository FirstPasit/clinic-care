// Tauri integration module
// This module provides functions to call Tauri backend when running as desktop app,
// or fallback to localStorage when running in browser

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Check if running inside Tauri
pub fn is_tauri() -> bool {
    if let Some(win) = window() {
        win.get("__TAURI__").is_some()
    } else {
        false
    }
}

/// Load clinic data - from file (Tauri) or localStorage (browser)
pub async fn load_data() -> Result<String, String> {
    if is_tauri() {
        match invoke("load_clinic_data", JsValue::NULL).await.as_string() {
            Some(data) => Ok(data),
            None => Err("Failed to load data from Tauri".to_string())
        }
    } else {
        // Fallback to localStorage
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            Ok(storage.get_item("clinic_all_data").ok().flatten().unwrap_or_default())
        } else {
            Err("localStorage not available".to_string())
        }
    }
}

/// Save clinic data - to file (Tauri) or localStorage (browser)  
pub async fn save_data(data: &str) -> Result<(), String> {
    if is_tauri() {
        let args = js_sys::Object::new();
        js_sys::Reflect::set(&args, &"data".into(), &data.into()).unwrap();
        invoke("save_clinic_data", args.into()).await;
        Ok(())
    } else {
        // Fallback to localStorage
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            storage.set_item("clinic_all_data", data).map_err(|_| "Failed to save".to_string())
        } else {
            Err("localStorage not available".to_string())
        }
    }
}

/// Get the data file path (Tauri only)
pub async fn get_data_path() -> Option<String> {
    if is_tauri() {
        invoke("get_data_path", JsValue::NULL).await.as_string()
    } else {
        None
    }
}

/// Create a backup (Tauri only)
pub async fn create_backup() -> Result<String, String> {
    if is_tauri() {
        match invoke("create_backup", JsValue::NULL).await.as_string() {
            Some(path) => Ok(path),
            None => Err("Failed to create backup".to_string())
        }
    } else {
        Err("Backup only available in desktop app".to_string())
    }
}

/// Open data folder (Tauri only)
pub async fn open_data_folder() -> Result<(), String> {
    if is_tauri() {
        invoke("open_data_folder", JsValue::NULL).await;
        Ok(())
    } else {
        Err("Only available in desktop app".to_string())
    }
}
