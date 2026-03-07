pub mod config;
pub mod daemon;
pub mod models;
pub mod parsers;
pub mod tray;

use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use crate::models::{AppConfig, Provider, TestResult, TestRequestPayload, TestResponsePayload};
use std::sync::Arc;
use rquest::Client;
use std::collections::HashMap;

#[tauri::command]
fn get_config() -> Result<AppConfig, String> {
    crate::config::load_config()
}

#[tauri::command]
async fn test_fetch(
    provider: Provider,
    symbol: String,
    state: tauri::State<'_, Arc<crate::daemon::AppState>>,
) -> Result<TestResult, String> {
    let url = provider.request.url_template.replace("{symbol}", &symbol);
    let method = provider.request.method.to_uppercase();
    
    // Build request payload recording
    let req_payload = TestRequestPayload {
        method: method.clone(),
        url: url.clone(),
        headers: provider.request.headers.clone(),
        body: provider.request.body.as_ref().map(|v| serde_json::to_string_pretty(v).unwrap_or_default()),
    };

    let mut req_builder = match method.as_str() {
        "POST" => state.client.post(&url),
        _ => state.client.get(&url),
    };

    for (k, v) in &provider.request.headers {
        req_builder = req_builder.header(k, v);
    }

    if let Some(body) = &provider.request.body {
        let body_str = serde_json::to_string(body).unwrap_or_default();
        req_builder = req_builder.header("Content-Type", "application/json").body(body_str);
    }

    // Attempt request
    match req_builder.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let mut res_headers = HashMap::new();
            for (k, v) in resp.headers() {
                res_headers.insert(k.as_str().to_string(), v.to_str().unwrap_or("").to_string());
            }
            let body_text = resp.text().await.unwrap_or_default();
            Ok(TestResult {
                success: true,
                expected_symbol: symbol,
                request: req_payload,
                response: Some(TestResponsePayload {
                    status,
                    headers: res_headers,
                    body: body_text,
                }),
                error_msg: None,
            })
        }
        Err(e) => {
            Ok(TestResult {
                success: false,
                expected_symbol: symbol,
                request: req_payload,
                response: None,
                error_msg: Some(e.to_string()),
            })
        }
    }
}

#[tauri::command]
fn save_config(
    config: AppConfig,
    state: tauri::State<Arc<crate::daemon::AppState>>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    crate::config::save_config(&config)?;
    let mut st = state.config.blocking_write();
    *st = config.clone();
    drop(st);
    crate::tray::rebuild_and_set_menu(&app_handle, &config);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let initial_config = crate::config::load_config().unwrap_or_default();
    let app_state = Arc::new(crate::daemon::AppState::new(initial_config.clone()));

    tauri::Builder::default()
        .manage(app_state.clone())
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            crate::daemon::spawn_daemon(app.handle().clone(), app_state.clone());

            let menu = crate::tray::build_tray_menu(app.handle(), &initial_config)?;

            let _tray = TrayIconBuilder::with_id("main")
                .title("行情加载中...")
                .menu(&menu)
                .on_menu_event(move |app: &tauri::AppHandle, event| {
                    let state = app.state::<Arc<crate::daemon::AppState>>();
                    crate::tray::handle_menu_event(app, event, state);
                })
                .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_config, save_config, test_fetch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
