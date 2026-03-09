use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppConfig {
    pub defaults: Defaults,
    pub providers: Vec<Provider>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Defaults {
    pub provider: String,
    pub instrument: String,
    #[serde(default = "default_show_instrument_name")]
    pub show_instrument_name: bool,
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval: u64,
}

fn default_show_instrument_name() -> bool {
    true
}

fn default_refresh_interval() -> u64 {
    1
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub key: String,
    pub label: String,
    pub request: RequestConfig,
    #[serde(default)]
    pub groups: Option<Vec<Group>>,
    #[serde(default)]
    pub instruments: Option<Vec<Instrument>>,
}

impl Provider {
    pub fn get_instruments(&self) -> Vec<Instrument> {
        if let Some(items) = &self.instruments {
            return items.clone();
        }

        let mut result = Vec::new();
        if let Some(groups) = &self.groups {
            for group in groups {
                for item in &group.instruments {
                    let mut i = item.clone();
                    i.group = group.name.clone();
                    result.push(i);
                }
            }
        }
        result
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub name: String,
    pub instruments: Vec<Instrument>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestConfig {
    #[serde(default = "default_method")]
    pub method: String,
    pub url_template: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default = "default_impersonate")]
    pub impersonate: String,
    #[serde(default = "default_timeout")]
    pub timeout: f64,
    #[serde(default)]
    pub verify: bool,
}

fn default_method() -> String {
    "GET".to_string()
}
fn default_impersonate() -> String {
    "chrome".to_string()
}
fn default_timeout() -> f64 {
    15.0
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Instrument {
    pub label: String,
    pub symbol: String,
    #[serde(default)]
    pub group: String,
    pub parser: ParserConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ParserConfig {
    #[serde(default = "default_parser_type")]
    pub r#type: String,
    #[serde(flatten)]
    pub options: HashMap<String, String>,
}

fn default_parser_type() -> String {
    "json_fields".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuoteSnapshot {
    pub source_key: String,
    pub source_label: String,
    pub quote_time: String,
    pub price: f64,
    pub update_time: String,
    pub fetched_at: String,
    pub data_source: String,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub change_value: Option<f64>,
    pub change_percent: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestRequestPayload {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestResponsePayload {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestResult {
    pub success: bool,
    pub expected_symbol: String,
    pub request: TestRequestPayload,
    pub response: Option<TestResponsePayload>,
    pub error_msg: Option<String>,
}
