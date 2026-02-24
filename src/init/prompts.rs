//! User interaction for the init command.
//! Uses `dialoguer` for interactive terminal prompts.

use crate::config::Framework;
use crate::init::templates::PatternOption;

/// Show a numbered menu and return the selected pattern key.
/// Returns None if the user cancels (Ctrl+C).
pub fn ask_pattern(_framework: &Framework, options: &[PatternOption]) -> Option<String> {
    use dialoguer::{theme::ColorfulTheme, Select};

    let items: Vec<String> = options
        .iter()
        .map(|o| format!("{} ({})", o.label, o.description))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("¿Qué patrón arquitectónico usas?")
        .items(&items)
        .default(0)
        .interact_opt()
        .ok()??;

    Some(options[selection].pattern.to_string())
}

/// Show a numbered menu to select the framework (used when auto-detection fails).
pub fn ask_framework() -> Option<Framework> {
    use dialoguer::{theme::ColorfulTheme, Select};

    let options: Vec<(&str, Framework)> = vec![
        ("Next.js", Framework::React),
        ("NestJS", Framework::NestJS),
        ("Express", Framework::Express),
        ("Django", Framework::Django),
        ("Spring Boot", Framework::Spring),
    ];

    let items: Vec<&str> = options.iter().map(|(label, _)| *label).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("¿Qué framework usas?")
        .items(&items)
        .default(0)
        .interact_opt()
        .ok()??;

    Some(options[selection].1.clone())
}

/// Print the JSON preview to the terminal.
pub fn show_preview(json: &str) {
    println!();
    println!("Vista previa de architect.json:");
    println!("─────────────────────────────────");
    println!("{}", json);
    println!("─────────────────────────────────");
    println!();
}

/// Ask yes/no confirmation. Returns true if confirmed.
pub fn confirm_write() -> bool {
    use dialoguer::{theme::ColorfulTheme, Confirm};

    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("¿Crear architect.json?")
        .default(true)
        .interact()
        .unwrap_or(false)
}
