use crate::models::{AppConfig, Instrument, Provider};
use std::sync::Arc;
use tauri::{
    menu::{CheckMenuItem, IsMenuItem, Menu, MenuEvent, MenuItem, Submenu},
    AppHandle, Manager, Wry,
};

pub fn build_tray_menu(app: &AppHandle, config: &AppConfig) -> tauri::Result<Menu<Wry>> {
    // 1. 选择品种 (Select Instrument)
    let mut instrument_items = Vec::new();
    if let Some(provider) = config
        .providers
        .iter()
        .find(|p| p.key == config.defaults.provider)
    {
        for inst in provider.get_instruments() {
            let is_selected_inst = inst.label == config.defaults.instrument;

            let i_item = CheckMenuItem::with_id(
                app,
                format!("instrument:{}:{}", provider.key, inst.label),
                &inst.label,
                true,
                is_selected_inst,
                None::<&str>,
            )?;
            instrument_items.push(i_item);
        }
    }

    let instruments_submenu = Submenu::with_items(
        app,
        "选择品种",
        true,
        &instrument_items
            .iter()
            .map(|i| i as &dyn IsMenuItem<Wry>)
            .collect::<Vec<_>>(),
    )?;

    // 2. 切换数据源 (Switch Provider)
    let mut provider_items = Vec::new();
    for provider in &config.providers {
        let is_selected_provider = provider.key == config.defaults.provider;

        let p_item = CheckMenuItem::with_id(
            app,
            format!("provider:{}", provider.key),
            &provider.label,
            true,
            is_selected_provider,
            None::<&str>,
        )?;
        provider_items.push(p_item);
    }

    let providers_submenu = Submenu::with_items(
        app,
        "切换数据源",
        true,
        &provider_items
            .iter()
            .map(|p| p as &dyn IsMenuItem<Wry>)
            .collect::<Vec<_>>(),
    )?;

    // 3. 菜单栏显示品种名称 (Show Instrument Name)
    let show_name_item = CheckMenuItem::with_id(
        app,
        "toggle_show_name",
        "菜单栏显示品种名称",
        true,
        config.defaults.show_instrument_name,
        None::<&str>,
    )?;

    // 4. 配置 (Config)
    let show_i = MenuItem::with_id(app, "show", "配置", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // 动态拼接最后的主菜单
    let menu = Menu::with_items(
        app,
        &[
            &instruments_submenu as &dyn IsMenuItem<Wry>,
            &providers_submenu as &dyn IsMenuItem<Wry>,
            &show_name_item as &dyn IsMenuItem<Wry>,
            &show_i as &dyn IsMenuItem<Wry>,
            &quit_i as &dyn IsMenuItem<Wry>,
        ],
    )?;

    Ok(menu)
}

pub fn handle_menu_event(
    app: &AppHandle,
    event: MenuEvent,
    state: tauri::State<Arc<crate::daemon::AppState>>,
) {
    let id_str = event.id.as_ref();

    if id_str == "quit" {
        std::process::exit(0);
    } else if id_str == "show" {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = window.set_focus();
        }
    } else if id_str == "toggle_show_name" {
        let mut config = state.config.blocking_write();
        config.defaults.show_instrument_name = !config.defaults.show_instrument_name;
        let cloned_config = config.clone();
        drop(config);

        let _ = crate::config::save_config(&cloned_config);
        rebuild_and_set_menu(app, &cloned_config);

        // Let daemon update immediately instead of waiting 1s (optional)
    } else if let Some(rest) = id_str.strip_prefix("provider:") {
        let provider_key = rest.to_string();
        update_selected_provider(app, &state, provider_key);
    } else if let Some(rest) = id_str.strip_prefix("instrument:") {
        let parts: Vec<&str> = rest.splitn(2, ':').collect();
        if parts.len() == 2 {
            let provider_key = parts[0].to_string();
            let instrument_label = parts[1].to_string();
            update_selected_instrument(app, &state, provider_key, instrument_label);
        }
    }
}

fn update_selected_provider(
    app: &AppHandle,
    state: &tauri::State<Arc<crate::daemon::AppState>>,
    provider_key: String,
) {
    let mut config = state.config.blocking_write();
    if config.defaults.provider == provider_key {
        return; // nothing changed
    }

    config.defaults.provider = provider_key.clone();

    // Auto pickup the first instrument if existing one is not compatible
    if let Some(provider) = config.providers.iter().find(|p| p.key == provider_key) {
        let instruments = provider.get_instruments();
        if !instruments
            .iter()
            .any(|i| i.label == config.defaults.instrument)
        {
            if let Some(first) = instruments.first() {
                config.defaults.instrument = first.label.clone();
            }
        }
    }

    let cloned_config = config.clone();
    drop(config); // Release write lock before I/O

    let _ = crate::config::save_config(&cloned_config);
    rebuild_and_set_menu(app, &cloned_config);
}

fn update_selected_instrument(
    app: &AppHandle,
    state: &tauri::State<Arc<crate::daemon::AppState>>,
    provider_key: String,
    instrument_label: String,
) {
    let mut config = state.config.blocking_write();
    config.defaults.provider = provider_key;
    config.defaults.instrument = instrument_label;

    let cloned_config = config.clone();
    drop(config);

    let _ = crate::config::save_config(&cloned_config);
    rebuild_and_set_menu(app, &cloned_config);
}

pub fn rebuild_and_set_menu(app: &AppHandle, config: &AppConfig) {
    if let Ok(new_menu) = build_tray_menu(app, config) {
        if let Some(tray) = app.tray_by_id("main") {
            let _ = tray.set_menu(Some(new_menu));
            println!("重建并更新托盘菜单成功！");
        }
    }
}
