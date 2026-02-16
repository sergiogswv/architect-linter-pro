/// End-to-end tests module
///
/// This module contains E2E tests that test the CLI as a black box

// Re-export common utilities
#[path = "../common/mod.rs"]
mod common;

// GitHub Action workflow execution tests
pub mod github_action;
