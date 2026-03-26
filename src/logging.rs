//! Logging module - Structured logging with tracing
//!
//! This module provides centralized logging configuration using the tracing crate.
//! It supports different log levels based on debug mode and provides structured
//! logging for better observability.

use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize the tracing subscriber with appropriate log level
///
/// # Arguments
/// * `debug_mode` - If true, enables DEBUG level logging. Otherwise uses INFO level.
///
/// # Examples
/// ```
/// use architect_linter_pro::logging;
/// logging::init(false); // INFO level
/// logging::init(true);  // DEBUG level
/// ```
pub fn init(debug_mode: bool) {
    let log_level = if debug_mode {
        Level::DEBUG
    } else {
        Level::INFO
    };

    // Create env filter with fallback to configured level
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level.to_string()));

    // Configure the subscriber
    let fmt_layer = fmt::layer()
        .with_target(debug_mode)
        .with_thread_ids(debug_mode)
        .with_line_number(debug_mode)
        .with_file(debug_mode)
        .with_ansi(false)
        .with_writer(std::io::stdout)
        .compact();

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    // Initialize the subscriber
    if let Err(e) = subscriber.try_init() {
        eprintln!("⚠️  Failed to initialize logging: {}", e);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_normal_mode() {
        // This test just ensures init doesn't panic
        // We can't test the actual logging output easily
        init(false);
    }

    #[test]
    fn test_init_debug_mode() {
        // This test just ensures init doesn't panic in debug mode
        init(true);
    }
}
