#[cfg(test)]
mod tests {
    use architect_linter_pro::metrics::PerformanceMetrics;
    use std::time::Duration;

    #[test]
    fn test_metrics_collection() {
        let metrics = PerformanceMetrics::new();

        // Test initial state
        assert_eq!(metrics.total_time_ms, 0);
        assert_eq!(metrics.files_analyzed, 0);
        assert_eq!(metrics.files_from_cache, 0);
        assert_eq!(metrics.memory_cache_hits, 0);
        assert_eq!(metrics.disk_cache_hits, 0);
        assert_eq!(metrics.peak_memory_mb, 0);
        assert_eq!(metrics.threads_used, 0);

        // Test cache hit rate calculation
        assert_eq!(metrics.cache_hit_rate(), 0.0);

        // Test string representation
        let metrics_str = format!("{}", metrics);
        assert!(metrics_str.contains("PerformanceMetrics"));
        assert!(metrics_str.contains("total_time_ms: 0"));
        assert!(metrics_str.contains("files_analyzed: 0"));
    }

    #[test]
    fn test_metrics_json_export() {
        let mut metrics = PerformanceMetrics::new();

        // Update metrics with some test data
        metrics.total_time_ms = 1500;
        metrics.files_analyzed = 42;
        metrics.files_from_cache = 10;
        metrics.memory_cache_hits = 15;
        metrics.disk_cache_hits = 8;
        metrics.peak_memory_mb = 256;
        metrics.threads_used = 4;

        // Test JSON serialization
        let json = serde_json::to_string(&metrics).expect("Failed to serialize metrics");
        assert!(json.contains("\"total_time_ms\":1500"));
        assert!(json.contains("\"files_analyzed\":42"));
        assert!(json.contains("\"memory_cache_hits\":15"));
        assert!(json.contains("\"disk_cache_hits\":8"));

        // Test JSON deserialization
        let deserialized: PerformanceMetrics = serde_json::from_str(&json).expect("Failed to deserialize metrics");
        assert_eq!(deserialized.total_time_ms, 1500);
        assert_eq!(deserialized.files_analyzed, 42);
        // Cache hit rate: (15+8)/42 * 100 = 54.76...
        let expected_rate = 54.76;
        let actual_rate = deserialized.cache_hit_rate();
        assert!((actual_rate - expected_rate).abs() < 0.01);
    }
}