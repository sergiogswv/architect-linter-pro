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
        if content.contains("\"next\"") {
            return Framework::NextJS;
        }
        if content.contains("\"react\"") {
            return Framework::React;
        }
        if content.contains("\"express\"") {
            return Framework::Express;
        }
        if content.contains("\"vue\"") {
            return Framework::Vue;
        }
        if content.contains("\"svelte\"") {
            return Framework::Svelte;
        }
        if content.contains("@remix-run") {
            return Framework::Remix;
        }
        if content.contains("\"solid-js\"") {
            return Framework::SolidJS;
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


    // 5. Check PHP (composer.json)
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

    // 6. Check Java (pom.xml - Maven) - Removed, Java no longer supported

    // 7. Check Java (build.gradle - Gradle) - Removed, Java no longer supported

    Framework::Unknown
}

/// Sugiere un límite de líneas de código (LOC) basado en el framework detectado.
#[allow(dead_code)]
pub fn get_loc_suggestion(framework: &Framework) -> usize {
    match framework {
        // TypeScript/JavaScript frameworks
        Framework::NestJS => 40,    // Métodos de clase
        Framework::Express => 60,   // Middlewares y handlers
        Framework::React => 25,     // Componentes funcionales pequeños
        Framework::NextJS => 30,    // API routes and components
        Framework::Vue => 30,       // Components
        Framework::Svelte => 30,    // Components
        Framework::Remix => 35,     // Route handlers
        Framework::SolidJS => 25,   // Components
        // Python frameworks
        Framework::Django => 50,    // Views, models
        Framework::Flask => 50,     // Route handlers
        Framework::FastAPI => 40,   // Endpoint functions
        // PHP frameworks
        Framework::Laravel => 50,   // Controller methods
        Framework::Symfony => 50,   // Controller methods
        Framework::Unknown => 50,   // Estándar general
    }
}
/// Sugiere el comando de build basado en el framework detectado.
pub fn get_build_command_suggestion(framework: &Framework) -> Option<String> {
    match framework {
        Framework::NestJS => Some("npm run build".to_string()),
        Framework::React | Framework::NextJS | Framework::Vue | Framework::Svelte | Framework::Remix | Framework::SolidJS => Some("npm run build".to_string()),
        Framework::Express => Some("npm run build".to_string()),
        _ => None,
    }
}
