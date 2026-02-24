//! The `init` subcommand — generates architect.json from a framework template.

pub mod prompts;
pub mod templates;

use crate::config::ConfigFile;
use crate::config::Framework;
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::Path;

/// Check that architect.json does not exist at `root`, unless `force` is true.
/// Returns Ok(()) if it's safe to proceed, Err if the user should abort.
pub fn check_no_existing_config(root: &Path, force: bool) -> Result<()> {
    let config_path = root.join("architect.json");
    if config_path.exists() && !force {
        return Err(miette::miette!(
            "Ya existe architect.json en {}.\n   Usa --force para sobreescribir.",
            root.display()
        ));
    }
    Ok(())
}

/// Serialize a ConfigFile and write it to `root/architect.json`.
pub fn write_config(root: &Path, config: &ConfigFile) -> Result<()> {
    let json = serde_json::to_string_pretty(config).into_diagnostic()?;
    let config_path = root.join("architect.json");
    fs::write(&config_path, json).into_diagnostic()?;
    Ok(())
}

/// Main entry point for the `init` subcommand.
///
/// Flow:
/// 1. Check if architect.json exists (respect --force)
/// 2. Detect framework from project files
/// 3. If not detected, ask the user
/// 4. Show pattern options and ask which to use
/// 5. Get template for (framework, pattern)
/// 6. Show preview and ask for confirmation
/// 7. Write architect.json
pub fn run_init(root: &Path, force: bool) -> Result<()> {
    // Step 1: Check for existing config
    check_no_existing_config(root, force)?;

    // Step 2: Detect framework
    let detected = crate::detector::detect_framework(root);
    let framework = if detected == Framework::Unknown {
        println!("No se detectó ningún framework conocido.");
        match prompts::ask_framework() {
            Some(fw) => {
                println!("Framework seleccionado: {}", fw.as_str());
                fw
            }
            None => {
                println!("Operación cancelada.");
                return Ok(());
            }
        }
    } else {
        println!("Detectado: {}", detected.as_str());
        detected
    };

    // Step 3: Get pattern options and ask
    let options = templates::patterns_for_framework(&framework);
    let pattern_key = match prompts::ask_pattern(&framework, &options) {
        Some(p) => p,
        None => {
            println!("Operación cancelada.");
            return Ok(());
        }
    };

    // Step 4: Get the template
    let config = match templates::get_template(&framework, &pattern_key) {
        Some(c) => c,
        None => {
            return Err(miette::miette!(
                "No hay template para {} / {}. Por favor reporta esto como un bug.",
                framework.as_str(),
                pattern_key
            ));
        }
    };

    // Step 5: Show preview
    let json = serde_json::to_string_pretty(&config).into_diagnostic()?;
    prompts::show_preview(&json);

    // Step 6: Confirm
    if !prompts::confirm_write() {
        println!("Operación cancelada. No se creó architect.json.");
        return Ok(());
    }

    // Step 7: Write
    write_config(root, &config)?;

    println!();
    println!("architect.json creado en {}", root.display());
    println!("   Ejecuta `architect-linter-pro .` para analizar tu proyecto.");

    Ok(())
}
