// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod config;
mod engine;
mod jiggle;
mod models;

use app_state::AppState;
use tauri::Manager;
use config::{load_settings, save_settings};
use engine::{get_runtime_status, start_engine, stop_engine};
use models::{AppSettings, AppStatus};

#[tauri::command]
fn load_app_settings(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
) -> Result<AppSettings, String> {
    let settings = load_settings(&app)?;

    {
        let mut current_settings = state
            .settings
            .lock()
            .map_err(|_| "Failed to lock app settings".to_string())?;
        *current_settings = settings.clone();
    }

    Ok(settings)
}

#[tauri::command]
fn apply_app_settings(
    settings: AppSettings,
    state: tauri::State<AppState>,
) -> Result<(), String> {
    let mut current_settings = state
        .settings
        .lock()
        .map_err(|_| "Failed to lock app settings".to_string())?;

    *current_settings = settings;
    Ok(())
}

#[tauri::command]
fn save_app_settings(
    app: tauri::AppHandle,
    settings: AppSettings,
    state: tauri::State<AppState>,
) -> Result<(), String> {
    save_settings(&app, &settings)?;

    {
        let mut current_settings = state
            .settings
            .lock()
            .map_err(|_| "Failed to lock app settings".to_string())?;
        *current_settings = settings;
    }

    Ok(())
}

#[tauri::command]
fn get_default_status() -> AppStatus {
    AppStatus::default()
}

#[tauri::command]
fn start_awake(state: tauri::State<AppState>) -> Result<(), String> {
    start_engine(&state)
}

#[tauri::command]
fn stop_awake(state: tauri::State<AppState>) -> Result<(), String> {
    stop_engine(&state);
    Ok(())
}

#[tauri::command]
fn get_runtime_status_command(state: tauri::State<AppState>) -> Result<AppStatus, String> {
    get_runtime_status(&state)
}

#[tauri::command]
fn get_next_movement_seconds(
    state: tauri::State<AppState>,
) -> Result<Option<u64>, String> {
    let next = state
        .next_movement_at
        .lock()
        .map_err(|_| "Failed to lock next movement".to_string())?;

    if let Some(time) = *next {
        let now = std::time::Instant::now();

        if time > now {
            return Ok(Some((time - now).as_secs()));
        }
    }

    Ok(None)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .setup(|app| {
            if let Some(icon) = app.default_window_icon() {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_icon(icon.clone().into());
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_app_settings,
            apply_app_settings,
            save_app_settings,
            get_default_status,
            start_awake,
            stop_awake,
            get_runtime_status_command,
            get_next_movement_seconds
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}