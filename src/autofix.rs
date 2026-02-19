use crate::config::{AIConfig, ForbiddenRule};
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser as SwcParser, StringInput, Syntax, TsConfig};

/// Representa una violación arquitectónica detectada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// Ruta del archivo con la violación
    pub file_path: PathBuf,
    /// Contenido completo del archivo
    pub file_content: String,
    /// Import ofensivo que causa la violación
    pub offensive_import: String,
    /// Regla que fue violada
    pub rule: ForbiddenRule,
    /// Línea donde ocurre la violación
    pub line_number: usize,
}

/// Tipo de fix sugerido por la IA
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "fix_type", rename_all = "snake_case")]
pub enum FixType {
    /// Refactorizar código (cambiar imports, etc.)
    Refactor { old_code: String, new_code: String },
    /// Mover archivo a otra capa y actualizar el import en el archivo afectado
    MoveFile {
        from: String,
        to: String,
        updated_import: Option<String>,
    },
    /// Crear un nuevo archivo (helper, util, etc.) y actualizar el import
    CreateFile {
        path: String,
        content: String,
        updated_import: String,
    },
    /// Crear nueva interfaz/abstracción (especializado en desacoplamiento)
    CreateInterface {
        interface_path: String,
        interface_code: String,
        updated_import: String,
    },
}

/// Respuesta estructurada de la IA
#[derive(Debug, Deserialize, Serialize)]
pub struct FixSuggestion {
    #[serde(flatten)]
    pub fix_type: FixType,
    pub explanation: String,
    pub confidence: String, // "high", "medium", "low"
}

/// Consulta a la IA para obtener una sugerencia de fix, con soporte opcional para reintento por error
pub async fn suggest_fix(
    violation: &Violation,
    project_root: &Path,
    ai_configs: &[AIConfig],
    previous_error: Option<&str>,
    previous_suggestion: Option<&str>,
) -> Result<FixSuggestion> {
    // Obtener estructura de carpetas del proyecto
    let folder_structure = get_project_structure(project_root);

    // Obtener un fragmento del código alrededor de la violación
    let lines: Vec<&str> = violation.file_content.lines().collect();
    let start_line = violation.line_number.saturating_sub(10);
    let end_line = std::cmp::min(violation.line_number + 10, lines.len());
    let relevant_code = lines[start_line..end_line].join("\n");

    // Construir prompt estructurado
    let mut prompt = format!(
        r#"Eres un Arquitecto de Software Senior especializado en refactorización. Tu objetivo es resolver una violación arquitectónica.

** REGLAS DE ORO **:
1. El JSON debe ser VÁLIDO y seguir la estructura exacta.
2. `old_code` debe ser EXACTAMENTE el contenido de la línea ofensiva o el bloque ofensivo.
3. `new_code` debe ser sintácticamente válido para el lenguaje del archivo.
4. MUY IMPORTANTE: Antes de sugerir un 'new_code', revisa la ESTRUCTURA DEL PROYECTO para asegurarte de que el archivo/carpeta de destino REALMENTE EXISTE.

** CONTEXTO DEL PROYECTO **:
{}

** VIOLACIÓN **:
Archivo: {}
Regla: No se permite importar desde '{}' en archivos situados en '{}'
Línea ofensiva (Línea {}): {}

** CÓDIGO FUENTE (Alrededor de la violación) **:
```
{}
```

** TAREA **:
Elige la mejor estrategia (refactor, move_file o create_interface) y responde ÚNICAMENTE con el JSON.
Si eliges 'refactor', asegúrate de que 'old_code' sea exactamente la línea '{}'."#,
        folder_structure,
        violation.file_path.display(),
        violation.rule.to,
        violation.rule.from,
        violation.line_number,
        violation.offensive_import,
        relevant_code,
        violation.offensive_import
    );

    // Si hubo un error previo, añadirlo al prompt para que la IA lo corrija
    if let Some(error) = previous_error {
        let suggestion_text = previous_suggestion
            .map(|s| format!("\nTu sugerencia anterior fue: {}", s))
            .unwrap_or_default();
        let error_type = if error.contains("sintaxis") {
            "DE SINTAXIS"
        } else {
            "EL BUILD"
        };
        prompt.push_str(&format!(
            "\n\n⚠️ **ATENCIÓN: TU INTENTO ANTERIOR FALLÓ {}**\n{} \nError: {}\nPor favor, intenta una estrategia DIFERENTE. Si el import anterior no funcionó, puede que la ruta sea incorrecta o necesites crear una interfaz.",
            error_type, suggestion_text, error
        ));
    }

    prompt.push_str(
        r#"

Responde siguiendo ESTRICTAMENTE este esquema JSON:

{
  "fix_type": "refactor",
  "old_code": "import { Objeto } from './incorrecto';",
  "new_code": "import { Objeto } from './correcto';",
  "explanation": "Breve explicación de la mejora.",
  "confidence": "high"
}

O BIEN (Si necesitas mover código a un nuevo archivo):

{
  "fix_type": "create_file",
  "path": "src/utils/materials.ts",
  "content": "export const myHelper = () => { ... }",
  "updated_import": "import { myHelper } from '../utils/materials';",
  "explanation": "Moviendo lógica a un nuevo archivo util.",
  "confidence": "high"
}

O BIEN (Si necesitas mover un archivo existente):

{
  "fix_type": "move_file",
  "from": "src/city/materials.ts",
  "to": "src/utils/materials.ts",
  "updated_import": "import { ... } from '../utils/materials';",
  "explanation": "Moviendo archivo a la capa de utils.",
  "confidence": "high"
}

No incluyas texto fuera del JSON."#,
    );

    // Hacer la petición a la IA usando el sistema de fallback
    let content = crate::ai::consultar_ia_con_fallback(prompt, ai_configs)
        .await
        .map_err(|e| miette::miette!("No se pudo obtener sugerencia de ningún modelo: {}", e))?;

    // Debug logging
    tracing::debug!("IA content response: {}", content);

    // Buscar el primer '{' y el último '}' para extraer solo el JSON
    let json_start = content.find('{').ok_or_else(|| {
        miette::miette!("No se encontró JSON en la respuesta de la IA: {}", content)
    })?;
    let json_end = content.rfind('}').unwrap_or(content.len() - 1) + 1;
    let clean_json = &content[json_start..json_end];

    // Parsear la respuesta JSON
    let suggestion: FixSuggestion =
        serde_json::from_str(clean_json)
            .into_diagnostic()
            .map_err(|e| {
                miette::miette!(
                    "Error parseando respuesta de IA: {}. \nContenido extraído: {}",
                    e,
                    clean_json
                )
            })?;

    Ok(suggestion)
}

/// Orquestador de sugerencia con auto-corrección
pub async fn suggest_fix_with_retry(
    violation: &Violation,
    project_root: &Path,
    ai_configs: &[AIConfig],
    initial_error: Option<&str>,
    previous_suggestion: Option<&str>,
) -> Result<FixSuggestion> {
    let mut attempts = 0;
    const MAX_ATTEMPTS: usize = 3;
    let mut last_error_msg = initial_error.map(|e| e.to_string()).unwrap_or_default();
    let current_prev_suggestion = previous_suggestion.map(|s| s.to_string());

    while attempts < MAX_ATTEMPTS {
        // Intentar obtener una sugerencia (puede fallar por red o por parseo JSON)
        let suggestion_result = if attempts == 0 && initial_error.is_none() {
            suggest_fix(violation, project_root, ai_configs, None, None).await
        } else {
            suggest_fix(
                violation,
                project_root,
                ai_configs,
                Some(&last_error_msg),
                current_prev_suggestion.as_deref(),
            )
            .await
        };

        match suggestion_result {
            Ok(suggestion) => {
                // Si parseó bien, validar sintaxis del código propuesto
                match dry_run_and_validate(&suggestion, violation, project_root) {
                    Ok(_) => return Ok(suggestion),
                    Err(e) => {
                        attempts += 1;
                        last_error_msg = format!("Error de sintaxis en el código propuesto: {}", e);
                        if attempts < MAX_ATTEMPTS {
                            println!("⚠️  La IA sugirió código con errores de sintaxis. Reintentando ({}/{})...", attempts, MAX_ATTEMPTS);
                        }
                    }
                }
            }
            Err(e) => {
                // Si falló el parseo JSON o la comunicación
                attempts += 1;
                last_error_msg = format!("Error de formato JSON o comunicación: {}", e);
                if attempts < MAX_ATTEMPTS {
                    println!(
                        "⚠️  Error en el formato de respuesta de la IA. Reintentando ({}/{})...",
                        attempts, MAX_ATTEMPTS
                    );
                } else {
                    return Err(e);
                }
            }
        }
    }

    Err(miette::miette!(
        "No se pudo obtener una sugerencia válida tras {} intentos.",
        MAX_ATTEMPTS
    ))
}

/// Simula la aplicación del fix y valida la sintaxis en memoria
fn dry_run_and_validate(
    suggestion: &FixSuggestion,
    violation: &Violation,
    _project_root: &Path,
) -> Result<()> {
    match &suggestion.fix_type {
        FixType::Refactor { old_code, new_code } => {
            let old = old_code.trim();
            let new = new_code.trim();

            // Intentar reemplazo exacto
            let mut updated_content = violation.file_content.replace(old, new);

            // Si no funcionó, intentar ignorando punto y coma si la IA lo olvidó
            if violation.file_content == updated_content && !old.ends_with(';') {
                let old_with_semi = format!("{};", old);
                updated_content = violation.file_content.replace(&old_with_semi, new);
            }

            // Si sigue sin funcionar, intentar un reemplazo basado en la línea ofensiva conocida
            if violation.file_content == updated_content {
                let offensive = violation.offensive_import.trim();
                updated_content = violation.file_content.replace(offensive, new);
            }

            if violation.file_content == updated_content {
                return Err(miette::miette!(
                    "El código antiguo ('{}') no se encontró exactamente en el archivo. \
                    Asegúrate de incluir los espacios y el punto y coma exactamente como están.",
                    old
                ));
            }
            validate_syntax_str(&updated_content, &violation.file_path)
        }
        FixType::MoveFile { updated_import, .. } => {
            if let Some(import_fix) = updated_import {
                let updated_content = violation
                    .file_content
                    .replace(&violation.offensive_import, import_fix);
                validate_syntax_str(&updated_content, &violation.file_path)
            } else {
                Ok(())
            }
        }
        FixType::CreateFile { updated_import, .. } => {
            let updated_content = violation
                .file_content
                .replace(&violation.offensive_import, updated_import);
            validate_syntax_str(&updated_content, &violation.file_path)
        }
        FixType::CreateInterface { updated_import, .. } => {
            let updated_content = violation
                .file_content
                .replace(&violation.offensive_import, updated_import);
            if violation.file_content == updated_content {
                return Err(miette::miette!("No se pudo reemplazar el import ofensivo. Asegúrate de que 'updated_import' sea correcto."));
            }
            validate_syntax_str(&updated_content, &violation.file_path)
        }
    }
}

/// Aplica un fix sugerido con validación de sintaxis
pub fn apply_fix(
    suggestion: &FixSuggestion,
    violation: &Violation,
    project_root: &Path,
) -> Result<String> {
    match &suggestion.fix_type {
        FixType::Refactor { old_code, new_code } => {
            let result = apply_refactor(violation, old_code, new_code)?;

            // Validar sintaxis después de aplicar leyendo el archivo
            let content = fs::read_to_string(&violation.file_path).into_diagnostic()?;
            if let Err(e) = validate_syntax_str(&content, &violation.file_path) {
                // Si la sintaxis es inválida, revertir
                fs::write(&violation.file_path, &violation.file_content).into_diagnostic()?;
                return Err(miette::miette!(
                    "El fix sugerido por la IA generó un error de sintaxis al aplicarse y ha sido revertido automáticamente.\nError: {}", 
                    e
                ));
            }
            Ok(result)
        }
        FixType::MoveFile {
            from,
            to,
            updated_import,
        } => apply_move_file(project_root, violation, from, to, updated_import.as_deref()),
        FixType::CreateFile {
            path,
            content,
            updated_import,
        } => apply_create_file(project_root, violation, path, content, updated_import),
        FixType::CreateInterface {
            interface_path,
            interface_code,
            updated_import,
        } => {
            let result = apply_create_interface(
                project_root,
                violation,
                interface_path,
                interface_code,
                updated_import,
            )?;

            // Validar sintaxis del archivo original (donde se cambió el import)
            let content = fs::read_to_string(&violation.file_path).into_diagnostic()?;
            if let Err(e) = validate_syntax_str(&content, &violation.file_path) {
                // Revertir el import (pero dejamos la interfaz creada, es inofensiva)
                fs::write(&violation.file_path, &violation.file_content).into_diagnostic()?;
                return Err(miette::miette!(
                    "El nuevo import para la interfaz generó un error de sintaxis y ha sido revertido.\nError: {}", 
                    e
                ));
            }
            Ok(result)
        }
    }
}

/// Valida que una cadena de código sea sintácticamente válida
pub fn validate_syntax_str(content: &str, file_path_hint: &Path) -> Result<()> {
    let extension = file_path_hint
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if !matches!(extension, "ts" | "tsx" | "js" | "jsx") {
        return Ok(());
    }

    let cm = Arc::new(SourceMap::default());
    let syntax = match extension {
        "ts" | "tsx" => Syntax::Typescript(TsConfig {
            decorators: true,
            tsx: extension == "tsx",
            ..Default::default()
        }),
        "js" | "jsx" => Syntax::Es(EsConfig {
            decorators: true,
            jsx: extension == "jsx",
            ..Default::default()
        }),
        _ => return Ok(()),
    };

    let fm = cm.new_source_file(
        swc_common::FileName::Custom(file_path_hint.to_string_lossy().to_string()),
        content.to_string(),
    );
    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = SwcParser::new_from(lexer);

    parser
        .parse_module()
        .map_err(|e| miette::miette!("Error de sintaxis: {:?}", e))?;

    Ok(())
}

/// Aplica una refactorización de código
fn apply_refactor(violation: &Violation, old_code: &str, new_code: &str) -> Result<String> {
    let content = fs::read_to_string(&violation.file_path).into_diagnostic()?;

    // Reemplazar el código antiguo por el nuevo
    let updated_content = content.replace(old_code.trim(), new_code.trim());

    if content == updated_content {
        return Err(miette::miette!(
            "No se pudo aplicar el fix: el código antiguo no se encontró exactamente"
        ));
    }

    // Escribir el archivo actualizado
    fs::write(&violation.file_path, &updated_content).into_diagnostic()?;

    Ok(format!(
        "✅ Refactorizado: {}",
        violation.file_path.display()
    ))
}

/// Aplica el movimiento de un archivo y opcionalmente actualiza el import
fn apply_move_file(
    project_root: &Path,
    violation: &Violation,
    from: &str,
    to: &str,
    updated_import: Option<&str>,
) -> Result<String> {
    let from_path = project_root.join(from);
    let to_path = project_root.join(to);

    // Crear el directorio destino si no existe
    if let Some(parent) = to_path.parent() {
        fs::create_dir_all(parent).into_diagnostic()?;
    }

    // Mover el archivo
    fs::rename(&from_path, &to_path).into_diagnostic()?;

    let mut msg = format!("✅ Archivo movido: {} → {}", from, to);

    // Actualizar el import si se proporcionó uno
    if let Some(import_fix) = updated_import {
        let content = fs::read_to_string(&violation.file_path).into_diagnostic()?;
        let updated_content = content.replace(&violation.offensive_import, import_fix);
        fs::write(&violation.file_path, updated_content).into_diagnostic()?;
        msg.push_str(&format!(
            " y import actualizado en {}",
            violation.file_path.display()
        ));
    }

    Ok(msg)
}

/// Crea un nuevo archivo y actualiza el import
fn apply_create_file(
    project_root: &Path,
    violation: &Violation,
    path: &str,
    content: &str,
    updated_import: &str,
) -> Result<String> {
    let full_path = project_root.join(path);

    // Crear el directorio si no existe
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).into_diagnostic()?;
    }

    // Crear el archivo con el contenido
    fs::write(&full_path, content).into_diagnostic()?;

    // Actualizar el import en el archivo original
    let file_content = fs::read_to_string(&violation.file_path).into_diagnostic()?;
    let updated_content = file_content.replace(&violation.offensive_import, updated_import);
    fs::write(&violation.file_path, updated_content).into_diagnostic()?;

    Ok(format!(
        "✅ Archivo creado: {} y import actualizado en {}",
        path,
        violation.file_path.display()
    ))
}

/// Crea una nueva interfaz y actualiza el import
fn apply_create_interface(
    project_root: &Path,
    violation: &Violation,
    interface_path: &str,
    interface_code: &str,
    updated_import: &str,
) -> Result<String> {
    let interface_full_path = project_root.join(interface_path);

    // Crear el directorio si no existe
    if let Some(parent) = interface_full_path.parent() {
        fs::create_dir_all(parent).into_diagnostic()?;
    }

    // Crear el archivo de interfaz
    fs::write(&interface_full_path, interface_code).into_diagnostic()?;

    // Actualizar el import en el archivo original
    let content = fs::read_to_string(&violation.file_path).into_diagnostic()?;
    let updated_content = content.replace(&violation.offensive_import, updated_import);
    fs::write(&violation.file_path, updated_content).into_diagnostic()?;

    Ok(format!(
        "✅ Interfaz creada: {} y import actualizado en {}",
        interface_path,
        violation.file_path.display()
    ))
}

/// Obtiene la estructura de carpetas del proyecto de forma más profunda
fn get_project_structure(root: &Path) -> String {
    let mut structure = String::new();
    let mut explorer = ProjectExplorer::new(root);
    explorer.explore(root, 0, &mut structure);
    structure
}

struct ProjectExplorer {
    max_depth: usize,
    max_items_per_dir: usize,
}

impl ProjectExplorer {
    fn new(_root: &Path) -> Self {
        Self {
            max_depth: 4,
            max_items_per_dir: 10,
        }
    }

    fn explore(&mut self, dir: &Path, depth: usize, out: &mut String) {
        if depth >= self.max_depth {
            return;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            let mut entries_vec: Vec<_> = entries.flatten().collect();
            // Sort entries: directories first
            entries_vec.sort_by_key(|e| !e.path().is_dir());

            for entry in entries_vec.iter().take(self.max_items_per_dir) {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                if name.starts_with('.')
                    || name == "node_modules"
                    || name == "target"
                    || name == "dist"
                {
                    continue;
                }

                let indent = "  ".repeat(depth);
                if path.is_dir() {
                    out.push_str(&format!("{}📁 {}/\n", indent, name));
                    self.explore(&path, depth + 1, out);
                } else {
                    // Only show files that are likely to be relevant for architecture (TS, JS, etc.)
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    if matches!(
                        ext,
                        "ts" | "tsx" | "js" | "jsx" | "py" | "go" | "php" | "java"
                    ) {
                        out.push_str(&format!("{}  - {}\n", indent, name));
                    }
                }
            }

            if entries_vec.len() > self.max_items_per_dir {
                let indent = "  ".repeat(depth);
                out.push_str(&format!(
                    "{}  ... ({} más items)\n",
                    indent,
                    entries_vec.len() - self.max_items_per_dir
                ));
            }
        }
    }
}

/// Ejecuta el comando de build configurado para validar los cambios
#[allow(dead_code)]
pub fn run_build_command(command: &str, project_root: &Path) -> Result<()> {
    let output = capture_build_output(command, project_root)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        return Err(miette::miette!(
            "El comando de build '{}' falló.\n\nSTDOUT:\n{}\n\nSTDERR:\n{}",
            command,
            stdout,
            stderr
        ));
    }

    Ok(())
}

/// Captura la salida del comando de build sin devolver error,
/// para poder comparar errores antes vs después de un fix.
pub fn capture_build_output(command: &str, project_root: &Path) -> Result<std::process::Output> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .current_dir(project_root)
            .output()
            .into_diagnostic()?
    } else {
        Command::new("sh")
            .args(&["-c", command])
            .current_dir(project_root)
            .output()
            .into_diagnostic()?
    };

    Ok(output)
}

/// Extrae las líneas de error de la salida del build (filtra sólo líneas que
/// contienen "error TS" o patrones similares de error del compilador)
pub fn extract_build_errors(output: &std::process::Output) -> Vec<String> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}\n{}", stdout, stderr);

    combined
        .lines()
        .filter(|line| {
            let l = line.trim();
            l.contains("error TS")
                || l.contains("error:")
                || l.contains("Error:")
                || l.contains("ERROR")
        })
        .map(|l| l.trim().to_string())
        .collect()
}
