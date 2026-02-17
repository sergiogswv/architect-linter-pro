use serde_json::Value;

/// Maneja la migración de archivos de configuración antiguos al formato actual.
/// Por ahora, el formato es estable, pero esta herramienta está preparada
/// para futuras actualizaciones de esquema.
pub fn migrate_config(mut config: Value) -> Value {
    let mut modified = false;

    // Ejemplo: Si en el futuro cambiamos 'max_lines_per_function' a 'max_lines'
    // if let Some(old_val) = config.get_mut("max_lines_per_function") {
    //     // ... lógica de migración
    // }

    // Asegurar que ignored_paths existe si no estaba
    if config.get("ignored_paths").is_none() {
        config["ignored_paths"] = serde_json::json!([
            "node_modules/",
            ".git/",
            "dist/",
            "build/",
            "target/",
            "coverage/"
        ]);
        modified = true;
    }

    if modified {
        eprintln!("ℹ️  La configuración ha sido migrada automáticamente al formato más reciente.");
    }

    config
}
