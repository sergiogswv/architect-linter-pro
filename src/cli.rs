/// Módulo CLI - Funciones relacionadas con la interfaz de línea de comandos
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Formato de reporte para exportación
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Json,
    Markdown,
}

impl ReportFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(ReportFormat::Json),
            "markdown" | "md" => Some(ReportFormat::Markdown),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            ReportFormat::Json => "json",
            ReportFormat::Markdown => "md",
        }
    }
}

/// Argumentos procesados de la línea de comandos
#[derive(Debug, Clone)]
pub struct CliArgs {
    /// Ruta del proyecto a analizar (None = modo interactivo)
    pub project_path: Option<String>,
    /// Activar modo watch
    pub watch_mode: bool,
    /// Activar modo fix (auto-reparación con IA)
    pub fix_mode: bool,
    /// Solo analizar archivos staged en git
    pub staged_mode: bool,
    /// Formato de reporte para exportar (json, markdown)
    pub report_format: Option<ReportFormat>,
    /// Ruta del archivo de salida para el reporte
    pub output_path: Option<String>,
}

/// Muestra la ayuda del CLI
pub fn print_help() {
    println!("architect-linter-pro {}", VERSION);
    println!();
    println!("Linter de arquitectura de software con Architecture Health Score");
    println!("Soporte multi-lenguaje: TypeScript, JavaScript, Python, Go, PHP, Java");
    println!();
    println!("USO:");
    println!("  architect-linter-pro [OPCIONES] [RUTA]");
    println!();
    println!("ARGUMENTOS:");
    println!("  [RUTA]    Ruta del proyecto a analizar (opcional, modo interactivo si se omite)");
    println!();
    println!("OPCIONES:");
    println!("  -h, --help           Muestra esta ayuda");
    println!("  -v, --version        Muestra la versión");
    println!("  -w, --watch          Modo watch: observa cambios y re-analiza automáticamente");
    println!("  -f, --fix            Modo fix: sugiere y aplica correcciones automáticas con IA");
    println!("  -s, --staged         Solo analizar archivos staged (git)");
    println!("  -r, --report <FMT>   Exportar reporte: json, markdown");
    println!("  -o, --output <PATH>  Archivo de salida para el reporte");
    println!();
    println!("EJEMPLOS:");
    println!("  architect-linter-pro                         # Modo interactivo");
    println!("  architect-linter-pro .                       # Analizar directorio actual");
    println!("  architect-linter-pro /ruta/a/proyecto        # Analizar proyecto específico");
    println!("  architect-linter-pro --watch .               # Modo watch en directorio actual");
    println!("  architect-linter-pro --fix .                 # Analizar y auto-corregir con IA");
    println!("  architect-linter-pro --staged .              # Solo archivos staged");
    println!("  architect-linter-pro --report json .         # Exportar reporte JSON a stdout");
    println!(
        "  architect-linter-pro -r md -o report.md .    # Exportar reporte Markdown a archivo"
    );
    println!();
    println!("SCORE GRADES:");
    println!("  A (90-100)  - Excellent architecture health");
    println!("  B (80-89)   - Good architecture health");
    println!("  C (70-79)   - Fair architecture health");
    println!("  D (60-69)   - Needs improvement");
    println!("  F (0-59)    - Critical issues");
    println!();
    println!("DOCUMENTACIÓN:");
    println!("  https://github.com/sergiogswv/architect-linter-pro");
}

/// Muestra la versión del linter
pub fn print_version() {
    println!("architect-linter-pro {}", VERSION);
}

/// Procesa los argumentos de línea de comandos
/// Retorna None si se procesó un flag especial (--help, --version)
/// Retorna Some(CliArgs) si hay que continuar con el análisis
pub fn process_args() -> Option<CliArgs> {
    let args: Vec<String> = env::args().collect();

    let mut watch_mode = false;
    let mut fix_mode = false;
    let mut staged_mode = false;
    let mut report_format: Option<ReportFormat> = None;
    let mut output_path: Option<String> = None;
    let mut project_path: Option<String> = None;

    // Procesar argumentos
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--version" | "-v" => {
                print_version();
                return None;
            }
            "--help" | "-h" => {
                print_help();
                return None;
            }
            "--watch" | "-w" => {
                watch_mode = true;
            }
            "--fix" | "-f" => {
                fix_mode = true;
            }
            "--staged" | "-s" => {
                staged_mode = true;
            }
            "--report" | "-r" => {
                // Next argument should be the format
                if i + 1 < args.len() {
                    i += 1;
                    if let Some(fmt) = ReportFormat::from_str(&args[i]) {
                        report_format = Some(fmt);
                    } else {
                        eprintln!(
                            "Error: Formato de reporte inválido '{}'. Usa 'json' o 'markdown'.",
                            args[i]
                        );
                        return None;
                    }
                } else {
                    eprintln!("Error: --report requiere un formato (json o markdown)");
                    return None;
                }
            }
            "--output" | "-o" => {
                // Next argument should be the output path
                if i + 1 < args.len() {
                    i += 1;
                    output_path = Some(args[i].clone());
                } else {
                    eprintln!("Error: --output requiere una ruta de archivo");
                    return None;
                }
            }
            _ => {
                // Si no es un flag, asumimos que es la ruta del proyecto
                if !args[i].starts_with('-') {
                    project_path = Some(args[i].clone());
                }
            }
        }
        i += 1;
    }

    Some(CliArgs {
        project_path,
        watch_mode,
        fix_mode,
        staged_mode,
        report_format,
        output_path,
    })
}
