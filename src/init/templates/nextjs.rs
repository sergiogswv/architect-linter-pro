use super::helpers::{base_config, rule};
use crate::config::ConfigFile;

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "feature-based" => Some(base_config(
            "feature-based",
            vec![
                rule("/features/", "/app/", "error", "Features must not import from the app layer"),
                rule("/features/*/", "/features/*/", "warning", "Features should be independent from each other"),
                rule("/components/", "/features/", "error", "Shared components must not depend on specific features"),
            ],
        )),
        "layered" => Some(base_config(
            "layered",
            vec![
                rule("/components/", "/lib/server/", "error", "Client components must not import server-only lib"),
                rule("/pages/", "/components/ui/", "warning", "Pages should use feature components, not raw UI primitives"),
            ],
        )),
        _ => None,
    }
}
