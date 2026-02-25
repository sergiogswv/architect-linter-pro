---
title: Windows Installation
sidebar_label: Windows
---

# Installation on Windows

## Quick Install (Recommended)

### Step 1: Clone the repository
```powershell
git clone https://github.com/sergio/architect-linter-pro.git
cd architect-linter-pro
```

### Step 2: Run the installation script

**Note**: It's normal if you see an error about execution policies. Use this command:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\install.ps1
```

**Explanation of flags**:
- `-NoProfile`: Avoids loading your PowerShell profile (prevents errors from `oh-my-posh` or others)
- `-ExecutionPolicy Bypass`: Allows running the script once without changing system configurations

### Step 3: Add to PATH

The script will show you instructions for adding the folder to PATH. You have 2 options:

#### Option A: Automatically (Faster)
1. Open **PowerShell as Administrator** (Win + X → "Terminal (Administrator)")
2. Run these commands:
```powershell
$oldPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$newPath = "$oldPath;C:\Users\YOUR_USERNAME\bin"
[Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
```
**Replace `YOUR_USERNAME` with your Windows username**

#### Option B: Manually
1. Press `Win + X` → "System"
2. Click on "Advanced system settings"
3. Click on "Environment Variables"
4. Under "User variables", select "Path" → "Edit"
5. Click "New" and add: `C:\Users\YOUR_USERNAME\bin`
6. Click "OK" in all windows

### Step 4: Restart the terminal

**IMPORTANT**: Close ALL PowerShell/Terminal windows and open a new one.

### Step 5: Verify installation
```powershell
architect-linter-pro --version
```

You should see: `architect-linter-pro 0.8.0`

---

## Common Issue: "Script execution is disabled"

If you try to run `.\install.ps1` directly and receive:
```
Cannot load file because running scripts is disabled on this system.
```

This is normal on Windows for security reasons. **Use the solution from Quick Install** above (with `-ExecutionPolicy Bypass`).

### Alternatives if the script doesn't work

---

## Solution 1: Temporary Bypass (Fastest) ✅

Run the script with a one-time bypass:

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

**Advantage**: Doesn't change any system configuration, only runs this script.

---

## Solution 2: Enable Scripts for Your User

If you plan to run PowerShell scripts regularly, enable the policy for your user:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Then run:
```powershell
.\install.ps1
```

**What it does**: Allows running local scripts, but still blocks scripts downloaded from the internet (unless signed).

---

## Solution 3: Manual Installation (Without Scripts)

If you prefer not to use PowerShell scripts at all:

### Step 1: Build the project
```powershell
cargo build --release
```

### Step 2: Create a folder for binaries
```powershell
mkdir $env:USERPROFILE\bin -Force
```

### Step 3: Copy the executable
```powershell
copy target\release\architect-linter-pro.exe $env:USERPROFILE\bin\architect-linter-pro.exe
```

### Step 4: Add to PATH

**Option A - Using PowerShell (Requires running as Administrator)**:
```powershell
$oldPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$newPath = "$oldPath;$env:USERPROFILE\bin"
[Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
```

**Option B - Manually**:
1. Press `Win + X` and select "System"
2. Click on "Advanced system settings"
3. Click on "Environment Variables"
4. Under "User variables", select "Path" and click "Edit"
5. Click "New" and add: `C:\Users\YOUR_USERNAME\bin`
6. Click "OK" in all windows

### Step 5: Verify
Restart your terminal and run:
```powershell
architect-linter-pro --help
```

---

## Verification and Usage

### Verify installation
```powershell
architect-linter-pro --version
# Output: architect-linter-pro 0.8.0

architect-linter-pro --help
# Shows complete help
```

### First use
```powershell
cd C:\path\to\your\project
architect-linter-pro
```

The linter will:
1. Automatically detect your framework
2. Guide you to create `architect.json` (first time)
3. Analyze your code and show architectural violations

---

## Uninstall

If you want to uninstall the linter:

```powershell
# Delete the binary
del $env:USERPROFILE\bin\architect-linter-pro.exe

# Optionally, delete the bin folder if empty
rmdir $env:USERPROFILE\bin
```

Then remove `%USERPROFILE%\bin` from your PATH by following Option B steps in reverse.

---

## Troubleshooting

### "cargo: command not found"
You need to install Rust first:
1. Download from: https://rustup.rs/
2. Run the installer
3. Restart your terminal
4. Verify with: `cargo --version`

### "Binary not found after installation"
1. **Verify you added PATH correctly**:
   ```powershell
   $env:Path -split ';' | Select-String 'bin'
   ```
   You should see `C:\Users\YOUR_USERNAME\bin` in the list

2. **IMPORTANT**: Close ALL PowerShell/Terminal windows
   - Environment variables only reload in new sessions
   - Opening a new tab is not enough, you must close all windows

3. **Open a new terminal** and try again:
   ```powershell
   architect-linter-pro --version
   ```

4. If you use VSCode, reload the window (Ctrl+Shift+P → "Reload Window")

### "Access Denied" when adding to PATH
You need to run PowerShell as Administrator:
1. Search for "PowerShell" in the start menu
2. Right-click → "Run as administrator"
3. Run the command to add to PATH

---

## Support

If you have issues, open an issue at: https://github.com/sergio/architect-linter-pro/issues
