use crate::models::AppConfig;
use std::fs;
use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    path.push("Library/Application Support/Forex Spot Monitor/data_sources.json");
    path
}

pub fn load_config() -> Result<AppConfig, String> {
    let path = get_config_path();
    if !path.exists() {
        // Fallback to embedded default config if the override is not found
        let default_contents = include_str!("../data_sources.json");
        let config: AppConfig =
            serde_json::from_str(default_contents).unwrap_or_else(|_| AppConfig::default());
        return Ok(config);
    }

    let contents = fs::read_to_string(&path).map_err(|e| format!("无法读取配置文件: {}", e))?;

    let config: AppConfig =
        serde_json::from_str(&contents).map_err(|e| format!("解析配置文件失败: {}", e))?;

    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = get_config_path();

    // 确保目标目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建配置目录失败: {}", e))?;
    }

    let contents =
        serde_json::to_string_pretty(config).map_err(|e| format!("序列化配置失败: {}", e))?;

    fs::write(&path, contents).map_err(|e| format!("写入配置文件失败: {}", e))?;

    Ok(())
}
