use architect_linter_pro::cache::{HybridCache, FileCacheEntry};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_three_layer_cache_with_promotion() {
    let temp_dir = TempDir::new().unwrap();
    let config_hash = "test_config".to_string();

    let mut cache = HybridCache::new(10, temp_dir.path(), config_hash.clone()).unwrap();

    // 1. Put file into cache
    let file_path = PathBuf::from("test.ts");
    let entry = FileCacheEntry {
        content_hash: "hash123".to_string(),
        violations: vec![],
        long_functions: vec![],
        import_count: 1,
        function_count: 1,
    };

    cache.put(file_path.clone(), entry);

    // 2. Get from memory (Layer 1)
    let result = cache.get(&file_path);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "hash123");

    // 3. Clear memory to force disk access
    cache.clear_memory();

    // 4. Get should promote from disk (Layer 2) to memory
    let result2 = cache.get(&file_path);
    assert!(result2.is_some(), "Should promote from disk to memory");
    assert_eq!(result2.unwrap(), "hash123");

    // 5. Second get should hit memory (not disk)
    let result3 = cache.get(&file_path);
    assert_eq!(result3, Some("hash123".to_string()));
}
