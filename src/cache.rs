use crate::analysis_result::{CategorizedViolation, LongFunction};
use crate::config::LinterContext;
use crate::memory_cache::MemoryCache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const CACHE_VERSION: u32 = 1;
const CACHE_DIR: &str = ".architect-cache";
const CACHE_FILE: &str = "cache.json";

/// Per-file cached analysis data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCacheEntry {
    pub content_hash: String,
    pub violations: Vec<CategorizedViolation>,
    pub long_functions: Vec<LongFunction>,
    pub import_count: usize,
    pub function_count: usize,
}

/// Disk-persisted analysis cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisCache {
    pub version: u32,
    pub config_hash: String,
    pub files: HashMap<String, FileCacheEntry>,
}

/// Compute xxh3-64 hash of raw bytes, returned as hex string
pub fn hash_content(bytes: &[u8]) -> String {
    format!("{:016x}", xxhash_rust::xxh3::xxh3_64(bytes))
}

/// Compute a config hash from the fields that affect analysis results.
/// If this changes, the entire cache is invalidated.
pub fn hash_config(ctx: &LinterContext) -> String {
    let mut data = format!("max_lines={};imports=", ctx.max_lines);
    for rule in &ctx.forbidden_imports {
        data.push_str(&rule.from);
        data.push(':');
        data.push_str(&rule.to);
        data.push(';');
    }
    hash_content(data.as_bytes())
}

impl AnalysisCache {
    /// Create a new empty cache with the given config hash
    pub fn new(config_hash: String) -> Self {
        Self {
            version: CACHE_VERSION,
            config_hash,
            files: HashMap::new(),
        }
    }

    /// Load cache from disk. Returns None if the file doesn't exist,
    /// can't be parsed, or has a version/config mismatch.
    pub fn load(project_root: &Path, config_hash: &str) -> Option<Self> {
        let cache_path = project_root.join(CACHE_DIR).join(CACHE_FILE);
        let data = fs::read_to_string(&cache_path).ok()?;
        let cache: Self = serde_json::from_str(&data).ok()?;

        if cache.version != CACHE_VERSION || cache.config_hash != config_hash {
            return None;
        }

        Some(cache)
    }

    /// Persist cache to disk
    pub fn save(&self, project_root: &Path) -> io::Result<()> {
        let cache_dir = project_root.join(CACHE_DIR);
        fs::create_dir_all(&cache_dir)?;
        let cache_path = cache_dir.join(CACHE_FILE);
        let json = serde_json::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(&cache_path, json)
    }

    /// Convert an absolute path to a normalized relative key (forward slashes)
    pub fn normalize_path(path: &Path, project_root: &Path) -> String {
        path.strip_prefix(project_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    /// Look up a cached entry. Returns Some only if the content hash matches.
    pub fn get(&self, key: &str, content_hash: &str) -> Option<&FileCacheEntry> {
        self.files.get(key).filter(|e| e.content_hash == content_hash)
    }

    /// Insert or update a cache entry
    pub fn insert(&mut self, key: String, entry: FileCacheEntry) {
        self.files.insert(key, entry);
    }

    /// Remove a file from the cache (e.g. when a watched file changes)
    pub fn remove(&mut self, key: &str) {
        self.files.remove(key);
    }
}

/// Hybrid cache combining memory (LRU) and disk cache layers
pub struct HybridCache {
    memory: MemoryCache,
    disk: AnalysisCache,
}

impl HybridCache {
    pub fn new(memory_capacity: usize, project_root: &Path, config_hash: String) -> Result<Self, Box<dyn std::error::Error>> {
        // Load disk cache from disk if it exists, otherwise create new
        let disk = AnalysisCache::load(project_root, &config_hash)
            .unwrap_or_else(|| AnalysisCache::new(config_hash));

        Ok(Self {
            memory: MemoryCache::new(memory_capacity),
            disk,
        })
    }

    pub fn get(&self, path: &PathBuf) -> Option<String> {
        // Layer 1: Memory cache
        if let Some(content) = self.memory.get(path) {
            return Some(content);
        }

        // Layer 2: Disk cache
        let key = path.to_string_lossy().replace('\\', "/");
        if let Some(entry) = self.disk.files.get(&key) {
            // Promote to memory cache
            self.memory.put(path.clone(), entry.content_hash.clone());
            return Some(entry.content_hash.clone());
        }

        None
    }

    pub fn put(&mut self, path: PathBuf, entry: FileCacheEntry) {
        let key = path.to_string_lossy().replace('\\', "/");
        // Update both layers
        self.memory.put(path, entry.content_hash.clone());
        self.disk.files.insert(key, entry);
    }

    pub fn clear_memory(&self) {
        self.memory.clear();
    }

    pub fn save(&self, project_root: &Path) -> io::Result<()> {
        self.disk.save(project_root)
    }
}
