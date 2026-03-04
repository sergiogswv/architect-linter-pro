#!/bin/bash

# Script unificado de instalación/actualización para Linux/macOS
# Detecta automáticamente si es instalación inicial o actualización

echo "🏛️  ARCHITECT-LINTER PRO v6.0.0 SETUP"
echo ""

# Obtener versiones para comparación
PROJECT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/.*"\([^"]*\)".*/\1/' 2>/dev/null || echo "unknown")
INSTALLED_VERSION="unknown"

# Detectar si ya está instalado
if command -v architect-linter-pro &> /dev/null; then
    MODE="actualización"
    INSTALLED_VERSION=$(architect-linter-pro --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
    echo "📦 Versión actual instalada: v${INSTALLED_VERSION}"
    echo "📦 Versión del proyecto: v${PROJECT_VERSION}"
    echo ""

    # Si es la misma versión, preguntar si continuar
    if [[ "$INSTALLED_VERSION" == "$PROJECT_VERSION" ]]; then
        echo "✅ Ya tienes la última versión instalada."
        echo ""
        read -p "¿Deseas reinstalar de todas formas? (s/N): " response
        if [[ ! "$response" =~ ^[SsYy]$ ]]; then
            echo "Operación cancelada."
            exit 0
        fi
    fi
else
    MODE="instalación"
    echo "📦 Primera instalación detectada"
    echo "📦 Versión del proyecto: v${PROJECT_VERSION}"
    echo ""
fi

# Verificar si hay instancias de architect-linter-pro en ejecución
echo "🔍 Verificando procesos en ejecución..."
RUNNING_PIDS=$(pgrep -f "architect-linter-pro" 2>/dev/null)

if [ ! -z "$RUNNING_PIDS" ]; then
    echo ""
    echo "⚠️  Encontradas instancias de architect-linter-pro en ejecución."
    echo "Cerrando automáticamente..."
    echo ""
    echo "$RUNNING_PIDS" | while read pid; do
        kill -9 "$pid" 2>/dev/null
        if [ $? -eq 0 ]; then
            echo "  ✓ Proceso $pid cerrado."
        fi
    done
    echo ""
    sleep 1
fi

echo "🦀 Compilando en modo release..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Compilación exitosa."
    echo ""

    if [ "$MODE" = "actualización" ]; then
        echo "📋 Actualizando binario en /usr/local/bin..."
    else
        echo "📋 Instalando binario en /usr/local/bin..."
    fi

    sudo cp target/release/architect-linter-pro /usr/local/bin/

    if [ $? -eq 0 ]; then
        echo ""
        if [ "$MODE" = "actualización" ]; then
            echo "✨ ¡Actualización exitosa!"
        else
            echo "✨ ¡Instalación exitosa!"
        fi
        echo ""
        echo "Nueva versión:"
        architect-linter-pro --version
        echo ""

        if [ "$MODE" = "instalación" ]; then
            echo "🚀 Ahora puedes usar 'architect-linter-pro' en cualquier carpeta."
            echo ""
            echo "📚 Ejemplos de uso (v6.0.0):"
            echo "  architect-linter-pro                    # Análisis básico"
            echo "  architect-linter-pro --watch            # Modo observación"
            echo "  architect-linter-pro --report json -o report.json"
            echo "  architect-linter-pro --help             # Ver todas las opciones"
            echo ""
            echo "Para verificar la instalación, ejecuta:"
            echo "  architect-linter-pro --version"
        else
            echo "💡 La nueva versión ya está disponible en tu terminal."
        fi
        echo ""
    else
        echo "⚠️  Error al copiar el binario. Intenta manualmente:"
        echo "  sudo cp target/release/architect-linter-pro /usr/local/bin/"
    fi
else
    echo "❌ Error en la compilación."
    echo ""
    echo "Posibles causas:"
    echo "  1. El archivo está en uso (cierra todas las instancias de architect-linter-pro)"
    echo "  2. No tienes Rust instalado (https://rustup.rs/)"
    echo "  3. No estás en el directorio del proyecto architect-linter-pro"
    echo ""
    echo "Si el problema persiste, ejecuta:"
    echo "  cargo clean"
    echo "Y vuelve a intentar."
    echo ""
fi
