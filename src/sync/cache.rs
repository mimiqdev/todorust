use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Cache {
    pub sync_token: String,
    pub cached_at: i64,
    pub data: CacheData,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CacheData {
    pub projects: Vec<super::models::SyncProject>,
    pub items: Vec<super::models::SyncTask>,
    pub sections: Vec<super::models::SyncSection>,
    pub labels: Vec<super::models::SyncLabel>,
    pub filters: Vec<super::models::SyncFilter>,
}

pub struct CacheManager {
    cache_path: PathBuf,
}

impl CacheManager {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("todorust");
        Self {
            cache_path: config_dir.join("cache.json"),
        }
    }

    pub fn load(&self) -> Result<Option<Cache>, crate::error::TodoError> {
        if !self.cache_path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&self.cache_path)?;
        let cache: Cache = serde_json::from_str(&content).map_err(|e| {
            crate::error::TodoError::InvalidInput(format!("Failed to parse cache: {}", e))
        })?;
        Ok(Some(cache))
    }

    pub fn save(&self, cache: &Cache) -> Result<(), crate::error::TodoError> {
        if let Some(parent) = self.cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(cache).map_err(|e| {
            crate::error::TodoError::InvalidInput(format!("Failed to serialize cache: {}", e))
        })?;
        std::fs::write(&self.cache_path, content)?;
        Ok(())
    }

    pub fn is_expired(&self, cache: &Cache, threshold_secs: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        (now - cache.cached_at) > threshold_secs as i64
    }
}
