use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Cache {
    pub sync_token: String,
    pub cached_at: i64,
    pub data: CacheData,
}

#[derive(Serialize, Deserialize, Clone, Default)]
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

    /// 清除缓存文件
    pub fn clear(&self) -> Result<(), crate::error::TodoError> {
        if self.cache_path.exists() {
            std::fs::remove_file(&self.cache_path)?;
        }
        Ok(())
    }

    /// 检查缓存文件是否存在
    pub fn exists(&self) -> bool {
        self.cache_path.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let cache_path = temp_dir.path().join("cache.json");

        let cache = Cache {
            sync_token: "test_token".to_string(),
            cached_at: 1234567890,
            data: CacheData::default(),
        };

        let manager = CacheManager {
            cache_path: cache_path.clone(),
        };
        manager.save(&cache).unwrap();

        let loaded = manager.load().unwrap().unwrap();
        assert_eq!(loaded.sync_token, "test_token");
        assert_eq!(loaded.cached_at, 1234567890);
    }

    #[test]
    fn test_cache_expired() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CacheManager {
            cache_path: temp_dir.path().join("cache.json"),
        };

        let old_cache = Cache {
            sync_token: "test".to_string(),
            cached_at: 1,
            data: CacheData::default(),
        };
        assert!(manager.is_expired(&old_cache, 300));

        let new_cache = Cache {
            sync_token: "test".to_string(),
            cached_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            data: CacheData::default(),
        };
        assert!(!manager.is_expired(&new_cache, 300));
    }

    #[test]
    fn test_cache_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CacheManager {
            cache_path: temp_dir.path().join("nonexistent.json"),
        };

        let result = manager.load().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_clear() {
        let temp_dir = TempDir::new().unwrap();
        let cache_path = temp_dir.path().join("cache.json");

        let cache = Cache {
            sync_token: "test".to_string(),
            cached_at: 123,
            data: CacheData::default(),
        };

        let manager = CacheManager {
            cache_path: cache_path.clone(),
        };
        manager.save(&cache).unwrap();
        assert!(manager.exists());

        manager.clear().unwrap();
        assert!(!manager.exists());
    }
}
