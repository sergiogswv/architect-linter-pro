use architect_linter_pro::memory_cache::MemoryCache;
use std::path::PathBuf;

#[test]
fn test_memory_cache_put_and_get() {
    let cache = MemoryCache::new(2);

    let path = PathBuf::from("test.ts");
    let content = "export const x = 1;".to_string();

    cache.put(path.clone(), content.clone());

    let retrieved = cache.get(&path);
    assert_eq!(retrieved, Some(content));
}

#[test]
fn test_memory_cache_lru_eviction() {
    let cache = MemoryCache::new(2);

    cache.put(PathBuf::from("a.ts"), "a".to_string());
    cache.put(PathBuf::from("b.ts"), "b".to_string());
    cache.put(PathBuf::from("c.ts"), "c".to_string());

    // "a" should be evicted (LRU)
    assert!(cache.get(&PathBuf::from("a.ts")).is_none());
    assert!(cache.get(&PathBuf::from("b.ts")).is_some());
    assert!(cache.get(&PathBuf::from("c.ts")).is_some());
}

#[test]
fn test_memory_cache_update_resets_lru() {
    let cache = MemoryCache::new(2);

    cache.put(PathBuf::from("a.ts"), "a".to_string());
    cache.put(PathBuf::from("b.ts"), "b".to_string());

    // Access "a" to make it recently used
    cache.get(&PathBuf::from("a.ts"));

    cache.put(PathBuf::from("c.ts"), "c".to_string());

    // "b" should be evicted (not recently accessed)
    assert!(cache.get(&PathBuf::from("b.ts")).is_none());
    assert!(cache.get(&PathBuf::from("a.ts")).is_some());
}

#[test]
fn test_memory_cache_clear() {
    let cache = MemoryCache::new(2);

    cache.put(PathBuf::from("a.ts"), "a".to_string());
    cache.clear();

    assert!(cache.get(&PathBuf::from("a.ts")).is_none());
}
