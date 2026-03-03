/// Framework detection module
///
/// This module provides traits and implementations for detecting
/// which frameworks are used in a project.

pub mod framework_detector;

pub use framework_detector::{
    detect_all_frameworks, DetectionResult, FrameworkDetector, PHPDetector, PythonDetector,
    TypeScriptDetector,
};
