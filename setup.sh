#!/bin/bash

# Script unificado de instalación/actualización para Linux/macOS
# Detecta automáticamente si es instalación inicial o actualización

echo "🏛️  ARCHITECT-LINTER PRO v4.0.0 SETUP"
echo ""

# Detectar si ya está instalado
if command -v architect-linter-pro &> /dev/null; then
    MODE="actualización"
    echo "📦 Versión actual instalada:"
    architect-linter-pro --version
    echo ""
else
    MODE="instalación"
    echo "📦 Primera instalación detectada"
    echo ""
fi

# Verificar si hay instancias de architect-linter-pro en ejecución
echo "🔍 Verificando procesos en ejecución..."
RUNNING_PIDS=$(pgrep -f "architect-linter-pro" 2>/dev/null)

if [ ! -z "$RUNNING_PIDS" ]; then
    echo ""
    echo "⚠️  ADVERTENCIA: Hay instancias de architect-linter-pro en ejecución."
    echo "Es necesario cerrarlas para poder actualizar el binario."
    echo ""
    echo "Procesos encontrados:"
    echo "$RUNNING_PIDS" | while read pid; do
        echo "  - PID: $pid"
    done
    echo ""
    read -p "¿Deseas cerrarlas automáticamente? (s/N): " response

    if [[ "$response" =~ ^[SsYy]$ ]]; then
        echo "Cerrando procesos..."
        echo "$RUNNING_PIDS" | while read pid; do
            kill -9 "$pid" 2>/dev/null
            if [ $? -eq 0 ]; then
                echo "  ✓ Proceso $pid cerrado."
            fi
        done
        echo ""
        sleep 1
    else
        echo ""
        echo "❌ Instalación cancelada."
        echo "Por favor cierra manualmente las instancias de architect-linter-pro y vuelve a ejecutar este script."
        echo ""
        exit 1
    fi
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
            echo "📚 Ejemplos de uso (v4.3.0):"
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
