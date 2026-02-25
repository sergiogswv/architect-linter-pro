//! License validation for pro features

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Tier {
    Community,
    Pro,
    Enterprise,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct License {
    pub key: String,
    pub tier: Tier,
    pub expires: Option<String>,
}

pub struct LicenseManager;

impl LicenseManager {
    /// Get license from ~/.architect-license.json
    pub fn get_license() -> Option<License> {
        let home = dirs::home_dir()?;
        let path = home.join(".architect-license.json");

        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(license) = serde_json::from_str::<License>(&content) {
                return Some(license);
            }
        }

        None
    }

    /// Check if Pro tier is available
    pub fn is_pro() -> bool {
        match Self::get_license() {
            Some(license) => matches!(license.tier, Tier::Pro | Tier::Enterprise),
            None => false,
        }
    }

    /// Validate Pro feature access
    pub fn require_pro_feature(feature: &str) -> Result<(), String> {
        if !Self::is_pro() {
            return Err(format!(
                "{} requires Pro license.\nGet started: https://architect-linter.dev/pro",
                feature
            ));
        }
        Ok(())
    }

    /// Get license path (useful for testing)
    pub fn get_license_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".architect-license.json")
        } else {
            PathBuf::from(".architect-license.json")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_parsing() {
        let json = r#"{"key": "pro-1234", "tier": "Pro", "expires": null}"#;
        let license: License = serde_json::from_str(json).unwrap();
        assert_eq!(license.tier, Tier::Pro);
    }

    #[test]
    fn test_tier_comparison() {
        let pro_license = License {
            key: "pro".to_string(),
            tier: Tier::Pro,
            expires: None,
        };
        assert_eq!(pro_license.tier, Tier::Pro);
    }

    #[test]
    fn test_license_tiers() {
        let community = License {
            key: "com-123".to_string(),
            tier: Tier::Community,
            expires: None,
        };
        let pro = License {
            key: "pro-123".to_string(),
            tier: Tier::Pro,
            expires: None,
        };
        let enterprise = License {
            key: "ent-123".to_string(),
            tier: Tier::Enterprise,
            expires: Some("2026-12-31".to_string()),
        };

        assert_eq!(community.tier, Tier::Community);
        assert_eq!(pro.tier, Tier::Pro);
        assert_eq!(enterprise.tier, Tier::Enterprise);
    }

    #[test]
    fn test_license_path() {
        let path = LicenseManager::get_license_path();
        assert!(path.to_string_lossy().contains(".architect-license.json"));
    }
}
