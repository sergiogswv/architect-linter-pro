use crate::ai::{AISuggestionResponse, SuggestedRule};
use crate::config::AIConfig;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use miette::{IntoDiagnostic, Result};
use std::env;
use std::path::PathBuf;

/// Imprime el banner de bienvenida con ASCII art y estilo de alto impacto
pub fn print_banner() {
    println!();
    println!(
        "{}",
        style(
            "╔══════════════════════════════════════════════════════════════════════════════════╗"
        )
        .cyan()
    );
    println!(
        "{}",
        style(
            r"
    ___    ____  ______ __  __________________  ______ ______ 
   /   |  / __ \/ ____// / / /  _/_  __/ ____/ / ____//_  __/ 
  / /| | / /_/ / /    / /_/ // /  / / / __/   / /      / /    
 / ___ |/ _, _/ /___ / __  // /  / / / /___  / /___   / /     
/_/  |_/_/ |_|\____//_/ /_/___/ /_/ /_____/  \____/  /_/      
                                                              
    __     _____  _   __ ______ ______ ____           
   / /    /  _/ / | / //_  __// ____// __ \          
  / /     / /  /  |/ /  / /  / __/  / /_/ /          
 / /___ _/ /  / /|  /  / /  / /___ / _, _/           
/_____//___/ /_/ |_/  /_/  /_____//_/ |_|            
"
        )
        .cyan()
        .bold()
    );
    println!(
        "{}",
        style(
            "╚══════════════════════════════════════════════════════════════════════════════════╝"
        )
        .cyan()
    );
    println!();
    println!(
        "{}",
        style("                 Manteniendo la arquitectura de tu código ⚡")
            .white()
            .bold()
    );
    println!();
}

/// Solicita al usuario una o más configuraciones de IA
pub fn ask_ai_configs() -> Result<Vec<AIConfig>> {
    let mut configs = Vec::new();

    loop {
        println!("🤖 CONFIGURACIÓN DE LA IA (#{})", configs.len() + 1);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Pedir un nombre para esta configuración
        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Nombre para esta configuración (ej: Claude Pro, Ollama Local)")
            .interact_text()
            .into_diagnostic()?;

        let providers = vec![
            "Claude (Anthropic)",
            "Gemini (Google)",
            "OpenAI",
            "Groq",
            "Ollama (Local)",
            "Kimi (Moonshot)",
            "DeepSeek",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Selecciona un proveedor de IA")
            .items(&providers)
            .default(0)
            .interact()
            .into_diagnostic()?;

        let provider = match selection {
            0 => crate::config::AIProvider::Claude,
            1 => crate::config::AIProvider::Gemini,
            2 => crate::config::AIProvider::OpenAI,
            3 => crate::config::AIProvider::Groq,
            4 => crate::config::AIProvider::Ollama,
            5 => crate::config::AIProvider::Kimi,
            6 => crate::config::AIProvider::DeepSeek,
            _ => unreachable!(),
        };

        // URLs base según el proveedor (Hardcoded)
        let default_url = match provider {
            crate::config::AIProvider::Claude => "https://api.anthropic.com".to_string(),
            crate::config::AIProvider::Gemini => {
                "https://generativelanguage.googleapis.com".to_string()
            }
            crate::config::AIProvider::OpenAI => "https://api.openai.com/v1".to_string(),
            crate::config::AIProvider::Groq => "https://api.groq.com/openai/v1".to_string(),
            crate::config::AIProvider::Ollama => "http://localhost:11434/v1".to_string(),
            crate::config::AIProvider::Kimi => "https://api.moonshot.ai/v1".to_string(),
            crate::config::AIProvider::DeepSeek => "https://api.deepseek.com".to_string(),
        };

        // Verificar si existen variables de entorno
        let env_url = env::var(format!("{}_BASE_URL", provider.as_str().to_uppercase())).ok();
        let env_key = env::var(format!("{}_API_KEY", provider.as_str().to_uppercase())).ok();

        // Solo pedimos la URL si es Ollama, para los demás usamos la hardcoded (o env)
        let api_url: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("URL de la API para {}", provider.as_str()))
            .default(env_url.unwrap_or(default_url))
            .interact_text()
            .into_diagnostic()?;

        let api_key: String = if provider == crate::config::AIProvider::Ollama {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("API Key (opcional para Ollama)")
                .allow_empty(true)
                .default(env_key.unwrap_or_else(|| String::new()))
                .interact_text()
                .into_diagnostic()?
        } else {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("API Key para {}", provider.as_str()))
                .default(env_key.unwrap_or_else(|| String::new()))
                .interact_text()
                .into_diagnostic()?
        };

        // Obtener modelos dinámicamente usando los curls
        println!(
            "🔍 Conectando con {} para obtener modelos...",
            provider.as_str()
        );
        let runtime = tokio::runtime::Runtime::new().into_diagnostic()?;
        let model: String =
            match runtime.block_on(crate::ai::obtener_modelos_disponibles(&provider, &api_url, &api_key)) {
                Ok(mut models) if !models.is_empty() => {
                    models.sort();
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Selecciona el modelo")
                        .items(&models)
                        .default(0)
                        .interact()
                        .into_diagnostic()?;
                    models[selection].clone()
                }
                Err(e) => {
                    println!(
                        "⚠️  No se pudieron obtener los modelos automáticamente: {}",
                        e
                    );
                    Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "Ingresa el nombre del modelo manualmente (ej: claude-3-5-sonnet-20241022)",
                    )
                    .interact_text()
                    .into_diagnostic()?
                }
                _ => {
                    println!("⚠️  La lista de modelos está vacía.");
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Ingresa el nombre del modelo manualmente")
                        .interact_text()
                        .into_diagnostic()?
                }
            };

        configs.push(crate::config::AIConfig {
            name,
            provider,
            api_url,
            api_key,
            model,
        });

        println!("✅ Configuración añadida.");

        let add_another = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("¿Deseas agregar otro modelo de IA?")
            .default(false)
            .interact()
            .into_diagnostic()?;

        if !add_another {
            break;
        }
    }

    Ok(configs)
}

/// Permite al usuario elegir qué reglas de las sugeridas por la IA desea aplicar.
pub fn ask_user_to_confirm_rules(
    suggestions: AISuggestionResponse,
) -> Result<(Vec<SuggestedRule>, usize)> {
    println!("\n🤖 El Arquitecto Virtual ha analizado tu proyecto.");
    println!(
        "\n🤖 El Arquitecto Virtual sugiere el patrón: {}",
        suggestions.pattern
    );

    let max_lines: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Límite máximo de líneas por función sugerido")
        .default(suggestions.suggested_max_lines)
        .interact_text()
        .into_diagnostic()?;

    println!("Deseas aplicar las siguientes reglas de importación?\n");

    // Preparamos las etiquetas para el menú (Regla + Razón)
    let items: Vec<String> = suggestions
        .rules
        .iter()
        .map(|r| format!("{} -> {} \n   └─ Razón: {}", r.from, r.to, r.reason))
        .collect();

    // Por defecto, todas las reglas están marcadas (true)
    let defaults = vec![true; items.len()];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Usa [Espacio] para marcar/desmarcar y [Enter] para confirmar")
        .items(&items)
        .defaults(&defaults)
        .interact()
        .into_diagnostic()?;

    // Filtramos solo las reglas seleccionadas por el usuario
    let mut selected_rules = Vec::new();
    for index in selections {
        selected_rules.push(suggestions.rules[index].clone());
    }

    Ok((selected_rules, max_lines))
}

pub fn get_interactive_path() -> Result<PathBuf> {
    let current_dir = env::current_dir().into_diagnostic()?;
    let search_dir = current_dir.parent().unwrap_or(&current_dir);

    let projects: Vec<PathBuf> = std::fs::read_dir(search_dir)
        .into_diagnostic()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir() && p.join("package.json").exists())
        .collect();

    let mut options: Vec<String> = projects
        .iter()
        .map(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Unknown".to_string())
        })
        .collect();

    options.push(">> Ingresar ruta manualmente...".into());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Selecciona proyecto")
        .items(&options)
        .interact()
        .into_diagnostic()?;

    if selection == options.len() - 1 {
        let path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Ruta completa")
            .interact_text()
            .into_diagnostic()?;

        Ok(PathBuf::from(path))
    } else {
        Ok(projects[selection].clone())
    }
}
pub fn print_manual_fix_advice(explanation: &str, error: &str) {
    println!();
    println!("{}", style("💡 Resumen para corrección manual:").yellow().bold());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{}", style("Lo que se intentó:").cyan());
    println!("  {}", explanation);
    println!();
    println!("{}", style("Error persistente:").red());
    let trimmed_error = if error.len() > 200 {
        format!("{}...", &error[..200])
    } else {
        error.to_string()
    };
    println!("  {}", trimmed_error);
    println!();
    println!("{}", style("Sugerencia de ajuste:").green());
    println!("  Verifica que el archivo de destino exista o que el 'interface_path'");
    println!("  sea correcto. A veces la IA sugiere rutas que no existen en el disco.");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
}
