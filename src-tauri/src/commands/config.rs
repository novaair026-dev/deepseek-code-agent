use anyhow::{Context, Result};
use keyring::Entry;
use serde::{Deserialize, Serialize};

const SERVICE_NAME: &str = "com.deepseek-code-agent";
const ACCOUNT_NAME: &str = "deepseek_api_key";

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyResult {
    pub configured: bool,
}

/// 检查 API Key 是否已配置
#[tauri::command]
pub fn get_api_key_configured() -> Result<ApiKeyResult, String> {
    match get_api_key_inner() {
        Ok(Some(_)) => Ok(ApiKeyResult { configured: true }),
        Ok(None) => Ok(ApiKeyResult { configured: false }),
        Err(e) => Err(format!("读取 API Key 失败: {}", e)),
    }
}

/// 设置 API Key（保存到系统钥匙串）
#[tauri::command]
pub fn set_api_key(api_key: String) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME).map_err(|e| e.to_string())?;
    entry.set_password(&api_key).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_api_key_inner() -> Result<Option<String>> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME).context("创建钥匙串条目失败")?;
    match entry.get_password() {
        Ok(key) if !key.is_empty() => Ok(Some(key)),
        Ok(_) => Ok(None),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(anyhow::anyhow!("读取钥匙串失败: {}", e)),
    }
}
