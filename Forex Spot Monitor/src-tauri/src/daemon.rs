use crate::models::{AppConfig, QuoteSnapshot, Provider};
use rquest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tauri::AppHandle;

pub fn build_request(
    client: &Client,
    provider: &Provider,
    symbol: &str,
) -> rquest::RequestBuilder {
    let url = provider.request.url_template.replace("{symbol}", symbol);
    let method = provider.request.method.to_uppercase();

    let mut req_builder = match method.as_str() {
        "POST" => client.post(&url),
        _ => client.get(&url),
    };

    // 1. 设置 Headers
    for (k, v) in &provider.request.headers {
        req_builder = req_builder.header(k, v);
    }

    // 2. 设置 JSON Body
    if let Some(body) = &provider.request.body {
        req_builder = req_builder.json(body);
    }

    // 3. 移除不支持的 Impersonate (浏览器模拟) 逻辑以确保编译通过
    // 如果后续需要该功能，需核实 rquest 5.1.0 的正确实现方式（通常在 ClientBuilder 级别）

    // 4. 设置超时 (应用配置超时，兜底 30s)
    let timeout_secs = if provider.request.timeout > 0.0 { provider.request.timeout } else { 15.0 };
    req_builder = req_builder.timeout(Duration::from_secs_f64(timeout_secs.min(60.0)));

    req_builder
}

fn get_trend_icon(change: f64) -> Option<tauri::image::Image<'static>> {
    let is_none = change.abs() < f64::EPSILON;

    let is_up = change > 0.0;
    let size: usize = 32;
    let mut rgba = vec![0u8; size * size * 4];

    let (r, g, b) = if is_none {
        (0u8, 0u8, 0u8) // Transparent, color doesn't matter
    } else if is_up {
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

    // 箭头各点坐标（32x32 画布，确保内容完整显示的同时最大限度靠右）
    let cx = 20.0_f64; 
    // 上箭头 ↑：尖朝上；下箭头 ↓：尖朝下
    let (tip_y, shaft_end_y, arm_end_y, arm_dx) = if is_up {
        (6.0_f64, 26.0, 16.0, 8.0) 
    } else {
        (26.0_f64, 6.0, 16.0, 8.0)
    };

    let stroke_r = 1.3; // Thinner arrow for better aesthetic
    let soft = 0.8;     // Sharper anti-aliasing

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

            let d = if is_none { 100.0 } else { d_shaft.min(d_left).min(d_right) };
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

                    let req_builder = build_request(&state.client, provider, &instrument.symbol);
                    
                    match req_builder.send().await {
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
                                    // Remove all padding to achieve maximum closeness with the icon
                                    let price_str = format!("{:.2}", snapshot.price);
                                    let title = if config.defaults.show_instrument_name {
                                        format!("{} {}", instrument.label, price_str)
                                    } else {
                                        price_str
                                    };

                                    latest_map.insert(cache_key, snapshot);
                                    drop(latest_map);

                                    match app.tray_by_id("main") {
                                        Some(tray) => {
                                            let icon = get_trend_icon(change);
                                            let _ = tray.set_icon(icon);
                                            if let Err(e) = tray.set_title(Some(title.clone())) {
                                                eprintln!("Failed to set MAC tray title: {:?}", e);
                                            }
                                        }
                                        None => {
                                            eprintln!("Tray 'main' NOT FOUND!");
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("解析行情返回数据遇到错误: {e:?}");
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("请求网络接口时遇到错误 (超时或连接中断): {e:?}");
                        }
                    }
                }
            }
        }
    });
}
