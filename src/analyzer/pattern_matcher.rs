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
        .replace("**", "")   // Quitar comodines globales
        .replace("*", ""); // Quitar comodines simples

    // Si el patrón termina en /, dejarlo; si no, mantenerlo como está
    normalized
}

/// Verifica si un path coincide con un patrón normalizado
/// Usa matching flexible para soportar diferentes formatos de import
pub fn matches_pattern(path: &str, pattern: &str) -> bool {
    let normalized_path = path.to_lowercase().replace("\\", "/");
    let normalized_pattern = pattern.to_lowercase();

    // Si el patrón está vacío después de normalización, no matchear nada
    if normalized_pattern.is_empty() {
        return false;
    }

    // Matching flexible para rutas absolutas y relativas
    // Ejemplos:
    // - Path: "c:/proyecto/src/components/button.jsx" con Pattern: "src/components/"
    // - Import: "../services/userservice" con Pattern: "src/services/"
    //   → Extraer "services/" del pattern y buscar "/services/" o "../services/" en el import
    // - Import: "@/api/posts" con Pattern: "src/api/"
    //   → Buscar "/api/" en el import

    if normalized_path.contains(&normalized_pattern) {
        return true;
    }

    // Para imports: si el patrón contiene "src/", extraer solo la carpeta después de src/
    // Ejemplo: "src/services/" → buscar también "/services/" o "services/"
    if normalized_pattern.contains("src/") {
        // Extraer la parte después de "src/"
        if let Some(folder_part) = normalized_pattern.strip_prefix("src/") {
            // Buscar "/folder/" o "../folder/" en el path (para imports relativos)
            let with_slash = format!("/{}", folder_part);
            let with_relative = format!("../{}", folder_part);
            let with_at = format!("@/{}", folder_part); // Para alias como @/services

            if normalized_path.contains(&with_slash)
                || normalized_path.contains(&with_relative)
                || normalized_path.contains(&with_at)
                || normalized_path.contains(folder_part)
            {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_pattern() {
        assert_eq!(normalize_pattern("src/components/**"), "src/components/");
        assert_eq!(normalize_pattern("**/*.tsx"), "/.tsx"); // Note: keeps leading /
        assert_eq!(normalize_pattern("src/services/**"), "src/services/");
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
