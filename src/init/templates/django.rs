use super::helpers::{base_config, rule};
use crate::config::ConfigFile;

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "mvt" => Some(base_config(
            "mvt",
            vec![
                rule("/templates/", "/models/", "error", "Templates must not import models directly"),
                rule("/views/", "/urls/", "warning", "Views should not import URL configuration"),
            ],
        )),
        "service-layer" => Some(base_config(
            "service-layer",
            vec![
                rule("/views/", "/models/", "warning", "Views should go through services, not access models directly"),
                rule("/services/", "/views/", "error", "Services must not depend on views"),
                rule("/repositories/", "/services/", "error", "Repositories must not depend on services"),
            ],
        )),
        _ => None,
    }
}
