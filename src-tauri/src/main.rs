// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use parser::{md_to_html, FilePathTx};
use tokio::sync::{mpsc, Mutex};

mod cmd;
mod parser;

fn main() {
    let (tx, mut rx) = mpsc::channel(1);
    tauri::Builder::default()
        .manage(FilePathTx {
            inner: Mutex::new(tx),
        })
        .invoke_handler(tauri::generate_handler![cmd::parse_md])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(path) = rx.recv().await {
                        let html = md_to_html(&path).await;
                        match html {
                            Ok(html) => {
                                cmd::send_to_js("md_parsed", html, &app_handle);
                            }
                            Err(e) => {
                                cmd::send_to_js("md_parsed", e, &app_handle);
                            }
                        }
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
