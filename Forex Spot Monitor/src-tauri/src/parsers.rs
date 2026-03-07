use crate::models::{Instrument, Provider, QuoteSnapshot, ParserConfig};
use chrono::{Local, TimeZone};
use std::error::Error;
use rquest::Response;

pub async fn parse_response(
    provider: &Provider,
    instrument: &Instrument,
    response: Response,
) -> Result<QuoteSnapshot, Box<dyn Error + Send + Sync>> {
    let raw_text = response.text().await?;
    let parser = &instrument.parser;

    match parser.r#type.as_str() {
        "sina_forex" | "sina_forex_text" => parse_sina_forex(provider, instrument, &raw_text),
        "sina_futures" | "sina_futures_text" => parse_sina_futures(provider, instrument, &raw_text),
        "swissquote_bbo" => parse_swissquote_bbo(provider, instrument, &raw_text),
        "gold_api_xau" => parse_gold_api_xau(provider, instrument, &raw_text),
        "json_fields" => parse_json_fields(provider, instrument, parser, &raw_text),
        _ => Err(format!("未注册的解析器: {}", parser.r#type).into()),
    }
}

fn extract_sina_payload(raw_text: &str, provider_symbol: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let re = regex::Regex::new(&format!(r#"var hq_str_{}="(.*?)";"#, regex::escape(provider_symbol)))?;
    if let Some(caps) = re.captures(raw_text.trim()) {
        let payload = caps.get(1).map_or("", |m| m.as_str());
        if payload.is_empty() {
            return Err("新浪返回空数据".into());
        }
        return Ok(payload.to_string());
    }
    Err(format!("新浪返回格式异常. Payload: {:?}", raw_text).into())
}

fn parse_sina_forex(
    provider: &Provider,
    instrument: &Instrument,
    raw_text: &str,
) -> Result<QuoteSnapshot, Box<dyn Error + Send + Sync>> {
    let payload = extract_sina_payload(raw_text, &instrument.symbol)?;
    let fields: Vec<&str> = payload.split(',').collect();
    if fields.len() < 11 {
        return Err("新浪外汇字段不足".into());
    }

    let quote_time = fields[0].to_string();
    let price: f64 = fields[3].parse()?;
    let name = if fields[9].is_empty() { &instrument.label } else { fields[9] };
    let trade_date = fields[10];

    Ok(QuoteSnapshot {
        source_key: format!("{}:{}", provider.key, instrument.symbol),
        source_label: instrument.label.clone(),
        quote_time: quote_time.clone(),
        price,
        update_time: format!("{} {}", trade_date, quote_time).trim().to_string(),
        fetched_at: Local::now().to_rfc3339(),
        data_source: format!("{} / {} / {}", provider.label, instrument.symbol, name),
        bid: None,
        ask: None,
        change_value: None,
        change_percent: None,
    })
}

fn parse_sina_futures(
    provider: &Provider,
    instrument: &Instrument,
    raw_text: &str,
) -> Result<QuoteSnapshot, Box<dyn Error + Send + Sync>> {
    let payload = extract_sina_payload(raw_text, &instrument.symbol)?;
    let fields: Vec<&str> = payload.split(',').collect();
    if fields.len() < 15 {
        return Err("新浪期货字段不足".into());
    }

    let price: f64 = fields[0].parse()?;
    let bid = fields[2].parse().ok();
    let ask = fields[3].parse().ok();
    let quote_time = fields[6].to_string();
    let trade_date = fields[12];
    let name = if fields[13].is_empty() { &instrument.label } else { fields[13] };

    Ok(QuoteSnapshot {
        source_key: format!("{}:{}", provider.key, instrument.symbol),
        source_label: instrument.label.clone(),
        quote_time: quote_time.clone(),
        price,
        update_time: format!("{} {}", trade_date, quote_time).trim().to_string(),
        fetched_at: Local::now().to_rfc3339(),
        data_source: format!("{} / {} / {}", provider.label, instrument.symbol, name),
        bid,
        ask,
        change_value: None,
        change_percent: None,
    })
}

fn parse_swissquote_bbo(
    provider: &Provider,
    instrument: &Instrument,
    raw_text: &str,
) -> Result<QuoteSnapshot, Box<dyn Error + Send + Sync>> {
    let payload: serde_json::Value = serde_json::from_str(raw_text)?;
    let venues = payload.as_array().ok_or("Swissquote 返回的不是数组")?;
    if venues.is_empty() {
        return Err("Swissquote 返回空数据".into());
    }

    let mut best_bid: Option<f64> = None;
    let mut best_ask: Option<f64> = None;
    let mut best_spread = f64::MAX;
    let mut best_ts = None;
    let mut best_platform = provider.label.clone();
    let mut best_profile = "default".to_string();

    for venue in venues {
        let ts = venue.get("ts").and_then(|v| {
            if let Some(s) = v.as_str() {
                Some(s.to_string())
            } else if let Some(n) = v.as_i64() {
                Some(n.to_string())
            } else if let Some(n) = v.as_u64() {
                Some(n.to_string())
            } else {
                None
            }
        });
        let topo = venue.get("topo").and_then(|v| v.as_object());
        
        let platform = topo
            .and_then(|t| t.get("platform"))
            .and_then(|p| p.as_str())
            .unwrap_or(&provider.label);

        if let Some(profiles) = venue.get("spreadProfilePrices").and_then(|v| v.as_array()) {
            for profile in profiles {
                if let (Some(b), Some(a)) = (
                    profile.get("bid").and_then(|v| v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse().ok()))),
                    profile.get("ask").and_then(|v| v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse().ok()))),
                ) {
                    let spread = a - b;
                    if spread < best_spread {
                        best_spread = spread;
                        best_bid = Some(b);
                        best_ask = Some(a);
                        best_ts = ts.clone();
                        best_platform = platform.to_string();
                        best_profile = profile.get("spreadProfile")
                            .and_then(|v| v.as_str())
                            .unwrap_or("default")
                            .to_string();
                    }
                }
            }
        }
    }

    let ts_str = best_ts.ok_or("Swissquote 缺少有效报价时戳")?;
    let ts_ms: i64 = ts_str.parse()?;
    let quote_dt = Local.timestamp_millis_opt(ts_ms).single().ok_or("无效的时间戳")?;
    
    let bid = best_bid.unwrap_or(0.0);
    let ask = best_ask.unwrap_or(0.0);
    let price = (bid + ask) / 2.0;

    Ok(QuoteSnapshot {
        source_key: format!("{}:{}", provider.key, instrument.symbol),
        source_label: instrument.label.clone(),
        quote_time: quote_dt.format("%H:%M:%S").to_string(),
        price,
        update_time: quote_dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        fetched_at: Local::now().to_rfc3339(),
        data_source: format!("{} / {} / {} / {}", provider.label, instrument.symbol, best_platform, best_profile),
        bid: Some(bid),
        ask: Some(ask),
        change_value: None,
        change_percent: None,
    })
}

fn parse_gold_api_xau(
    provider: &Provider,
    instrument: &Instrument,
    raw_text: &str,
) -> Result<QuoteSnapshot, Box<dyn Error + Send + Sync>> {
    let payload: serde_json::Value = serde_json::from_str(raw_text)?;
    
    let price_val = payload.get("price").ok_or("gold-api 缺少 price 字段")?;
    let price = price_val.as_f64().or_else(|| price_val.as_str().and_then(|s| s.parse().ok())).ok_or("price 不是数字")?;
    
    let updated_at = payload.get("updatedAt").and_then(|v| v.as_str()).ok_or("gold-api 缺少 updatedAt 字段")?;
    
    // updated_at e.g. "2023-10-25T14:30:00.000Z"
    let dt = chrono::DateTime::parse_from_rfc3339(&updated_at.replace("Z", "+00:00"))?.with_timezone(&Local);

    Ok(QuoteSnapshot {
        source_key: format!("{}:{}", provider.key, instrument.symbol),
        source_label: instrument.label.clone(),
        quote_time: dt.format("%H:%M:%S").to_string(),
        price,
        update_time: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        fetched_at: Local::now().to_rfc3339(),
        data_source: format!("{} / {}", provider.label, instrument.symbol),
        bid: None,
        ask: None,
        change_value: None,
        change_percent: None,
    })
}

fn parse_json_fields(
    provider: &Provider,
    instrument: &Instrument,
    parser: &ParserConfig,
    raw_text: &str,
) -> Result<QuoteSnapshot, Box<dyn Error + Send + Sync>> {
    let payload: serde_json::Value = serde_json::from_str(raw_text)?;
    
    let price = extract_json_float(&payload, parser.options.get("price_path"))
        .ok_or("无法提取 price_path")?;
    
    let bid = extract_json_float(&payload, parser.options.get("bid_path"));
    let ask = extract_json_float(&payload, parser.options.get("ask_path"));
    
    let quote_time = extract_json_string(&payload, parser.options.get("quote_time_path"))
        .unwrap_or_else(|| "--".to_string());
    
    let update_time = extract_json_string(&payload, parser.options.get("update_time_path"))
        .unwrap_or_else(|| quote_time.clone());
        
    let name = extract_json_string(&payload, parser.options.get("name_path"))
        .unwrap_or_else(|| instrument.label.clone());

    let data_source_template = parser.options.get("data_source_template")
        .map(|s| s.as_str())
        .unwrap_or("{provider_label} / {symbol} / {name}");

    let data_source = data_source_template
        .replace("{provider_label}", &provider.label)
        .replace("{provider_key}", &provider.key)
        .replace("{symbol}", &instrument.symbol)
        .replace("{instrument_label}", &instrument.label)
        .replace("{name}", &name);

    Ok(QuoteSnapshot {
        source_key: format!("{}:{}", provider.key, instrument.symbol),
        source_label: instrument.label.clone(),
        quote_time,
        price,
        update_time,
        fetched_at: Local::now().to_rfc3339(),
        data_source,
        bid,
        ask,
        change_value: None,
        change_percent: None,
    })
}

fn extract_json_value<'a>(payload: &'a serde_json::Value, path: Option<&String>) -> Option<&'a serde_json::Value> {
    let path = path?;
    if path.is_empty() { return None; }
    
    // Simply walk the tree using dot notation string or use jsonpath
    let mut current = payload;
    for segment in path.split('.') {
        if segment.is_empty() { continue; }
        if let Some(arr) = current.as_array() {
            if let Ok(idx) = segment.parse::<usize>() {
                current = arr.get(idx)?;
            } else {
                return None;
            }
        } else if let Some(obj) = current.as_object() {
            current = obj.get(segment)?;
        } else {
            return None;
        }
    }
    Some(current)
}

fn extract_json_float(payload: &serde_json::Value, path: Option<&String>) -> Option<f64> {
    let val = extract_json_value(payload, path)?;
    val.as_f64().or_else(|| val.as_str().and_then(|s| s.parse().ok()))
}

fn extract_json_string(payload: &serde_json::Value, path: Option<&String>) -> Option<String> {
    let val = extract_json_value(payload, path)?;
    val.as_str().map(|s| s.to_string()).or_else(|| Some(val.to_string()))
}
