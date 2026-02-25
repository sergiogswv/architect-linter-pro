/// Architecture Recommendations Engine
/// Generates actionable recommendations based on project metrics and architecture score
pub struct ArchitectureRecommendations;

impl ArchitectureRecommendations {
    /// Generates recommendations based on project metrics
    ///
    /// # Arguments
    /// * `violation_count` - Number of architecture violations found
    /// * `total_files` - Total number of files in the project
    /// * `circular_deps` - Number of circular dependencies detected
    /// * `avg_method_length` - Average method/function length
    ///
    /// # Returns
    /// A vector of actionable recommendations as strings
    pub fn recommend_improvements(
        violation_count: usize,
        total_files: usize,
        circular_deps: usize,
        avg_method_length: usize,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Based on violation count
        if violation_count > 50 {
            recommendations.push(
                "🔴 High violation count. Refactor systematically from most violated layer."
                    .to_string(),
            );
        } else if violation_count > 20 {
            recommendations.push("🟡 Moderate violations. Address within next sprint.".to_string());
        }

        // Based on file count
        if total_files > 500 {
            recommendations.push(
                "📦 Large codebase. Consider splitting into packages/modules for better organization."
                    .to_string(),
            );
        }

        // Based on circular dependencies
        if circular_deps > 0 {
            recommendations.push(format!(
                "⚠️  {} circular dependencies found. Break with abstraction layers.",
                circular_deps
            ));
        }

        // Based on code metrics
        if avg_method_length > 50 {
            recommendations.push(
                "📏 Average method length is high. Extract smaller, focused methods."
                    .to_string(),
            );
        }

        recommendations
    }

    /// Generates a tiered action plan based on the current architecture score
    ///
    /// The action plan is tailored to the architecture health score:
    /// - Critical (< 30): Break down violations and establish foundations
    /// - Moderate (30-70): Address remaining issues and optimize
    /// - Good (> 70): Maintain and monitor
    ///
    /// # Arguments
    /// * `current_score` - The current architecture score (0-100)
    ///
    /// # Returns
    /// A vector of action items in order of priority
    pub fn generate_action_plan(current_score: f64) -> Vec<String> {
        let mut actions = Vec::new();

        if current_score < 30.0 {
            actions.push("1. Identify and fix critical violations first".to_string());
            actions.push("2. Establish clear layer boundaries".to_string());
            actions.push("3. Create architect.json with rules".to_string());
            actions.push("4. Refactor systematically".to_string());
        } else if current_score < 70.0 {
            actions.push("1. Address remaining violations".to_string());
            actions.push("2. Break circular dependencies".to_string());
            actions.push("3. Optimize hot paths".to_string());
        } else {
            actions.push("✅ Architecture is well-organized!".to_string());
            actions.push("Continue monitoring with architect lint in CI/CD.".to_string());
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_violations_recommendation() {
        let recs = ArchitectureRecommendations::recommend_improvements(60, 100, 0, 30);
        assert!(recs.iter().any(|r| r.contains("High violation")));
    }

    #[test]
    fn test_moderate_violations_recommendation() {
        let recs = ArchitectureRecommendations::recommend_improvements(30, 100, 0, 30);
        assert!(recs.iter().any(|r| r.contains("Moderate")));
    }

    #[test]
    fn test_large_codebase_recommendation() {
        let recs = ArchitectureRecommendations::recommend_improvements(10, 600, 0, 30);
        assert!(recs.iter().any(|r| r.contains("Large codebase")));
    }

    #[test]
    fn test_circular_deps_recommendation() {
        let recs = ArchitectureRecommendations::recommend_improvements(10, 100, 3, 30);
        assert!(recs.iter().any(|r| r.contains("circular")));
    }

    #[test]
    fn test_method_length_recommendation() {
        let recs = ArchitectureRecommendations::recommend_improvements(10, 100, 0, 60);
        assert!(recs.iter().any(|r| r.contains("method length")));
    }

    #[test]
    fn test_action_plan_critical_score() {
        let actions = ArchitectureRecommendations::generate_action_plan(20.0);
        assert!(actions.len() >= 4);
        assert!(actions[0].contains("critical"));
    }

    #[test]
    fn test_action_plan_moderate_score() {
        let actions = ArchitectureRecommendations::generate_action_plan(50.0);
        assert!(actions.len() >= 3);
        assert!(actions[0].contains("remaining"));
    }

    #[test]
    fn test_action_plan_good_score() {
        let actions = ArchitectureRecommendations::generate_action_plan(85.0);
        assert!(actions.iter().any(|a| a.contains("✅")));
    }
}
