use super::helpers::{base_config, rule};
use crate::config::ConfigFile;

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "hexagonal" => Some(base_config(
            "hexagonal",
            vec![
                rule("/domain/", "/application/", "error", "Domain must not depend on application layer"),
                rule("/domain/", "/infrastructure/", "error", "Domain must not depend on infrastructure"),
                rule("/application/", "/infrastructure/", "error", "Application layer must not depend on infrastructure directly"),
            ],
        )),
        "clean" => Some(base_config(
            "clean",
            vec![
                rule("/entities/", "/use-cases/", "error", "Entities must not depend on use cases"),
                rule("/entities/", "/adapters/", "error", "Entities must not depend on adapters"),
                rule("/use-cases/", "/adapters/", "error", "Use cases must not depend on adapters"),
                rule("/use-cases/", "/frameworks/", "error", "Use cases must not depend on frameworks"),
            ],
        )),
        "layered" => Some(base_config(
            "layered",
            vec![
                rule("/controllers/", "/repositories/", "error", "Controllers must go through services, not access repositories directly"),
                rule("/repositories/", "/controllers/", "error", "Repositories must not depend on controllers"),
                rule("/repositories/", "/services/", "error", "Repositories must not depend on services"),
            ],
        )),
        _ => None,
    }
}
