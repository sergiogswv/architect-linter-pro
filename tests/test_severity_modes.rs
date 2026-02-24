use std::env;
use std::process::Command;

fn binary_path() -> String {
    let mut path = env::current_exe().unwrap();
    path.pop(); // remove test binary name
    path.pop(); // remove deps/
    path.push("architect-linter-pro");
    if cfg!(target_os = "windows") {
        path.set_extension("exe");
    }
    path.to_string_lossy().to_string()
}

#[test]
fn test_severity_flag_accepted() {
    let bin = binary_path();
    // If binary doesn't exist (not built yet), skip
    if !std::path::Path::new(&bin).exists() {
        return;
    }
    let output = Command::new(&bin)
        .args(["--severity", "error", "--help"])
        .output()
        .expect("failed to run binary");
    assert!(output.status.success(), "exit code: {}", output.status);
}

#[test]
fn test_severity_warning_flag_accepted() {
    let bin = binary_path();
    if !std::path::Path::new(&bin).exists() {
        return;
    }
    let output = Command::new(&bin)
        .args(["--severity", "warning", "--help"])
        .output()
        .expect("failed to run binary");
    assert!(output.status.success(), "exit code: {}", output.status);
}

#[test]
fn test_severity_info_flag_accepted() {
    let bin = binary_path();
    if !std::path::Path::new(&bin).exists() {
        return;
    }
    let output = Command::new(&bin)
        .args(["--severity", "info", "--help"])
        .output()
        .expect("failed to run binary");
    assert!(output.status.success(), "exit code: {}", output.status);
}
