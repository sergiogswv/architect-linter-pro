use crate::ai::SuggestedRule;
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::Path;
use std::sync::Arc;

use super::husky::setup_husky_pre_commit;
use super::ignored_paths::get_framework_ignored_paths;
use super::loader::{AIConfigFile, ConfigFile};
use super::types::{AIConfig, ArchPattern, ForbiddenRule, LinterContext};

/// Orquestador de configuración: Carga silenciosa o Wizard con IA
pub fn setup_or_load_config(root: &Path) -> Result<Arc<LinterContext>> {
    let config_path = root.join("architect.json");

    if config_path.exists() {
        // MODO AUTOMÁTICO: carga silenciosa
        let ctx = super::loader::load_config(root)?;
        return Ok(Arc::new(ctx));
    }

    // MODO CONFIGURACIÓN (IA Discovery)
    println!("📝 No encontré 'architect.json'. Iniciando descubrimiento asistido por IA...\n");

    // 0. Pedir configuración de IA si no existe
    let ai_configs = crate::ui::ask_ai_configs()?;

    // Seleccionar cuál usar para el descubrimiento inicial
    let ai_config = if ai_configs.len() > 1 {
        let names: Vec<String> = ai_configs.iter().map(|c| c.name.clone()).collect();
        let selection = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("¿Qué modelo deseas usar para el descubrimiento inicial?")
            .items(&names)
            .default(0)
            .interact()
            .into_diagnostic()?;
        ai_configs[selection].clone()
    } else {
        ai_configs[0].clone()
    };

    // 1. Discovery (Input local)
    let project_info = crate::discovery::get_architecture_snapshot(root);

    // 2. IA (Procesamiento inteligente)
    let runtime = tokio::runtime::Runtime::new().into_diagnostic()?;

    // a. Obtener Top 3 arquitecturas sugeridas
    println!("🔍 Analizando framework y sugiriendo mejores prácticas...");
    let top_3 = runtime
        .block_on(crate::ai::sugerir_top_3_arquitecturas(
            &project_info.framework,
            ai_configs.clone(),
        ))
        .map_err(|e| miette::miette!("Error consultando Top 3 arquitecturas: {}", e))?;

    // b. Seleccionar modo: Uno del Top 3 o Análisis Automático
    let choice = crate::ui::ask_for_architecture_choice(&top_3)?;

    let suggestions = match choice {
        Some(index) => {
            let selected_pattern = &top_3[index].name;
            println!(
                "🚀 Generando reglas para el patrón: {}...",
                selected_pattern
            );
            runtime
                .block_on(crate::ai::sugerir_reglas_para_patron(
                    selected_pattern,
                    project_info,
                    ai_configs.clone(),
                ))
                .map_err(|e| miette::miette!("Error sugiriendo reglas para el patrón: {}", e))?
        }
        None => {
            println!("🔍 Realizando análisis profundo de la estructura actual...");
            runtime
                .block_on(crate::ai::sugerir_arquitectura_inicial(
                    project_info,
                    ai_configs.clone(),
                ))
                .map_err(|e| miette::miette!("Error consultando la IA: {}", e))?
        }
    };

    // 3. UI (Wizard de confirmación)
    let (selected_rules, max_lines) = crate::ui::ask_user_to_confirm_rules(suggestions.clone())?;

    // 4. Config (Persistencia)
    let final_ctx = save_config_from_wizard(
        root,
        suggestions.pattern,
        selected_rules,
        max_lines,
        ai_configs,
        ai_config.name.clone(),
    )?;

    println!("✅ Configuración guardada exitosamente.\n");
    Ok(Arc::new(final_ctx))
}

/// PERSISTENCIA: Guarda las reglas de la IA y devuelve el contexto nuevo
pub fn save_config_from_wizard(
    root: &Path,
    pattern_name: String,
    rules: Vec<SuggestedRule>,
    max_lines: usize,
    ai_configs: Vec<AIConfig>,
    selected_name: String,
) -> Result<LinterContext> {
    // 1. Guardar architect.json (reglas - compartible en el repo)
    let config_path = root.join("architect.json");

    // Convertimos de SuggestedRule (IA) a ForbiddenRule (Linter)
    let forbidden_imports: Vec<ForbiddenRule> = rules
        .into_iter()
        .map(|r| ForbiddenRule {
            from: r.from,
            to: r.to,
            severity: None,
            reason: None,
        })
        .collect();

    let framework = crate::detector::detect_framework(root);

    // Obtener build_command sugerido
    let suggested_build_command = crate::detector::get_build_command_suggestion(&framework);

    // Obtener ignored_paths según el framework
    let ignored_paths = get_framework_ignored_paths(&framework);

    // Mapear nombre de patrón a Enum si es posible
    let architecture_pattern = match pattern_name.to_lowercase().as_str() {
        p if p.contains("hexagonal") => ArchPattern::Hexagonal,
        p if p.contains("clean") => ArchPattern::Clean,
        p if p.contains("mvc") => ArchPattern::MVC,
        _ => ArchPattern::Custom(pattern_name),
    };

    // Valores por defecto para el primer architect.json
    let config = ConfigFile {
        max_lines_per_function: max_lines,
        architecture_pattern,
        forbidden_imports: forbidden_imports.clone(),
        ignored_paths: ignored_paths.clone(),
        build_command: suggested_build_command,
        ai_fix_retries: 3,
    };

    let json = serde_json::to_string_pretty(&config).into_diagnostic()?;
    fs::write(&config_path, json).into_diagnostic()?;

    // 2. Guardar .architect.ai.json (config de IA - PRIVADO, en .gitignore)
    if !ai_configs.is_empty() {
        let ai_config_path = root.join(".architect.ai.json");
        let ai_config_file = AIConfigFile {
            configs: ai_configs.clone(),
            selected_name: selected_name.clone(),
        };
        let ai_json = serde_json::to_string_pretty(&ai_config_file).into_diagnostic()?;
        fs::write(&ai_config_path, ai_json).into_diagnostic()?;

        println!("🔐 Configuraciones de IA guardadas en: .architect.ai.json");
        println!("⚠️  Este archivo contiene API keys y NO debe ser compartido en el repositorio.");

        // Actualizar .gitignore automáticamente
        update_gitignore(root)?;

        println!();
    }

    // Instalar husky y pre-commit hook después de guardar la configuración
    setup_husky_pre_commit(root)?;

    Ok(LinterContext {
        max_lines: config.max_lines_per_function,
        framework,
        pattern: config.architecture_pattern,
        forbidden_imports,
        ignored_paths,
        ai_configs,
        build_command: config.build_command,
        ai_fix_retries: config.ai_fix_retries,
    })
}

/// Actualiza el .gitignore del proyecto para incluir .architect.ai.json
fn update_gitignore(root: &Path) -> Result<()> {
    let gitignore_path = root.join(".gitignore");
    let entry_to_add =
        "# Architect Linter - AI Configuration (contains API keys)\n.architect.ai.json";

    // Verificar si el archivo existe
    if gitignore_path.exists() {
        // Leer el contenido actual
        let content = fs::read_to_string(&gitignore_path).into_diagnostic()?;

        // Verificar si ya contiene la entrada
        if content.contains(".architect.ai.json") {
            println!("✅ .architect.ai.json ya está en el .gitignore");
            return Ok(());
        }

        // Agregar la entrada al final
        let mut updated_content = content;
        if !updated_content.ends_with('\n') {
            updated_content.push('\n');
        }
        updated_content.push_str(entry_to_add);
        updated_content.push('\n');

        fs::write(&gitignore_path, updated_content).into_diagnostic()?;
        println!("✅ .architect.ai.json agregado al .gitignore");
    } else {
        // Crear el .gitignore con la entrada
        let mut gitignore_content =
            String::from("# Architect Linter - AI Configuration (contains API keys)\n");
        gitignore_content.push_str(".architect.ai.json\n");

        fs::write(&gitignore_path, gitignore_content).into_diagnostic()?;
        println!("✅ .gitignore creado con .architect.ai.json");
    }

    Ok(())
}
