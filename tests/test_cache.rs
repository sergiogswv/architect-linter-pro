use architect_linter_pro::cache::{AnalysisCache, FileCacheEntry};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_analysis_cache_basic_ops() {
    let temp_dir = TempDir::new().unwrap();
    let config_hash = "test_config".to_string();

    let mut cache = AnalysisCache::new(config_hash.clone());

    // 1. Put file into cache
    let file_path = PathBuf::from("test.ts");
    let key = AnalysisCache::normalize_path(&file_path, temp_dir.path());
    let entry = FileCacheEntry {
        content_hash: "hash123".to_string(),
        violations: vec![],
        long_functions: vec![],
        import_count: 1,
        function_count: 1,
    };

    cache.insert(key.clone(), entry);

    // 2. Get from memory
    let result = cache.get(&key, "hash123");
    assert!(result.is_some());
    assert_eq!(result.unwrap().content_hash, "hash123");

    // 3. Test persistence
    cache.save(temp_dir.path()).unwrap();

    // 4. Load from disk
    let loaded_cache =
        AnalysisCache::load(temp_dir.path(), &config_hash).expect("Should load cache");
    let result2 = loaded_cache.get(&key, "hash123");
    assert!(result2.is_some(), "Should load from disk");
    assert_eq!(result2.unwrap().content_hash, "hash123");
}
