//! Unit tests for CLI module
//!
//! This test suite verifies CLI argument parsing and helper functions.

use architect_linter_pro::cli::{CliArgs, ReportFormat};

// ============================================================================
// Tests for ReportFormat
// ============================================================================

#[test]
fn test_report_format_from_str_json() {
    let format = ReportFormat::from_str("json");
    assert_eq!(format, Some(ReportFormat::Json));
}

#[test]
fn test_report_format_from_str_json_uppercase() {
    let format = ReportFormat::from_str("JSON");
    assert_eq!(format, Some(ReportFormat::Json));
}

#[test]
fn test_report_format_from_str_json_mixed_case() {
    let format = ReportFormat::from_str("Json");
    assert_eq!(format, Some(ReportFormat::Json));
}

#[test]
fn test_report_format_from_str_markdown() {
    let format = ReportFormat::from_str("markdown");
    assert_eq!(format, Some(ReportFormat::Markdown));
}

#[test]
fn test_report_format_from_str_md() {
    let format = ReportFormat::from_str("md");
    assert_eq!(format, Some(ReportFormat::Markdown));
}

#[test]
fn test_report_format_from_str_md_uppercase() {
    let format = ReportFormat::from_str("MD");
    assert_eq!(format, Some(ReportFormat::Markdown));
}

#[test]
fn test_report_format_from_str_markdown_uppercase() {
    let format = ReportFormat::from_str("MARKDOWN");
    assert_eq!(format, Some(ReportFormat::Markdown));
}

#[test]
fn test_report_format_from_str_invalid() {
    let format = ReportFormat::from_str("invalid");
    assert_eq!(format, None);
}

#[test]
fn test_report_format_from_str_empty() {
    let format = ReportFormat::from_str("");
    assert_eq!(format, None);
}

#[test]
fn test_report_format_from_str_partial_json() {
    let format = ReportFormat::from_str("jsonn");
    assert_eq!(format, None);
}

// ============================================================================
// Tests for CliArgs structure
// ============================================================================

#[test]
fn test_cli_args_default_values() {
    let args = CliArgs {
        project_path: None,
        watch_mode: false,
        fix_mode: false,
        staged_mode: false,
        incremental_mode: false,
        no_cache: false,
        daemon_mode: false,
        report_format: None,
        output_path: None,
        ..Default::default()
    };

    assert!(args.project_path.is_none());
    assert!(!args.watch_mode);
    assert!(!args.fix_mode);
    assert!(!args.staged_mode);
    assert!(!args.incremental_mode);
    assert!(!args.no_cache);
    assert!(!args.daemon_mode);
    assert!(args.report_format.is_none());
    assert!(args.output_path.is_none());
}

#[test]
fn test_cli_args_with_all_flags() {
    let args = CliArgs {
        project_path: Some("/path/to/project".to_string()),
        watch_mode: true,
        fix_mode: true,
        staged_mode: true,
        incremental_mode: true,
        no_cache: true,
        daemon_mode: true,
        report_format: Some(ReportFormat::Json),
        output_path: Some("/path/to/report.json".to_string()),
        ..Default::default()
    };

    assert_eq!(args.project_path, Some("/path/to/project".to_string()));
    assert!(args.watch_mode);
    assert!(args.fix_mode);
    assert!(args.staged_mode);
    assert!(args.incremental_mode);
    assert!(args.no_cache);
    assert!(args.daemon_mode);
    assert_eq!(args.report_format, Some(ReportFormat::Json));
    assert_eq!(args.output_path, Some("/path/to/report.json".to_string()));
}

#[test]
fn test_cli_args_with_watch_mode() {
    let args = CliArgs {
        project_path: Some(".".to_string()),
        watch_mode: true,
        fix_mode: false,
        staged_mode: false,
        incremental_mode: false,
        no_cache: false,
        daemon_mode: false,
        report_format: None,
        output_path: None,
        ..Default::default()
    };

    assert!(args.watch_mode);
    assert!(!args.fix_mode);
    assert!(!args.staged_mode);
}

#[test]
fn test_cli_args_with_staged_mode() {
    let args = CliArgs {
        project_path: Some(".".to_string()),
        watch_mode: false,
        fix_mode: false,
        staged_mode: true,
        incremental_mode: false,
        no_cache: false,
        daemon_mode: false,
        report_format: None,
        output_path: None,
        ..Default::default()
    };

    assert!(!args.watch_mode);
    assert!(!args.fix_mode);
    assert!(args.staged_mode);
}

#[test]
fn test_cli_args_with_incremental_mode() {
    let args = CliArgs {
        project_path: Some(".".to_string()),
        watch_mode: false,
        fix_mode: false,
        staged_mode: false,
        incremental_mode: true,
        no_cache: false,
        daemon_mode: false,
        report_format: None,
        output_path: None,
        ..Default::default()
    };

    assert!(!args.staged_mode);
    assert!(args.incremental_mode);
}

#[test]
fn test_cli_args_with_report_json() {
    let args = CliArgs {
        project_path: None,
        watch_mode: false,
        fix_mode: false,
        staged_mode: false,
        incremental_mode: false,
        no_cache: false,
        daemon_mode: false,
        report_format: Some(ReportFormat::Json),
        output_path: None,
        ..Default::default()
    };

    assert_eq!(args.report_format, Some(ReportFormat::Json));
}

#[test]
fn test_cli_args_with_report_markdown() {
    let args = CliArgs {
        project_path: None,
        watch_mode: false,
        fix_mode: false,
        staged_mode: false,
        incremental_mode: false,
        no_cache: false,
        daemon_mode: false,
        report_format: Some(ReportFormat::Markdown),
        output_path: None,
        ..Default::default()
    };

    assert_eq!(args.report_format, Some(ReportFormat::Markdown));
}

#[test]
fn test_cli_args_with_daemon_mode() {
    let args = CliArgs {
        project_path: Some(".".to_string()),
        watch_mode: false,
        fix_mode: false,
        staged_mode: false,
        incremental_mode: false,
        no_cache: false,
        daemon_mode: true,
        report_format: None,
        output_path: None,
        ..Default::default()
    };

    assert!(args.daemon_mode);
    assert!(!args.watch_mode);
}

#[test]
fn test_cli_args_with_no_cache() {
    let args = CliArgs {
        project_path: Some(".".to_string()),
        watch_mode: false,
        fix_mode: false,
        staged_mode: false,
        incremental_mode: false,
        no_cache: true,
        daemon_mode: false,
        report_format: None,
        output_path: None,
        ..Default::default()
    };

    assert!(args.no_cache);
}

#[test]
fn test_cli_args_with_output_path() {
    let args = CliArgs {
        project_path: None,
        watch_mode: false,
        fix_mode: false,
        staged_mode: false,
        incremental_mode: false,
        no_cache: false,
        daemon_mode: false,
        report_format: None,
        output_path: Some("report.md".to_string()),
        ..Default::default()
    };

    assert_eq!(args.output_path, Some("report.md".to_string()));
}

#[test]
fn test_report_format_equality() {
    assert_eq!(ReportFormat::Json, ReportFormat::Json);
    assert_eq!(ReportFormat::Markdown, ReportFormat::Markdown);
    assert_ne!(ReportFormat::Json, ReportFormat::Markdown);
}

#[test]
fn test_cli_args_clone() {
    let args1 = CliArgs {
        project_path: Some("test".to_string()),
        watch_mode: true,
        fix_mode: false,
        staged_mode: false,
        incremental_mode: false,
        no_cache: false,
        daemon_mode: false,
        report_format: Some(ReportFormat::Json),
        output_path: None,
        ..Default::default()
    };

    let args2 = args1.clone();

    assert_eq!(args1.project_path, args2.project_path);
    assert_eq!(args1.watch_mode, args2.watch_mode);
    assert_eq!(args1.report_format, args2.report_format);
}
