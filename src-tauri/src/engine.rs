use std::{thread, time::{Duration, Instant}};

use crate::app_state::AppState;
use crate::jiggle::jiggle_cursor;
use crate::models::{AppStatus, AwakeMode};

pub fn start_engine(state: &AppState) -> Result<(), String> {
    if state
        .is_running
        .load(std::sync::atomic::Ordering::SeqCst)
    {
        return Err("Awake engine is already running".into());
    }

    state
        .is_running
        .store(true, std::sync::atomic::Ordering::SeqCst);

    {
        let settings = state
            .settings
            .lock()
            .map_err(|_| "Failed to lock settings".to_string())?
            .clone();

        let mut status = state
            .status
            .lock()
            .map_err(|_| "Failed to lock status".to_string())?;

        status.is_running = true;
        status.active_mode = settings.mode;
    }

    let running = state.is_running.clone();
    let settings = state.settings.clone();
    let status = state.status.clone();
    let next_movement = state.next_movement_at.clone();

    thread::spawn(move || {
        while running.load(std::sync::atomic::Ordering::SeqCst) {
            let current_settings = match settings.lock() {
                Ok(guard) => guard.clone(),
                Err(_) => break,
            };

            let next_time = Instant::now() + Duration::from_secs(current_settings.interval_seconds);

            if let Ok(mut next) = next_movement.lock() {
                *next = Some(next_time);
            }

            thread::sleep(Duration::from_secs(current_settings.interval_seconds));

            if !running.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }

            match current_settings.mode {
                AwakeMode::JiggleCursor | AwakeMode::Smart => {
                    if let Err(error) = jiggle_cursor(
                        current_settings.jiggle_pixels,
                        current_settings.movement_duration_ms,
                        current_settings.movement_pattern,
                    ) {
                        eprintln!("[Aurora Awake] jiggle error: {error}");
                    } else {
                        println!(
                            "[Aurora Awake] jiggle tick -> interval: {}s, pixels: {}, duration_ms: {}, pattern: {:?}",
                            current_settings.interval_seconds,
                            current_settings.jiggle_pixels,
                            current_settings.movement_duration_ms,
                            current_settings.movement_pattern
                        );
                    }
                }
                AwakeMode::PreventSleep => {
                    println!("[Aurora Awake] PreventSleep mode not implemented yet");
                }
            }
        }

        if let Ok(mut s) = status.lock() {
            s.is_running = false;
        }

        if let Ok(mut next) = next_movement.lock() {
            *next = None;
        }

        println!("[Aurora Awake] engine stopped");
    });

    Ok(())
}

pub fn stop_engine(state: &AppState) {
    state.stop();
}

pub fn get_runtime_status(state: &AppState) -> Result<AppStatus, String> {
    let status = state
        .status
        .lock()
        .map_err(|_| "Failed to lock status".to_string())?
        .clone();

    Ok(status)
}