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
    let mut rgba = vec![0u8; 16 * 16 * 4];
    
    let (r, g, b, a) = if is_up {
        (52u8, 199u8, 89u8, 255u8) // System Green
    } else {
        (255u8, 59u8, 48u8, 255u8) // System Red
    };

    // Draw arrow
    for y in 2..14 {
        for x in 2..14 {
            let mut draw = false;
            if is_up {
                if x == 7 || x == 8 {
                    if y >= 6 { draw = true; }
                }
                if y >= 2 && y <= 6 {
                    let w = y - 2; 
                    if (7 - w) <= x && x <= (8 + w) {
                        draw = true;
                    }
                }
            } else {
                if x == 7 || x == 8 {
                    if y <= 9 { draw = true; }
                }
                if y >= 9 && y <= 13 {
                    let w = 13 - y;
                    if (7 - w) <= x && x <= (8 + w) {
                        draw = true;
                    }
                }
            }

            if draw {
                let idx = (y * 16 + x) * 4;
                rgba[idx] = r;
                rgba[idx+1] = g;
                rgba[idx+2] = b;
                rgba[idx+3] = a;
            }
        }
    }
    
    Some(tauri::image::Image::new_owned(rgba, 16, 16))
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
        let mut ticker = interval(Duration::from_secs(1));
        loop {
            ticker.tick().await;
            let config = state.config.read().await.clone();
            
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
