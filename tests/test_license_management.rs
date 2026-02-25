use architect_linter_pro::license::{License, LicenseManager, Tier};

#[test]
fn test_license_tier_validation() {
    let pro_license = License {
        key: "pro-12345".to_string(),
        tier: Tier::Pro,
        expires: None,
    };

    assert_eq!(pro_license.tier, Tier::Pro);
}

#[test]
fn test_community_vs_pro_features() {
    // Community: no access
    assert!(LicenseManager::require_pro_feature("security-audit").is_err());

    // Pro: access allowed (in real scenario with valid license)
    // This test shows the API, actual license check requires file
}

#[test]
fn test_license_serialization() {
    let license = License {
        key: "pro-1234".to_string(),
        tier: Tier::Pro,
        expires: Some("2026-12-31".to_string()),
    };

    let json = serde_json::to_string(&license).unwrap();
    let deserialized: License = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.key, license.key);
    assert_eq!(deserialized.tier, license.tier);
    assert_eq!(deserialized.expires, license.expires);
}

#[test]
fn test_all_tier_types() {
    let community = License {
        key: "com-1".to_string(),
        tier: Tier::Community,
        expires: None,
    };
    let pro = License {
        key: "pro-1".to_string(),
        tier: Tier::Pro,
        expires: None,
    };
    let enterprise = License {
        key: "ent-1".to_string(),
        tier: Tier::Enterprise,
        expires: Some("2027-12-31".to_string()),
    };

    assert_eq!(community.tier, Tier::Community);
    assert_eq!(pro.tier, Tier::Pro);
    assert_eq!(enterprise.tier, Tier::Enterprise);
}

#[test]
fn test_license_path_generation() {
    let path = LicenseManager::get_license_path();
    let path_str = path.to_string_lossy();
    assert!(path_str.contains(".architect-license.json"));
}
