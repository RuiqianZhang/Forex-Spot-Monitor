use crate::models::{AppConfig, QuoteSnapshot};
use rquest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tauri::AppHandle;
use tauri::Manager;
use tauri::image;

fn get_trend_icon(change: f64) -> Option<tauri::image::Image<'static>> {
    if change.abs() < f64::EPSILON {
        return None;
    }

    let is_up = change > 0.0;
    let size: usize = 44;
    let mut rgba = vec![0u8; size * size * 4];

    let (r, g, b) = if is_up {
        (52u8, 199u8, 89u8)
    } else {
        (255u8, 59u8, 48u8)
    };

    // 点到线段距离，端点自然形成圆弧 cap
    let dist_seg = |px: f64, py: f64, ax: f64, ay: f64, bx: f64, by: f64| -> f64 {
        let dx = bx - ax;
        let dy = by - ay;
        let len_sq = dx * dx + dy * dy;
        if len_sq < 1e-10 {
            return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
        }
        let t = ((px - ax) * dx + (py - ay) * dy) / len_sq;
        let t = t.clamp(0.0, 1.0);
        ((px - (ax + t * dx)).powi(2) + (py - (ay + t * dy)).powi(2)).sqrt()
    };

    // 箭头各点坐标（44x44 画布）
    let cx = 22.0_f64;
    // 上箭头 ↑：尖朝上；下箭头 ↓：尖朝下
    let (tip_y, shaft_end_y, arm_end_y, arm_dx) = if is_up {
        (9.0_f64, 35.0, 22.0, 11.0) // 竖杆从 y=9 到 y=35，箭头臂到 y=22 ±11
    } else {
        (35.0_f64, 9.0, 22.0, 11.0)
    };

    let stroke_r = 2.6; // 半线宽（决定线条粗细）
    let soft = 0.9;     // 抗锯齿软化范围

    for py in 0..size {
        for px in 0..size {
            let x = px as f64 + 0.5;
            let y = py as f64 + 0.5;

            // ① 竖杆
            let d_shaft = dist_seg(x, y, cx, tip_y, cx, shaft_end_y);
            // ② 左臂
            let d_left  = dist_seg(x, y, cx, tip_y, cx - arm_dx, arm_end_y);
            // ③ 右臂
            let d_right = dist_seg(x, y, cx, tip_y, cx + arm_dx, arm_end_y);

            let d = d_shaft.min(d_left).min(d_right);
            let alpha = ((stroke_r + soft - d) / soft).clamp(0.0, 1.0);

            if alpha > 0.0 {
                let a = (alpha * 255.0) as u8;
                let idx = (py * size + px) * 4;
                rgba[idx]     = r;
                rgba[idx + 1] = g;
                rgba[idx + 2] = b;
                rgba[idx + 3] = a;
            }
        }
    }

    Some(tauri::image::Image::new_owned(rgba, size as u32, size as u32))
}

pub struct AppState {
    pub config: RwLock<AppConfig>,
    pub client: Client,
    pub latest_quotes: RwLock<std::collections::HashMap<String, QuoteSnapshot>>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let client = Client::builder()
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            config: RwLock::new(config),
            client,
            latest_quotes: RwLock::new(std::collections::HashMap::new()),
        }
    }
}

pub fn spawn_daemon(app: AppHandle, state: Arc<AppState>) {
    tauri::async_runtime::spawn(async move {
        loop {
            let config = state.config.read().await.clone();
            let interval_secs = config.defaults.refresh_interval.max(1);
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
            
            // Just a placeholder structure - we will fill in the real 
            // fetching and parsing logic later.
            let provider_key = config.defaults.provider.clone();
            let instrument_label = config.defaults.instrument.clone();
            
            println!("Tick: provider={}, instrument={}", provider_key, instrument_label);

            if let Some(provider) = config.providers.iter().find(|p| p.key == provider_key) {
                if let Some(instrument) = provider.get_instruments().iter().find(|i| i.label == instrument_label) {
                    println!("Found provider and instrument, start fetching...");
                    // Simulate fetching -> Real Fetching
                    let url = provider.request.url_template.replace("{symbol}", &instrument.symbol);
                    match state.client.get(&url).send().await {
                        Ok(resp) => {
                            match crate::parsers::parse_response(provider, instrument, resp).await {
                                Ok(mut snapshot) => {
                                    // Load previous cache for diff comparison
                                    let cache_key = snapshot.source_key.clone();
                                    let mut latest_map = state.latest_quotes.write().await;
                                    
                                    if let Some(prev) = latest_map.get(&cache_key) {
                                        snapshot.change_value = Some(snapshot.price - prev.price);
                                        if prev.price > 0.0 {
                                            snapshot.change_percent = Some(snapshot.change_value.unwrap() / prev.price * 100.0);
                                        }
                                    }
                                    
                                    let change = snapshot.change_value.unwrap_or(0.0);
                                    let title = if config.defaults.show_instrument_name {
                                        format!("{} {:.2}", instrument.label, snapshot.price)
                                    } else {
                                        format!("{:.2}", snapshot.price)
                                    };

                                    latest_map.insert(cache_key, snapshot);
                                    drop(latest_map);

                                    match app.tray_by_id("main") {
                                        Some(tray) => {
                                            let icon = get_trend_icon(change);
                                            let _ = tray.set_icon(icon);
                                            if let Err(e) = tray.set_title(Some(title.clone())) {
                                                println!("Failed to set MAC tray title: {:?}", e);
                                            } else {
                                                println!("Success set tray title to: {}", title);
                                            }
                                        }
                                        None => {
                                            println!("Tray 'main' NOT FOUND! Cannot set title: {}", title);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("解析行情返回数据遇到错误: {e:?}");
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("请求网络接口时遇到错误: {e:?}");
                        }
                    }
                }
            }
        }
    });
}
