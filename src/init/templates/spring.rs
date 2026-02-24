use super::helpers::{base_config, rule};
use crate::config::ConfigFile;

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "layered" => Some(base_config(
            "layered",
            vec![
                rule("/controller/", "/repository/", "error", "Controllers must go through services, not access repositories directly"),
                rule("/repository/", "/controller/", "error", "Repositories must not depend on controllers"),
                rule("/repository/", "/service/", "error", "Repositories must not depend on services"),
            ],
        )),
        "hexagonal" => Some(base_config(
            "hexagonal",
            vec![
                rule("/domain/", "/infrastructure/", "error", "Domain must not depend on infrastructure"),
                rule("/domain/", "/application/", "error", "Domain must not depend on application layer"),
                rule("/application/", "/infrastructure/", "error", "Application must not depend on infrastructure directly"),
            ],
        )),
        _ => None,
    }
}
