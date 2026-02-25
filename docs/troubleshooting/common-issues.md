---
title: Common Issues & Solutions
sidebar_label: Common Issues
---

# Common Issues & Solutions

## Installation Issues

### Issue: "command not found: architect-linter-pro"

**Symptoms:** After installation, running `architect-linter-pro` returns "command not found"

**Solutions:**
1. Verify the binary is in your PATH:
   - **Windows:** `echo $env:Path -split ';' | Select-String 'bin'`
   - **Linux/macOS:** `echo $PATH | grep bin`

2. **IMPORTANT:** Close ALL terminal windows and open a new one. Environment variables only reload in new terminal sessions.

3. If using VSCode, reload the window: Ctrl+Shift+P → "Reload Window"

4. Verify installation:
   - **Windows:** `$env:Path -split ';' | Select-String 'Users'`
   - **Linux/macOS:** `which architect-linter-pro`

### Issue: "cargo: command not found"

**Symptoms:** Build from source fails with this error

**Solution:** Install Rust first
1. Download from: https://rustup.rs/
2. Execute the installer
3. Restart your terminal
4. Verify: `cargo --version`

## Configuration Issues

### Issue: "architect.json not found"

**Symptoms:** Linter reports that architect.json is missing

**Solution:**
1. Create architect.json in your project root
2. Use the quick start: `architect --init`
3. Or copy a template from the [Templates](/docs/templates) section

### Issue: "Invalid JSON in architect.json"

**Symptoms:** Configuration error when running architect

**Solution:**
1. Verify JSON syntax with [jsonlint.com](https://jsonlint.com/)
2. Check for common errors in the [Configuration Errors](/docs/troubleshooting/config-errors) guide
3. Ensure all required fields are present

## Build Issues

### Issue: Build fails after architect --fix

**Symptoms:** Auto-fix runs but subsequent build fails

**Solution:**
1. The linter attempts to rebuild automatically
2. If it fails, check the build error messages
3. Run `architect --fix` again - it adapts based on previous failures
4. Review the generated changes manually if needed

## Performance Issues

### Issue: Linter is slow on large projects

**Symptoms:** Analysis takes a long time to complete

**Solution:**
1. Use the `--watch` mode for incremental analysis instead of full scans
2. Add files to `.gitignore` or configure exclusions in architect.json
3. Reduce complexity limits temporarily to identify bottlenecks

## Getting Help

If you don't find your issue here:

1. Check the [Installation Guide](/docs/installation) for your platform
2. Review the [Configuration Errors](/docs/troubleshooting/config-errors) guide
3. Open an issue on GitHub with:
   - Your `architect.json` file
   - The complete error message
   - Your project type/framework
   - Your operating system and terminal type
