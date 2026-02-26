# 🚀 Architect Linter Pro - Installation & Updates

## Quick Start

### First Installation
```bash
cd /path/to/architect-linter-pro
./install.sh
```

### Update Existing Installation
```bash
cd /path/to/architect-linter-pro
./install.sh
```

That's it! The script will:
1. ✅ Check for updates
2. ✅ Compile the latest version
3. ✅ Backup your current version
4. ✅ Install automatically
5. ✅ Verify the installation

---

## Command Options

### Check for Updates (without installing)
```bash
./install.sh --check-only
```
Returns:
- Exit code 0 if up to date
- Exit code 1 if update available

### Force Reinstall
```bash
./install.sh --force
```
Useful if something went wrong or you want to rebuild.

### Verbose Output
```bash
./install.sh --verbose
```
Shows full compilation output for debugging.

### Combine Options
```bash
./install.sh --force --verbose
./install.sh --check-only --verbose
```

---

## What Happens During Installation

### 1. Version Check
```
📦 Project version:   v5.0.2
💾 Installed version: v5.0.0
```

### 2. Build
Compiles the latest Rust binary using:
```bash
cargo build --release
```

### 3. Backup
Saves your current version:
```
~/.cargo/bin/.architect-backups/
architect-linter-pro.v5.0.0.20260226_110046
```
(Automatically keeps last 3 backups)

### 4. Install
Copies new binary to installation directory:
```
~/.cargo/bin/architect-linter-pro
```

### 5. Verify
Tests that the installation works:
```bash
architect-linter-pro --version
```

---

## Troubleshooting

### "Binary is in use" Error
If you see: `cp: no se puede crear el fichero... El archivo de texto está ocupado`

**Solution:** The installer handles this automatically, but if needed:
```bash
# Close any active architect-linter-pro processes
pkill -f architect-linter-pro

# Run installer again
./install.sh --force
```

### Build Failed
```bash
# Check if Cargo is installed
cargo --version

# If not, install Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Try installing again
./install.sh --force --verbose
```

### Version Mismatch After Install
```bash
# This is usually harmless but if concerned:
./install.sh --force

# Verify:
architect-linter-pro --version
```

---

## Backup & Recovery

### View Backups
```bash
ls -lh ~/.cargo/bin/.architect-backups/
```

### Restore Previous Version
```bash
# List available backups
ls ~/.cargo/bin/.architect-backups/

# Restore specific backup
cp ~/.cargo/bin/.architect-backups/architect-linter-pro.v5.0.0.TIMESTAMP \
   ~/.cargo/bin/architect-linter-pro
chmod +x ~/.cargo/bin/architect-linter-pro

# Verify
architect-linter-pro --version
```

---

## Automation

### GitHub Actions Example
```yaml
- name: Update Architect Linter
  run: |
    cd ~/architect-linter-pro
    ./install.sh --force
```

### Cron Job (Auto-update daily)
```bash
# Edit crontab
crontab -e

# Add this line:
0 2 * * * cd ~/architect-linter-pro && ./install.sh --force > /tmp/architect-update.log 2>&1
```

### Pre-commit Hook
```bash
#!/bin/bash
cd ~/architect-linter-pro
./install.sh --check-only

if [ $? -eq 1 ]; then
    echo "⚠️  Update available for Architect Linter Pro"
    echo "Run: cd ~/architect-linter-pro && ./install.sh"
fi
```

---

## Version History

See [CHANGELOG.md](./CHANGELOG.md) for detailed changes in each version.

---

## Support

If you encounter issues:

1. Check [CHANGELOG.md](./CHANGELOG.md) for known issues
2. Run with verbose output: `./install.sh --verbose`
3. Check backups are working: `ls ~/.cargo/bin/.architect-backups/`
4. Review installation logs if available

