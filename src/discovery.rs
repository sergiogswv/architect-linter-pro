use crate::config::default_ignored_paths;
use crate::detector;
use crate::parsers;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(Serialize)]
pub struct ProjectContext {
    pub framework: String,
    pub dependencies: Vec<String>,
    pub folder_structure: Vec<String>,
    pub key_files: Vec<String>,
}

/// Recolecta todos los archivos soportados que el linter debe analizar.
/// Incluye: TypeScript (.ts, .tsx), JavaScript (.js, .jsx), Python (.py), Go (.go), PHP (.php), Java (.java)
/// Respeta los patrones de exclusión definidos en ignored_paths.
pub fn collect_files(root: &Path, ignored_paths: &[String]) -> Vec<PathBuf> {
    let supported_exts = parsers::supported_extensions();

    WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| is_not_ignored_with_patterns(e, root, ignored_paths))
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path_str = e.path().to_string_lossy();

            // Ignorar archivos de definición TypeScript
            if path_str.ends_with(".d.ts") {
                return false;
            }

            // Verificar si la extensión está en la lista de soportadas
            e.path().extension().map_or(false, |ext| {
                supported_exts.iter().any(|&supported| ext == supported)
            })
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Genera un resumen completo del proyecto para que la IA tome decisiones arquitectónicas.
pub fn get_architecture_snapshot(root: &Path) -> ProjectContext {
    let mut folders = Vec::new();
    let mut key_files = Vec::new();

    let framework_enum = detector::detect_framework(root);
    let framework = framework_enum.as_str().to_string();
    let dependencies = get_dependency_list(root);

    let src_path = root.join("src");
    let scan_path = if src_path.exists() {
        src_path
    } else {
        root.to_path_buf()
    };

    // Use default_ignored_paths instead of hardcoded filter
    let ignored = default_ignored_paths();

    let walker = WalkDir::new(scan_path)
        .into_iter()
        .filter_entry(|e| is_not_ignored_with_patterns(e, root, &ignored));

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative_path = path.strip_prefix(root).unwrap_or(path);
        let path_str = relative_path.to_string_lossy().into_owned();

        if path.is_dir() {
            if path_str != "." && !path_str.is_empty() {
                folders.push(path_str);
            }
        } else if is_architectural_file(path) {
            key_files.push(path_str);
        }
    }

    ProjectContext {
        framework,
        dependencies,
        folder_structure: folders,
        key_files,
    }
}

fn get_dependency_list(root: &Path) -> Vec<String> {
    let mut deps_list = Vec::new();

    // 1. JavaScript/TypeScript: package.json
    let pkg_path = root.join("package.json");
    if let Ok(content) = fs::read_to_string(&pkg_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(deps) = json.get("dependencies").and_then(|d| d.as_object()) {
                deps_list.extend(deps.keys().cloned());
            }
            if let Some(dev_deps) = json.get("devDependencies").and_then(|d| d.as_object()) {
                deps_list.extend(dev_deps.keys().cloned());
            }
        }
    }

    // 2. Python: requirements.txt
    let requirements_path = root.join("requirements.txt");
    if let Ok(content) = fs::read_to_string(&requirements_path) {
        for line in content.lines() {
            let line = line.trim();
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // Extract package name (strip version specifiers like ==, >=, <=, ~=, !=, etc.)
            let pkg_name = line
                .split(&['=', '<', '>', '~', '!', ';'][..])
                .next()
                .unwrap_or(line)
                .trim()
                .to_string();
            if !pkg_name.is_empty() {
                deps_list.push(pkg_name);
            }
        }
    }

    // 3. Python: pyproject.toml (simple line-by-line parsing)
    let pyproject_path = root.join("pyproject.toml");
    if let Ok(content) = fs::read_to_string(&pyproject_path) {
        let mut in_dependencies = false;
        let mut bracket_count = 0;
        for line in content.lines() {
            let trimmed = line.trim();

            // Detect dependencies section start
            if trimmed.starts_with("dependencies") && trimmed.contains('[') {
                in_dependencies = true;
                bracket_count = trimmed.matches('[').count() as i32 - trimmed.matches(']').count() as i32;
                continue;
            }

            if in_dependencies {
                // Track brackets for multi-line arrays
                bracket_count += trimmed.matches('[').count() as i32;
                bracket_count -= trimmed.matches(']').count() as i32;

                if bracket_count <= 0 && trimmed.ends_with(']') {
                    in_dependencies = false;
                    continue;
                }

                // Extract quoted package names
                if trimmed.starts_with('"') || trimmed.starts_with("'") {
                    let pkg = trimmed
                        .trim_start_matches('"')
                        .trim_start_matches('\'')
                        .split(&['"', '\'', ' ', '>', '<', '=', '~', '^'][..])
                        .next()
                        .unwrap_or("")
                        .to_string();
                    if !pkg.is_empty() {
                        deps_list.push(pkg);
                    }
                }
            }
        }
    }

    // 4. Go: go.mod
    let go_mod_path = root.join("go.mod");
    if let Ok(content) = fs::read_to_string(&go_mod_path) {
        let mut in_require = false;
        for line in content.lines() {
            let trimmed = line.trim();

            // Detect require block
            if trimmed == "require (" {
                in_require = true;
                continue;
            }
            if in_require && trimmed == ")" {
                in_require = false;
                continue;
            }

            // Parse require entries
            if in_require && !trimmed.is_empty() && !trimmed.starts_with("//") {
                // Format: module/path version
                if let Some(module) = trimmed.split_whitespace().next() {
                    deps_list.push(module.to_string());
                }
            }

            // Single-line require: require module/path version
            if trimmed.starts_with("require ") && !trimmed.starts_with("require (") {
                if let Some(rest) = trimmed.strip_prefix("require ") {
                    if let Some(module) = rest.split_whitespace().next() {
                        deps_list.push(module.to_string());
                    }
                }
            }
        }
    }

    // 5. PHP: composer.json
    let composer_path = root.join("composer.json");
    if let Ok(content) = fs::read_to_string(&composer_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(deps) = json.get("require").and_then(|d| d.as_object()) {
                deps_list.extend(deps.keys().cloned());
            }
            if let Some(dev_deps) = json.get("require-dev").and_then(|d| d.as_object()) {
                deps_list.extend(dev_deps.keys().cloned());
            }
        }
    }

    // 6. Java: pom.xml (Maven) - simple line scan for artifactId
    let pom_path = root.join("pom.xml");
    if let Ok(content) = fs::read_to_string(&pom_path) {
        let mut in_dependencies = false;
        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed == "<dependencies>" {
                in_dependencies = true;
                continue;
            }
            if trimmed == "</dependencies>" {
                in_dependencies = false;
                continue;
            }

            if in_dependencies && trimmed.starts_with("<artifactId>") && trimmed.ends_with("</artifactId>") {
                let artifact = trimmed
                    .trim_start_matches("<artifactId>")
                    .trim_end_matches("</artifactId>");
                if !artifact.is_empty() {
                    deps_list.push(artifact.to_string());
                }
            }
        }
    }

    // 7. Java: build.gradle (Gradle) - simple line scan
    let gradle_path = root.join("build.gradle");
    if let Ok(content) = fs::read_to_string(&gradle_path) {
        for line in content.lines() {
            let trimmed = line.trim();

            // Match: implementation 'group:artifact:version' or implementation "group:artifact:version"
            if trimmed.starts_with("implementation ") || trimmed.starts_with("api ") {
                let quote_char = if trimmed.contains('\'') { '\'' } else { '"' };
                if let Some(start) = trimmed.find(quote_char) {
                    if let Some(end) = trimmed.rfind(quote_char) {
                        if start < end {
                            let dep = &trimmed[start + 1..end];
                            // Extract artifact name (last part before version)
                            if let Some(artifact) = dep.split(':').nth(1) {
                                deps_list.push(artifact.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // Also check build.gradle.kts
    let gradle_kts_path = root.join("build.gradle.kts");
    if let Ok(content) = fs::read_to_string(&gradle_kts_path) {
        for line in content.lines() {
            let trimmed = line.trim();

            // Match: implementation("group:artifact:version")
            if trimmed.starts_with("implementation(") || trimmed.starts_with("api(") {
                if let Some(start) = trimmed.find('"') {
                    if let Some(end) = trimmed.rfind('"') {
                        if start < end {
                            let dep = &trimmed[start + 1..end];
                            if let Some(artifact) = dep.split(':').nth(1) {
                                deps_list.push(artifact.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    deps_list
}

/// Verifica si una entrada debe ser ignorada según los patrones configurados
pub fn is_not_ignored_with_patterns(entry: &DirEntry, root: &Path, ignored_paths: &[String]) -> bool {
    // Obtener la ruta relativa al root del proyecto
    let entry_path = entry.path();
    let relative_path = entry_path
        .strip_prefix(root)
        .unwrap_or(entry_path)
        .to_string_lossy()
        .replace('\\', "/"); // Normalizar separadores para Windows

    // Verificar si la ruta coincide con algún patrón de exclusión
    for pattern in ignored_paths {
        let normalized_pattern = pattern.replace('\\', "/");

        // Coincidencia exacta o si la ruta comienza con el patrón
        if relative_path == normalized_pattern.trim_end_matches('/')
            || relative_path.starts_with(&normalized_pattern)
            || relative_path.starts_with(&format!("{}/", normalized_pattern.trim_end_matches('/')))
        {
            return false;
        }

        // También verificar el nombre del directorio/archivo directamente
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name == normalized_pattern.trim_end_matches('/')
                || format!("{}/", file_name) == normalized_pattern
            {
                return false;
            }
        }
    }

    true
}

fn is_architectural_file(path: &Path) -> bool {
    let s = path.to_string_lossy().to_lowercase();

    // TypeScript/JavaScript patterns
    let ts_js_patterns = [
        ".controller.ts",
        ".controller.js",
        ".service.ts",
        ".service.js",
        ".entity.ts",
        ".entity.js",
        ".repository.ts",
        ".repository.js",
        ".dto.ts",
        ".dto.js",
        ".module.ts",
        ".module.js",
        ".handler.ts",
        ".handler.js",
    ];

    // Python patterns
    let py_patterns = [
        "_controller.py",
        "_service.py",
        "_repository.py",
        "_model.py",
        "_view.py",
        "_handler.py",
    ];

    // Go patterns
    let go_patterns = [
        "_controller.go",
        "_service.go",
        "_repository.go",
        "_handler.go",
        "_model.go",
    ];

    // PHP patterns
    let php_patterns = [
        "controller.php",
        "service.php",
        "repository.php",
        "model.php",
        "handler.php",
        "entity.php",
    ];

    // Java patterns
    let java_patterns = [
        "controller.java",
        "service.java",
        "repository.java",
        "model.java",
        "handler.java",
        "entity.java",
        "dao.java",
    ];

    ts_js_patterns.iter().any(|&pattern| s.ends_with(pattern))
        || py_patterns.iter().any(|&pattern| s.ends_with(pattern))
        || go_patterns.iter().any(|&pattern| s.ends_with(pattern))
        || php_patterns.iter().any(|&pattern| s.ends_with(pattern))
        || java_patterns.iter().any(|&pattern| s.ends_with(pattern))
}
