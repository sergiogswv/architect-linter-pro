use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Configura husky y el hook pre-commit en el proyecto destino
pub fn setup_husky_pre_commit(root: &Path) -> Result<()> {
    let package_json_path = root.join("package.json");

    // Verificar si el proyecto tiene package.json
    if !package_json_path.exists() {
        println!("⚠️  No se encontró package.json, omitiendo configuración de husky.");
        return Ok(());
    }

    println!("🔧 Configurando husky y pre-commit hook...");

    // Ejecutar npx husky-init
    let husky_init_output = Command::new("npx")
        .args(["husky-init"])
        .current_dir(root)
        .output();

    match husky_init_output {
        Ok(output) => {
            if output.status.success() {
                println!("✅ husky-init ejecutado correctamente.");

                // Crear el hook pre-commit
                let pre_commit_path = root.join(".husky").join("pre-commit");
                let pre_commit_content = r#"#!/bin/sh
. "$(dirname "$0")/_/husky.sh"

echo "🏗️  Validando configuración de arquitectura..."
architect-linter-pro --check .
if [ $? -ne 0 ]; then
  exit 1
fi

echo "🏗️  Ejecutando Architect Linter Pro..."

# Ejecutar architect-linter-pro en el directorio actual (.)
architect-linter-pro .

# Si el linter encuentra errores, el commit se cancelará
if [ $? -ne 0 ]; then
  echo ""
  echo "❌ El commit fue cancelado debido a violaciones de arquitectura"
  echo "💡 Corrige los errores reportados arriba y vuelve a intentar el commit"
  exit 1
fi

echo "✅ Validación de arquitectura exitosa"
exit 0
"#
                .to_string();

                // Escribir el hook
                fs::write(&pre_commit_path, pre_commit_content).into_diagnostic()?;

                // Dar permisos de ejecución al hook (Unix-like systems)
                #[cfg(unix)]
                {
                    let _ = Command::new("chmod")
                        .args(["+x", pre_commit_path.to_str().unwrap()])
                        .status();
                }

                // Crear también versión para Windows si es necesario
                #[cfg(windows)]
                {
                    let pre_commit_bat = root.join(".husky").join("pre-commit.bat");
                    let pre_commit_bat_content = r#"@echo off
echo 🏗️  Validando configuración de arquitectura...
architect-linter-pro --check .
if errorlevel 1 (
    exit /b 1
)

echo 🏗️  Ejecutando Architect Linter Pro...

# Ejecutar architect-linter-pro en el directorio actual (.)
architect-linter-pro .

# Si el linter encuentra errores, el commit se cancelará
if errorlevel 1 (
    echo.
    echo ❌ El commit fue cancelado debido a violaciones de arquitectura
    echo 💡 Corrige los errores reportados arriba y vuelve a intentar el commit
    exit /b 1
)

echo ✅ Validación de arquitectura exitosa
exit /b 0
"#
                    .to_string();
                    let _ = fs::write(&pre_commit_bat, pre_commit_bat_content);
                }

                println!("✅ Hook pre-commit configurado exitosamente.");
                println!("💡 Ahora architect-linter se ejecutará automáticamente en cada commit.");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("⚠️  Error al ejecutar husky-init: {}", stderr);
                println!("💡 Puedes configurar husky manualmente con: npx husky-init");
            }
        }
        Err(e) => {
            println!("⚠️  No se pudo ejecutar npx husky-init: {}", e);
            println!("💡 Asegúrate de tener Node.js y npm instalados.");
            println!("💡 Para configurar husky manualmente, ejecuta: npx husky-init");
        }
    }

    Ok(())
}
