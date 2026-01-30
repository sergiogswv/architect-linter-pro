mod analyzer;
mod config;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use miette::{GraphicalReportHandler, IntoDiagnostic, Result};
use rayon::prelude::*;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use swc_common::SourceMap;
use walkdir::WalkDir;

use crate::analyzer::analyze_file;
use crate::config::LinterConfig;

fn main() -> Result<()> {
    println!("🏛️  WELCOME TO ARCHITECT-LINTER");

    let args: Vec<String> = env::args().collect();
    let project_root = if args.len() > 1 {
        PathBuf::from(&args[1]).canonicalize().into_diagnostic()?
    } else {
        get_interactive_path()?
    };

    let config = Arc::new(load_config(&project_root)?);
    let files = collect_files(&project_root);

    if files.is_empty() {
        println!("✅ No .ts files found.");
        return Ok(());
    }

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len}")
            .into_diagnostic()?,
    );

    let error_count = Arc::new(Mutex::new(0));

    files.par_iter().for_each(|file_path| {
        let cm = Arc::new(SourceMap::default());
        if let Err(e) = analyze_file(&cm, file_path, &config) {
            let mut count = error_count.lock().unwrap();
            *count += 1;
            let mut out = String::new();
            let _ = GraphicalReportHandler::new().render_report(&mut out, e.as_ref());
            println!("\n📌 Archivo: {}\n{}", file_path.display(), out);
        }
        pb.inc(1);
    });

    pb.finish_and_clear();
    let total_errors = *error_count.lock().unwrap();
    if total_errors > 0 {
        println!("❌ Se encontraron {} violaciones.", total_errors);
        std::process::exit(1);
    } else {
        println!("✨ Proyecto impecable.");
        std::process::exit(0);
    }
}

// Helpers para mantener el main limpio
fn load_config(root: &PathBuf) -> Result<LinterConfig> {
    let path = root.join("architect.json");
    if path.exists() {
        let content = std::fs::read_to_string(path).into_diagnostic()?;
        Ok(serde_json::from_str(&content).unwrap_or_default())
    } else {
        Ok(LinterConfig::default())
    }
}

fn collect_files(root: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            !["node_modules", "dist", ".git", "target"]
                .contains(&e.file_name().to_str().unwrap_or(""))
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "ts"))
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn get_interactive_path() -> Result<PathBuf> {
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
        .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    options.push(">> Manual...".into());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Proyecto")
        .items(&options)
        .interact()
        .into_diagnostic()?;

    if selection == options.len() - 1 {
        let s: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Ruta")
            .interact_text()
            .into_diagnostic()?;
        Ok(PathBuf::from(s))
    } else {
        Ok(projects[selection].clone())
    }
}
