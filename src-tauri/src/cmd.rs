use serde::Serialize;
use tauri::{command, AppHandle, Manager};

use crate::parser::{FilePathTx, md_to_html};

pub fn send_to_js<R: tauri::Runtime, S>(event: &str, payload: S, manager: &AppHandle<R>)
where
    S: Serialize + Clone,
{
    manager.emit_all(event, payload).expect("failed to emit");
}

#[command]
pub async fn parse_md(path: String, state: tauri::State<'_, FilePathTx>) -> Result<(), String> {
    let tx = state.inner.lock().await;
    tx.send(path).await.map_err(|e| e.to_string())
}

#[command]
pub async fn parse_md_str(content: String) -> String {
    md_to_html(&content).await.unwrap_or_else(|e| e.to_string())
}
