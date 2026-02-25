//! Feature gating for Community vs Pro versions

pub struct Features;

impl Features {
    /// Check if Pro features are enabled
    pub fn is_pro() -> bool {
        cfg!(feature = "pro")
    }

    /// Check if security audit is available
    pub fn is_security_audit_enabled() -> bool {
        cfg!(feature = "security-audit")
    }

    /// Assert that a Pro feature is available
    pub fn require_pro(feature: &str) -> Result<(), String> {
        if !Self::is_pro() {
            return Err(format!(
                "{} requires Pro license. Learn more: https://architect-linter.dev/pro",
                feature
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pro_features_gating() {
        #[cfg(feature = "pro")]
        assert!(Features::is_pro());

        #[cfg(not(feature = "pro"))]
        assert!(!Features::is_pro());
    }

    #[test]
    fn test_require_pro_feature() {
        let result = Features::require_pro("security-audit");
        #[cfg(feature = "pro")]
        assert!(result.is_ok());

        #[cfg(not(feature = "pro"))]
        assert!(result.is_err());
    }

    #[test]
    fn test_security_audit_feature() {
        #[cfg(feature = "security-audit")]
        assert!(Features::is_security_audit_enabled());

        #[cfg(not(feature = "security-audit"))]
        assert!(!Features::is_security_audit_enabled());
    }
}
