use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Representa una dependencia cíclica detectada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularDependency {
    /// El ciclo completo de dependencias
    pub cycle: Vec<String>,
    /// Descripción legible del problema
    pub description: String,
}

/// Analizador de dependencias cíclicas
pub struct CircularDependencyAnalyzer {
    /// Grafo de dependencias: node -> [nodes que importa]
    graph: HashMap<String, Vec<String>>,
    /// Directorio raíz del proyecto
    project_root: PathBuf,
    /// Grafo inverso: node -> [nodes que lo importan]
    reverse_graph: HashMap<String, Vec<String>>,
}

impl CircularDependencyAnalyzer {
    /// Crea un nuevo analizador de dependencias cíclicas
    pub fn new(project_root: &Path) -> Self {
        let canonical_root = project_root
            .canonicalize()
            .unwrap_or_else(|_| project_root.to_path_buf());
        Self {
            graph: HashMap::new(),
            project_root: canonical_root,
            reverse_graph: HashMap::new(),
        }
    }

    /// Analiza todos los archivos y construye el grafo de dependencias
    pub fn build_graph(&mut self, files: &[PathBuf]) -> Result<()> {
        for file_path in files {
            // Extraer imports del archivo
            let imports = self.extract_imports(file_path)?;

            // Normalizar la ruta del archivo actual
            let normalized_current = self.normalize_file_path(file_path);
            let current_key = normalized_current.clone();

            // Insertar en el grafo
            self.graph
                .entry(current_key.clone())
                .or_insert_with(Vec::new);

            // Procesar cada import
            for import_path in imports {
                if let Some(resolved) = self.resolve_import_path(file_path, &import_path) {
                    let normalized_import = self.normalize_file_path(&resolved);

                    // Solo agregar dependencias internas del proyecto
                    if self.is_internal_dependency(&normalized_import) {
                        // Evitar auto-importaciones (ciclos triviales de 1 nodo)
                        if current_key == normalized_import {
                            continue;
                        }

                        self.graph
                            .entry(current_key.clone())
                            .or_insert_with(Vec::new)
                            .push(normalized_import.clone());

                        // Actualizar grafo inverso
                        self.reverse_graph
                            .entry(normalized_import)
                            .or_insert_with(Vec::new)
                            .push(current_key.clone());
                    }
                }
            }
        }

        Ok(())
    }

    /// Detecta todos los ciclos en el grafo de dependencias
    pub fn detect_cycles(&self) -> Vec<CircularDependency> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for node in self.graph.keys() {
            if !visited.contains(node) {
                self.dfs_detect_cycles(node, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    /// DFS para detectar ciclos en el grafo
    fn dfs_detect_cycles(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<CircularDependency>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = self.graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs_detect_cycles(neighbor, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(neighbor) {
                    // Encontramos un ciclo
                    let cycle_start = path.iter().position(|x| x == neighbor).unwrap_or(0);
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(neighbor.clone());

                    cycles.push(CircularDependency {
                        cycle: cycle.clone(),
                        description: self.format_cycle_description(&cycle),
                    });
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }

    /// Extrae todos los imports de un archivo usando escaneo de líneas
    fn extract_imports(&self, file_path: &Path) -> Result<Vec<String>> {
        let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Solo procesar archivos TypeScript/JavaScript
        if !matches!(extension, "ts" | "tsx" | "js" | "jsx") {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(file_path).into_diagnostic()?;
        Ok(extract_imports_from_content(&content))
    }

    /// Resuelve un path de import a una ruta de archivo real
    fn resolve_import_path(&self, current_file: &Path, import_path: &str) -> Option<PathBuf> {
        // Ignorar imports externos (node_modules, @/aliases si no se resuelven, etc.)
        if import_path.starts_with('@')
            || import_path.starts_with("node_modules")
            || (!import_path.starts_with('.') && !import_path.starts_with('/'))
        {
            // Podríamos agregar lógica para resolver alias de TypeScript aquí
            // Por ahora, solo procesamos imports relativos
            return None;
        }

        // Resolver path relativo
        let current_dir = current_file.parent()?;
        let resolved = current_dir.join(import_path);

        // 1. Intentar el archivo exacto si existe (import './App.css')
        if resolved.exists() && resolved.is_file() {
            return Some(resolved);
        }

        // 2. Intentar diferentes extensiones si el archivo exacto no existe
        let extensions = ["ts", "tsx", "js", "jsx"];
        for ext in &extensions {
            let with_ext = resolved.with_extension(ext);
            if with_ext.exists() {
                return Some(with_ext);
            }
        }

        // 3. Intentar index.ts/js en directorios
        if resolved.is_dir() {
            let index_ts = resolved.join("index.ts");
            let index_js = resolved.join("index.js");

            if index_ts.exists() {
                return Some(index_ts);
            }
            if index_js.exists() {
                return Some(index_js);
            }
        }

        None
    }

    /// Normaliza una ruta de archivo a una representación canónica
    fn normalize_file_path(&self, path: &Path) -> String {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        let canon_str = normalize_path_str(&canonical.to_string_lossy());
        let root_str = normalize_path_str(&self.project_root.to_string_lossy());

        // Eliminar el prefijo de la raíz para obtener la ruta relativa
        if canon_str.starts_with(&root_str) {
            let relative = &canon_str[root_str.len()..];
            let normalized = relative.trim_start_matches('/').to_string();
            if normalized.is_empty() {
                return ".".to_string();
            }
            normalized
        } else {
            canon_str
        }
    }

    /// Verifica si una dependencia es interna del proyecto
    fn is_internal_dependency(&self, path: &str) -> bool {
        // Es interna si no contiene node_modules
        !path.contains("node_modules")
    }

    /// Formatea una descripción legible del ciclo
    fn format_cycle_description(&self, cycle: &[String]) -> String {
        if cycle.is_empty() {
            return "Ciclo vacío".to_string();
        }

        let mut desc = String::from("Dependencia cíclica detectada:\n");
        for (i, node) in cycle.iter().enumerate() {
            if i < cycle.len() - 1 {
                desc.push_str(&format!("  {} → {}\n", node, cycle[i + 1]));
            }
        }
        desc.push_str(&format!(
            "\n  ⚠️  Esto rompe la jerarquía de capas y crea acoplamiento circular."
        ));

        desc
    }

    /// Actualiza un archivo específico en el grafo (para watch mode)
    pub fn update_file(&mut self, file_path: &Path) -> Result<()> {
        let normalized_current = self.normalize_file_path(file_path);

        // Eliminar aristas antiguas del nodo
        self.invalidate_node(&normalized_current);

        // Re-extraer imports
        let imports = self.extract_imports(file_path)?;

        // Reconstruir aristas
        self.graph
            .entry(normalized_current.clone())
            .or_insert_with(Vec::new);

        for import_path in imports {
            if let Some(resolved) = self.resolve_import_path(file_path, &import_path) {
                let normalized_import = self.normalize_file_path(&resolved);

                if self.is_internal_dependency(&normalized_import) {
                    // Evitar auto-importaciones (ciclos triviales de 1 nodo)
                    if normalized_current == normalized_import {
                        continue;
                    }

                    self.graph
                        .entry(normalized_current.clone())
                        .or_insert_with(Vec::new)
                        .push(normalized_import.clone());

                    // Actualizar grafo inverso
                    self.reverse_graph
                        .entry(normalized_import)
                        .or_insert_with(Vec::new)
                        .push(normalized_current.clone());
                }
            }
        }

        Ok(())
    }

    /// Invalida un nodo en el grafo (elimina sus aristas)
    pub fn invalidate_node(&mut self, node: &str) {
        // Eliminar aristas salientes del grafo directo
        if let Some(deps) = self.graph.get(node) {
            // Eliminar referencias en el grafo inverso
            for dep in deps {
                if let Some(reverse_deps) = self.reverse_graph.get_mut(dep) {
                    reverse_deps.retain(|n| n != node);
                }
            }
        }
        self.graph.remove(node);

        // Eliminar aristas entrantes del grafo inverso
        if let Some(reverse_deps) = self.reverse_graph.get(node) {
            // Eliminar referencias en el grafo directo
            for dep in reverse_deps.clone() {
                if let Some(forward_deps) = self.graph.get_mut(&dep) {
                    forward_deps.retain(|n| n != node);
                }
            }
        }
        self.reverse_graph.remove(node);
    }

    /// Obtiene todos los nodos conectados a un nodo dado (componente fuertemente conexo aproximado)
    /// Útil para re-analizar solo la parte del grafo afectada por un cambio
    pub fn get_affected_nodes(&self, start_node: &str) -> HashSet<String> {
        let mut affected = HashSet::new();
        let mut to_visit = vec![start_node.to_string()];

        while let Some(node) = to_visit.pop() {
            if affected.contains(&node) {
                continue;
            }
            affected.insert(node.clone());

            // Agregar dependencias directas (hacia adelante)
            if let Some(deps) = self.graph.get(&node) {
                for dep in deps {
                    if !affected.contains(dep) {
                        to_visit.push(dep.clone());
                    }
                }
            }

            // Agregar dependencias inversas (hacia atrás)
            if let Some(reverse_deps) = self.reverse_graph.get(&node) {
                for dep in reverse_deps {
                    if !affected.contains(dep) {
                        to_visit.push(dep.clone());
                    }
                }
            }
        }

        affected
    }

    /// Detecta ciclos solo en un subconjunto del grafo (para análisis incremental)
    pub fn detect_cycles_in_subgraph(&self, nodes: &HashSet<String>) -> Vec<CircularDependency> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for node in nodes {
            if !visited.contains(node) {
                self.dfs_detect_cycles_filtered(
                    node,
                    nodes,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                    &mut cycles,
                );
            }
        }

        cycles
    }

    /// DFS para detectar ciclos solo en un subgrafo específico
    fn dfs_detect_cycles_filtered(
        &self,
        node: &str,
        allowed_nodes: &HashSet<String>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<CircularDependency>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = self.graph.get(node) {
            for neighbor in neighbors {
                // Solo seguir nodos que están en el subgrafo permitido
                if !allowed_nodes.contains(neighbor) {
                    continue;
                }

                if !visited.contains(neighbor) {
                    self.dfs_detect_cycles_filtered(
                        neighbor,
                        allowed_nodes,
                        visited,
                        rec_stack,
                        path,
                        cycles,
                    );
                } else if rec_stack.contains(neighbor) {
                    // Encontramos un ciclo
                    let cycle_start = path.iter().position(|x| x == neighbor).unwrap_or(0);
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(neighbor.clone());

                    cycles.push(CircularDependency {
                        cycle: cycle.clone(),
                        description: self.format_cycle_description(&cycle),
                    });
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }
}

/// Función pública para analizar dependencias cíclicas en un proyecto
pub fn analyze_circular_dependencies(
    files: &[PathBuf],
    project_root: &Path,
) -> Result<Vec<CircularDependency>> {
    let mut analyzer = CircularDependencyAnalyzer::new(project_root);
    analyzer.build_graph(files)?;
    Ok(analyzer.detect_cycles())
}

/// Extrae todos los imports de un contenido de archivo usando escaneo de líneas
fn extract_imports_from_content(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        // ES import: import ... from '...'
        if let Some(path) = extract_es_import_path(line) {
            imports.push(path);
        }
        // CommonJS require: require('...')
        if let Some(path) = extract_require_path(line) {
            imports.push(path);
        }
    }
    imports
}

fn extract_es_import_path(line: &str) -> Option<String> {
    if !line.starts_with("import") {
        return None;
    }
    // Find from '...' or from "..."
    let from_idx = line.rfind(" from ")?;
    let after_from = line[from_idx + 6..].trim();
    let quote = after_from.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    let end = after_from[1..].find(quote)?;
    Some(after_from[1..=end].to_string())
}

fn extract_require_path(line: &str) -> Option<String> {
    let req_idx = line.find("require(")?;
    let after = &line[req_idx + 8..];
    let quote = after.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    let end = after[1..].find(quote)?;
    Some(after[1..=end].to_string())
}

/// Normaliza un string de ruta para comparación cross-platform:
/// - Reemplaza backslashes por forward slashes
/// - Convierte a minúsculas (case-insensitive en Windows)
/// - Elimina el prefijo extendido de Windows `\\?\` (aparece como `//?/` tras el reemplazo)
///
/// Este prefijo lo añade `canonicalize()` en Windows y puede causar que `starts_with`
/// falle al comparar `project_root` con rutas de archivo, generando falsos positivos.
fn normalize_path_str(path: &str) -> String {
    let s = path.replace('\\', "/").to_lowercase();
    // Strip the Windows extended-length path prefix \\?\ (becomes //?/ after backslash replacement)
    s.strip_prefix("//?/").map(str::to_string).unwrap_or(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path_str_strips_windows_unc_prefix() {
        // On Windows, canonicalize() returns paths with \\?\ prefix.
        // After replacing backslashes, \\?\ becomes //?/
        let unc_path = "//?/c:/users/sergio/project/src/app.tsx";
        let result = normalize_path_str(unc_path);
        assert_eq!(result, "c:/users/sergio/project/src/app.tsx");
    }

    #[test]
    fn test_normalize_path_str_leaves_unix_paths_unchanged() {
        let unix_path = "/home/user/project/src/app.tsx";
        let result = normalize_path_str(unix_path);
        assert_eq!(result, "/home/user/project/src/app.tsx");
    }

    #[test]
    fn test_normalize_path_str_lowercases_windows_path() {
        let path = "C:/Users/Sergio/project/src/App.tsx";
        let result = normalize_path_str(path);
        assert_eq!(result, "c:/users/sergio/project/src/app.tsx");
    }

    #[test]
    fn test_normalize_path_str_unc_prefix_then_project_root_starts_with_works() {
        // Simulates the core bug: project_root without //?/ but file path with //?/
        // After stripping both, starts_with should succeed
        let file_path = "//?/c:/users/sergio/terminal-core/src/app.tsx";
        let root_path = "c:/users/sergio/terminal-core";

        let file_normalized = normalize_path_str(file_path);
        // file_normalized = "c:/users/sergio/terminal-core/src/app.tsx"
        assert!(
            file_normalized.starts_with(root_path),
            "After stripping //?/, file path should start with root. Got: {}",
            file_normalized
        );
    }
}

/// Imprime un reporte de dependencias cíclicas
pub fn print_circular_dependency_report(cycles: &[CircularDependency]) {
    if cycles.is_empty() {
        println!("✅ No se detectaron dependencias cíclicas.");
        return;
    }

    println!("\n🔴 DEPENDENCIAS CÍCLICAS DETECTADAS\n");
    println!(
        "Se encontraron {} ciclo(s) de dependencias:\n",
        cycles.len()
    );

    for (i, cycle) in cycles.iter().enumerate() {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Ciclo #{}", i + 1);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Mostrar el ciclo completo usando el campo cycle
        println!("📂 Rutas del ciclo:");
        for (j, path) in cycle.cycle.iter().enumerate() {
            if j < cycle.cycle.len() - 1 {
                println!("  {} →", path);
            } else {
                println!("  {} ↑ (cierra el ciclo)", path);
            }
        }
        println!();

        println!("{}", cycle.description);
        println!();
    }

    println!("💡 Soluciones sugeridas:");
    println!("  1. Aplicar Inyección de Dependencias para romper el ciclo");
    println!("  2. Extraer la lógica compartida a un tercer módulo");
    println!("  3. Usar eventos/observadores en lugar de llamadas directas");
    println!("  4. Aplicar el principio de inversión de dependencias (DIP)");
}
