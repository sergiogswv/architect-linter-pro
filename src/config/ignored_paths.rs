use crate::config::Framework;

/// Valores por defecto para ignored_paths
pub fn default_ignored_paths() -> Vec<String> {
    vec![
        "node_modules/".to_string(),
        "dist/".to_string(),
        "build/".to_string(),
        ".git/".to_string(),
        "coverage/".to_string(),
        ".next/".to_string(),
        "out/".to_string(),
        ".nuxt/".to_string(),
        ".output/".to_string(),
        ".vite/".to_string(),            // Vite cache
        ".turbo/".to_string(),           // Turborepo cache
        ".parcel-cache/".to_string(),    // Parcel cache
        ".cache/".to_string(),           // Generic cache
        ".architect-cache/".to_string(), // Architect Linter cache
        "target/".to_string(),           // Rust
        "__pycache__/".to_string(),      // Python
        ".vscode/".to_string(),          // VSCode
        ".idea/".to_string(),            // IntelliJ
        ".claude/".to_string(),          // Claude AI assistant
        "venv/".to_string(),             // Python venv
        ".venv/".to_string(),            // Python venv
        "vendor/".to_string(),           // PHP/Go vendor
        ".gradle/".to_string(),          // Gradle
    ]
}

/// Obtiene los patrones de exclusión según el framework detectado
pub fn get_framework_ignored_paths(framework: &Framework) -> Vec<String> {
    let mut paths = vec![
        "node_modules/".to_string(),
        ".git/".to_string(),
        "coverage/".to_string(),
        ".vscode/".to_string(),
        ".idea/".to_string(),
    ];

    match framework {
        // TypeScript/JavaScript Frameworks
        Framework::NestJS | Framework::Express => {
            paths.extend(vec!["dist/".to_string(), "build/".to_string()]);
        }
        Framework::React | Framework::NextJS | Framework::Vue | Framework::Svelte | Framework::Remix | Framework::SolidJS => {
            paths.extend(vec![
                "build/".to_string(),
                "dist/".to_string(),
                ".next/".to_string(),        // Next.js
                "out/".to_string(),
                ".vite/".to_string(),        // Vite
                ".turbo/".to_string(),       // Turborepo
                ".parcel-cache/".to_string(), // Parcel
            ]);
        }
        // Python frameworks
        Framework::Django | Framework::Flask | Framework::FastAPI => {
            paths.extend(vec![
                "venv/".to_string(),
                ".venv/".to_string(),
                "env/".to_string(),
                ".env/".to_string(),
                "__pycache__/".to_string(),
                "*.pyc".to_string(),
                ".pytest_cache/".to_string(),
                ".mypy_cache/".to_string(),
                "htmlcov/".to_string(),
                "*.egg-info/".to_string(),
            ]);
        }
        // PHP frameworks
        Framework::Laravel | Framework::Symfony => {
            paths.extend(vec![
                "vendor/".to_string(),
                "storage/".to_string(),
                "bootstrap/cache/".to_string(),
                "*.log".to_string(),
            ]);
        }
        Framework::Unknown => {
            paths.extend(vec![
                "dist/".to_string(),
                "build/".to_string(),
                "out/".to_string(),
                ".vite/".to_string(),
                ".cache/".to_string(),
            ]);
        }
    }

    paths
}
