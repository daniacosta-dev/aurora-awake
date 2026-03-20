use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Instant;

use crate::models::{AppSettings, AppStatus};

pub struct AppState {
    pub is_running: Arc<AtomicBool>,
    pub settings: Arc<Mutex<AppSettings>>,
    pub status: Arc<Mutex<AppStatus>>,
    pub next_movement_at: Arc<Mutex<Option<Instant>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            settings: Arc::new(Mutex::new(AppSettings::default())),
            status: Arc::new(Mutex::new(AppStatus::default())),
            next_movement_at: Arc::new(Mutex::new(None)),
        }
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);

        if let Ok(mut status) = self.status.lock() {
            status.is_running = false;
        }

        if let Ok(mut next) = self.next_movement_at.lock() {
            *next = None;
        }
    }
}