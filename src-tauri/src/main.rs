#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod notifications;

use crate::notifications::NotificationClient;
use notifications::github;
use serde::Deserialize;

fn main() {
    let github = github::Client::new("");
    let clients = CodeHosts { github };

    tauri::Builder::default()
        .manage(clients)
        .invoke_handler(tauri::generate_handler![get_notifications, handle])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Retrieves notifications from the specified code host based on the provided options.
#[tauri::command]
async fn get_notifications(
    code_hosts: tauri::State<'_, CodeHosts>,
    code_host: CodeHost,
    opts: Option<notifications::GetOptions>,
) -> Result<Vec<notifications::Notification>, String> {
    match code_host {
        CodeHost::GitHub => code_hosts.github.get(opts).await,
    }
}

/// Handles a notification based on the provided event.
#[tauri::command]
async fn handle(
    code_hosts: tauri::State<'_, CodeHosts>,
    code_host: CodeHost,
    event: notifications::Event,
) -> Result<String, String> {
    Err("foo".to_string())
}

/// Code host clients that can be interfaced with.
struct CodeHosts {
    github: github::Client,
}

/// Enumerated list of supported code hosts.
#[derive(Deserialize)]
enum CodeHost {
    GitHub,
}
