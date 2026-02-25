//! Incremental analysis for watch mode

use std::collections::{HashSet, HashMap};
use std::path::PathBuf;

pub struct IncrementalAnalyzer {
    last_files: HashSet<PathBuf>,
    last_analysis: HashMap<PathBuf, usize>, // violations per file
    threshold: i32,
}

impl IncrementalAnalyzer {
    pub fn new() -> Self {
        Self {
            last_files: HashSet::new(),
            last_analysis: HashMap::new(),
            threshold: 5,
        }
    }

    /// Detect files that changed since last analysis
    pub fn detect_changes(&mut self, current_files: HashSet<PathBuf>) -> Vec<PathBuf> {
        let new_files: Vec<_> = current_files
            .iter()
            .filter(|f| !self.last_files.contains(*f))
            .cloned()
            .collect();

        self.last_files = current_files;
        new_files
    }

    /// Decide if full rescan is needed
    pub fn should_full_rescan(&self, violations_now: usize, violations_last: usize) -> bool {
        let diff = (violations_now as i32 - violations_last as i32).abs();
        diff > self.threshold
    }

    /// Update analysis state
    pub fn update(&mut self, file: PathBuf, violation_count: usize) {
        self.last_analysis.insert(file, violation_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_new_files() {
        let mut analyzer = IncrementalAnalyzer::new();
        let files = vec![PathBuf::from("test.rs"), PathBuf::from("main.rs")]
            .into_iter()
            .collect();

        let new = analyzer.detect_changes(files);
        assert_eq!(new.len(), 2);
    }

    #[test]
    fn test_rescan_threshold() {
        let analyzer = IncrementalAnalyzer::new();
        assert!(!analyzer.should_full_rescan(10, 5)); // diff = 5, not > threshold
        assert!(analyzer.should_full_rescan(20, 5)); // diff = 15 > threshold
    }
}
