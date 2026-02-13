use crate::config::Framework;
use std::fs;
use std::path::Path;

/// Analiza los archivos de dependencias para determinar el framework del proyecto.
/// Soporta: JavaScript/TypeScript (package.json), Python (requirements.txt/pyproject.toml/Pipfile),
/// Go (go.mod), PHP (composer.json), Java (pom.xml/build.gradle)
pub fn detect_framework(root: &Path) -> Framework {
    // 1. Check JavaScript/TypeScript (package.json)
    let pkg_path = root.join("package.json");
    if let Ok(content) = fs::read_to_string(&pkg_path) {
        if content.contains("@nestjs/core") {
            return Framework::NestJS;
        }
        if content.contains("\"react\"") {
            return Framework::React;
        }
        if content.contains("@angular/core") {
            return Framework::Angular;
        }
        if content.contains("\"express\"") {
            return Framework::Express;
        }
    }

    // 2. Check Python (requirements.txt)
    let requirements_path = root.join("requirements.txt");
    if let Ok(content) = fs::read_to_string(&requirements_path) {
        let content_lower = content.to_lowercase();
        if content_lower.contains("django") {
            return Framework::Django;
        }
        if content_lower.contains("fastapi") {
            return Framework::FastAPI;
        }
        if content_lower.contains("flask") {
            return Framework::Flask;
        }
    }

    // 3. Check Python (pyproject.toml)
    let pyproject_path = root.join("pyproject.toml");
    if let Ok(content) = fs::read_to_string(&pyproject_path) {
        let content_lower = content.to_lowercase();
        if content_lower.contains("django") {
            return Framework::Django;
        }
        if content_lower.contains("fastapi") {
            return Framework::FastAPI;
        }
        if content_lower.contains("flask") {
            return Framework::Flask;
        }
    }

    // 4. Check Python (Pipfile)
    let pipfile_path = root.join("Pipfile");
    if let Ok(content) = fs::read_to_string(&pipfile_path) {
        let content_lower = content.to_lowercase();
        if content_lower.contains("django") {
            return Framework::Django;
        }
        if content_lower.contains("fastapi") {
            return Framework::FastAPI;
        }
        if content_lower.contains("flask") {
            return Framework::Flask;
        }
    }

    // 5. Check Go (go.mod)
    let go_mod_path = root.join("go.mod");
    if let Ok(content) = fs::read_to_string(&go_mod_path) {
        if content.contains("gin-gonic/gin") || content.contains("github.com/gin-gonic/gin") {
            return Framework::Gin;
        }
        if content.contains("labstack/echo") || content.contains("github.com/labstack/echo") {
            return Framework::Echo;
        }
    }

    // 6. Check PHP (composer.json)
    let composer_path = root.join("composer.json");
    if let Ok(content) = fs::read_to_string(&composer_path) {
        if content.contains("laravel/framework") {
            return Framework::Laravel;
        }
        if content.contains("symfony/framework-bundle")
            || content.contains("symfony/flex")
            || content.contains("\"symfony/")
        {
            return Framework::Symfony;
        }
    }

    // 7. Check Java (pom.xml - Maven)
    let pom_path = root.join("pom.xml");
    if let Ok(content) = fs::read_to_string(&pom_path) {
        if content.contains("spring-boot") || content.contains("org.springframework") {
            return Framework::Spring;
        }
    }

    // 8. Check Java (build.gradle - Gradle)
    let gradle_path = root.join("build.gradle");
    if !gradle_path.exists() {
        // Also check build.gradle.kts
        let _ = root.join("build.gradle.kts");
    }
    if let Ok(content) = fs::read_to_string(&gradle_path) {
        if content.contains("spring-boot") || content.contains("org.springframework") {
            return Framework::Spring;
        }
    }
    let gradle_kts_path = root.join("build.gradle.kts");
    if let Ok(content) = fs::read_to_string(&gradle_kts_path) {
        if content.contains("spring-boot") || content.contains("org.springframework") {
            return Framework::Spring;
        }
    }

    Framework::Unknown
}

/// Sugiere un límite de líneas de código (LOC) basado en el framework detectado.
#[allow(dead_code)]
pub fn get_loc_suggestion(framework: &Framework) -> usize {
    match framework {
        Framework::NestJS => 40,  // Métodos de clase
        Framework::React => 25,   // Componentes funcionales pequeños
        Framework::Angular => 50, // Componentes con lógica más extensa
        Framework::Express => 60, // Middlewares y handlers
        // Python frameworks
        Framework::Django => 50,   // Views, models
        Framework::Flask => 50,    // Route handlers
        Framework::FastAPI => 40,  // Endpoint functions
        // Go frameworks
        Framework::Gin => 40,      // Handler functions
        Framework::Echo => 40,     // Handler functions
        // Java frameworks
        Framework::Spring => 50,   // Service methods, controllers
        // PHP frameworks
        Framework::Laravel => 50,  // Controller methods
        Framework::Symfony => 50,  // Controller methods
        Framework::Unknown => 50,  // Estándar general
    }
}
