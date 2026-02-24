# Script unificado de instalación/actualización para Windows
# Detecta automáticamente si es instalación inicial o actualización

Write-Host "===========================================  ARCHITECT-LINTER PRO v4.3.0 SETUP" -ForegroundColor Cyan
Write-Host ""

# Detectar si ya está instalado
$binPath = "$env:USERPROFILE\bin\architect-linter-pro.exe"
$isUpdate = Test-Path $binPath

if ($isUpdate) {
    $mode = "actualizacion"
    Write-Host "Actualizando..." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Version actual instalada:" -ForegroundColor Cyan
    & $binPath --version
    Write-Host ""
} else {
    $mode = "instalacion"
    Write-Host "Primera instalacion detectada" -ForegroundColor Green
    Write-Host ""
}

# Verificar si hay instancias de architect-linter-pro en ejecución
Write-Host "Verificando procesos en ejecucion..." -ForegroundColor Cyan
$runningProcesses = Get-Process -Name "architect-linter-pro" -ErrorAction SilentlyContinue

if ($runningProcesses) {
    Write-Host ""
    Write-Host "ADVERTENCIA: Hay instancias de architect-linter-pro en ejecucion." -ForegroundColor Yellow
    Write-Host "Es necesario cerrarlas para poder actualizar el binario." -ForegroundColor Yellow
    Write-Host ""

    $runningProcesses | ForEach-Object {
        Write-Host "  - PID: $($_.Id)" -ForegroundColor White
    }

    Write-Host ""
    $response = Read-Host "Deseas cerrarlas automaticamente? (S/N)"

    if ($response -eq "S" -or $response -eq "s" -or $response -eq "Y" -or $response -eq "y") {
        Write-Host "Cerrando procesos..." -ForegroundColor Yellow
        $runningProcesses | ForEach-Object {
            Stop-Process -Id $_.Id -Force
            Write-Host "  Proceso $($_.Id) cerrado." -ForegroundColor Green
        }
        Write-Host ""
        Start-Sleep -Seconds 1
    } else {
        Write-Host ""
        Write-Host "Instalacion cancelada." -ForegroundColor Red
        Write-Host "Por favor cierra manualmente las instancias de architect-linter-pro y vuelve a ejecutar este script." -ForegroundColor Yellow
        Write-Host ""
        exit 1
    }
}

Write-Host "Compilando en modo release..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "Compilacion exitosa." -ForegroundColor Green
    Write-Host ""

    # Recolectar todas las ubicaciones donde instalar
    $destinos = @()

    # 1. ~/bin (instalación propia del script)
    $destPath = "$env:USERPROFILE\bin"
    if (!(Test-Path $destPath)) {
        Write-Host "Creando carpeta $destPath..." -ForegroundColor Yellow
        New-Item -ItemType Directory -Path $destPath | Out-Null
    }
    $destinos += "$destPath\architect-linter-pro.exe"

    # 2. ~/.cargo/bin (si existe una versión instalada con cargo install)
    $cargoBin = "$env:USERPROFILE\.cargo\bin\architect-linter-pro.exe"
    if (Test-Path $cargoBin) {
        $destinos += $cargoBin
    }

    # Copiar a cada destino
    $copiasFallidas = @()
    foreach ($destino in $destinos) {
        $timestampAntes = $null
        if (Test-Path $destino) {
            $timestampAntes = (Get-Item $destino).LastWriteTime
        }

        Write-Host "Instalando en: $destino..." -ForegroundColor Cyan
        try {
            Copy-Item "target\release\architect-linter-pro.exe" -Destination $destino -Force -ErrorAction Stop

            # Verificar que la copia realmente cambió el archivo
            $timestampDespues = (Get-Item $destino).LastWriteTime
            if ($timestampAntes -and $timestampAntes -eq $timestampDespues) {
                Write-Host "  ADVERTENCIA: El archivo no cambio (mismo timestamp). Puede estar bloqueado." -ForegroundColor Yellow
                $copiasFallidas += $destino
            } else {
                Write-Host "  OK" -ForegroundColor Green
            }
        } catch {
            Write-Host "  ERROR: $_" -ForegroundColor Red
            Write-Host "  El archivo puede estar en uso. Cierra todas las terminales y reintenta." -ForegroundColor Yellow
            $copiasFallidas += $destino
        }
    }

    if ($copiasFallidas.Count -gt 0) {
        Write-Host ""
        Write-Host "No se pudieron actualizar los siguientes binarios:" -ForegroundColor Red
        $copiasFallidas | ForEach-Object { Write-Host "  $_" -ForegroundColor White }
        Write-Host "Cierra todas las terminales abiertas y vuelve a ejecutar el script." -ForegroundColor Yellow
        Write-Host ""
        exit 1
    }

    Write-Host ""
    if ($isUpdate) {
        Write-Host "Actualizacion exitosa!" -ForegroundColor Green
    } else {
        Write-Host "Instalacion exitosa!" -ForegroundColor Green
    }
    Write-Host ""

    Write-Host "Nueva version:" -ForegroundColor Cyan
    & "$destPath\architect-linter-pro.exe" --version
    Write-Host ""

    if ($isUpdate) {
        Write-Host "Abre una nueva terminal para usar la version actualizada." -ForegroundColor Yellow
    } else {
        # Verificar si está en el PATH solo en instalación
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($currentPath -notlike "*$destPath*") {
            Write-Host "IMPORTANTE: Debes agregar $destPath a tu PATH" -ForegroundColor Yellow
            Write-Host ""
            Write-Host "Opcion 1 - Agregar automaticamente (ejecuta PowerShell como Administrador):" -ForegroundColor Cyan
            Write-Host ""
            $pathCommand = @"
`$oldPath = [Environment]::GetEnvironmentVariable('Path', 'User')
`$newPath = "`$oldPath;$destPath"
[Environment]::SetEnvironmentVariable('Path', `$newPath, 'User')
"@
            Write-Host $pathCommand -ForegroundColor White
            Write-Host ""
            Write-Host "Opcion 2 - Agregar manualmente:" -ForegroundColor Cyan
            Write-Host "  1. Presiona Win + X y selecciona 'Sistema'" -ForegroundColor White
            Write-Host "  2. Click en 'Configuracion avanzada del sistema'" -ForegroundColor White
            Write-Host "  3. Click en 'Variables de entorno'" -ForegroundColor White
            Write-Host "  4. En 'Variables de usuario', selecciona 'Path' y click 'Editar'" -ForegroundColor White
            Write-Host "  5. Click 'Nuevo' y agrega: $destPath" -ForegroundColor White
            Write-Host "  6. Click 'Aceptar' en todas las ventanas" -ForegroundColor White
            Write-Host ""
        } else {
            Write-Host "Ahora puedes usar 'architect-linter-pro' en cualquier carpeta." -ForegroundColor Green
            Write-Host ""
            Write-Host "Ejemplos de uso (v4.3.0):" -ForegroundColor Cyan
            Write-Host "  architect-linter-pro                    # Analisis basico" -ForegroundColor White
            Write-Host "  architect-linter-pro --watch            # Modo observacion" -ForegroundColor White
            Write-Host "  architect-linter-pro --report json -o report.json" -ForegroundColor White
            Write-Host "  architect-linter-pro --help             # Ver todas las opciones" -ForegroundColor White
            Write-Host ""
            Write-Host "Para verificar la instalacion:" -ForegroundColor Cyan
            Write-Host "  architect-linter-pro --version" -ForegroundColor White
            Write-Host ""
        }
    }
    Write-Host ""
} else {
    Write-Host "Error en la compilacion." -ForegroundColor Red
    Write-Host ""
    Write-Host "Posibles causas:" -ForegroundColor Yellow
    Write-Host "  1. El archivo esta en uso (cierra todas las instancias de architect-linter-pro)" -ForegroundColor White
    Write-Host "  2. No tienes Rust instalado (https://rustup.rs/)" -ForegroundColor White
    Write-Host "  3. No estas en el directorio del proyecto architect-linter-pro" -ForegroundColor White
    Write-Host ""
    Write-Host "Si el problema persiste, ejecuta:" -ForegroundColor Cyan
    Write-Host "  cargo clean" -ForegroundColor White
    Write-Host "Y vuelve a intentar." -ForegroundColor White
    Write-Host ""
}
