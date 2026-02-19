//! Pattern matching utilities for import validation

/// Normaliza un patrón glob para hacer matching simple
/// Ejemplos:
/// - "src/components/**" → "src/components/"
/// - "**/*.tsx" → ".tsx"
/// - "src/services/**" → "src/services/"
pub fn normalize_pattern(pattern: &str) -> String {
    let normalized = pattern
        .to_lowercase()
        .replace("\\", "/")  // Normalizar separadores de Windows
        .replace("**", "*")   // Unificar comodines
        .replace("//", "/");

    // Limpiar duplicados de / resultantes de la normalización
    normalized.replace("//", "/")
}

/// Verifica si un path coincide con un patrón normalizado
pub fn matches_pattern(path: &str, pattern: &str) -> bool {
    let normalized_path = path.to_lowercase().replace("\\", "/");
    let raw_pattern = pattern.to_lowercase().replace("\\", "/");

    // Si el patrón tiene *, hacemos un split básico
    // Ejemplo: "src/modules/*/controllers/*" -> partes: ["src/modules/", "/controllers/"]
    let parts: Vec<&str> = raw_pattern.split('*').filter(|s| !s.is_empty()).collect();

    if parts.is_empty() {
        return normalized_path.contains(&raw_pattern);
    }

    // Todas las partes deben estar presentes en el orden correcto
    let mut last_pos = 0;
    for part in parts {
        if let Some(pos) = normalized_path[last_pos..].find(part) {
            last_pos += pos + part.len();
        } else {
            // Intentar también sin el prefijo "src/" para imports relativos/alias
            if part.starts_with("src/") {
                let folder_part = &part[4..];
                if let Some(pos) = normalized_path.find(folder_part) {
                    last_pos = pos + folder_part.len();
                    continue;
                }
            }
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_pattern() {
        assert_eq!(normalize_pattern("src/components/**"), "src/components/*");
        assert_eq!(normalize_pattern("**/*.tsx"), "*/*.tsx"); // Note: keeps leading wildcard
        assert_eq!(normalize_pattern("src/services/**"), "src/services/*");
    }

    #[test]
    fn test_matches_pattern_basic() {
        assert!(matches_pattern(
            "src/components/button.tsx",
            "src/components/"
        ));
        assert!(!matches_pattern("src/services/api.ts", "src/components/"));
    }

    #[test]
    fn test_matches_pattern_with_relative_imports() {
        assert!(matches_pattern("../services/userservice", "src/services/"));
        assert!(matches_pattern("@/services/api", "src/services/"));
    }
}
