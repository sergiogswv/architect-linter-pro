use super::helpers::{base_config, rule};
use crate::config::ConfigFile;

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "mvc" => Some(base_config(
            "mvc",
            vec![
                rule("/routes/", "/models/", "error", "Routes must go through controllers, not access models directly"),
                rule("/models/", "/controllers/", "error", "Models must not depend on controllers"),
                rule("/middleware/", "/controllers/", "warning", "Middleware should not depend on specific controllers"),
            ],
        )),
        "hexagonal" => Some(base_config(
            "hexagonal",
            vec![
                rule("/domain/", "/infrastructure/", "error", "Domain must not depend on infrastructure"),
                rule("/domain/", "/adapters/", "error", "Domain must not depend on adapters"),
                rule("/application/", "/infrastructure/", "error", "Application must not depend on infrastructure directly"),
            ],
        )),
        "feature-based" => Some(base_config(
            "feature-based",
            vec![
                rule("/features/*/", "/features/*/", "warning", "Features should be independent from each other"),
                rule("/shared/", "/features/", "error", "Shared utilities must not depend on specific features"),
            ],
        )),
        _ => None,
    }
}
