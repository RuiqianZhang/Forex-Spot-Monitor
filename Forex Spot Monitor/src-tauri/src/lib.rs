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
// 移除未使用的 rquest::Client 导入
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
    // 使用统一的构建请求逻辑
    let req_builder = crate::daemon::build_request(&state.client, &provider, &symbol);
    
    // Build request payload recording for UI
    let req_payload = TestRequestPayload {
        method: provider.request.method.to_uppercase(),
        url: provider.request.url_template.replace("{symbol}", &symbol),
        headers: provider.request.headers.clone(),
        body: provider.request.body.as_ref().map(|v| serde_json::to_string_pretty(v).unwrap_or_default()),
    };
    
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
                            #[cfg(target_os = "macos")]
                            let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            // macOS: 通过 NSApp 原生接口设置程序坞图标
            #[cfg(target_os = "macos")]
            {
                use std::path::PathBuf;
                use objc2::AnyThread;
                use objc2_app_kit::{NSApplication, NSImage};
                use objc2_foundation::{NSString, MainThreadMarker};
                let icon_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("icons/icon.icns");
                if icon_path.exists() {
                    unsafe {
                        let path_str = icon_path.to_str().unwrap_or("");
                        let ns_path = NSString::from_str(path_str);
                        if let Some(image) = NSImage::initWithContentsOfFile(
                            NSImage::alloc(),
                            &ns_path
                        ) {
                            let mtm = MainThreadMarker::new().unwrap();
                            let ns_app = NSApplication::sharedApplication(mtm);
                            ns_app.setApplicationIconImage(Some(&image));
                        }
                    }
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let _ = window.hide();
                api.prevent_close();
                #[cfg(target_os = "macos")]
                let _ = window.app_handle().set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_config, save_config, test_fetch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
