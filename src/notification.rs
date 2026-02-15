//! OS Native Notifications for Architect Linter Pro

use notify_rust::Notification;

pub fn send_violation_alert(project_name: &str, violation_count: usize) {
    let summary = format!("🚫 Arquitectura Violada - {}", project_name);
    let body = format!(
        "Se detectaron {} violaciones arquitectónicas. Haz commit para ver los detalles.",
        violation_count
    );

    let _ = Notification::new()
        .summary(&summary)
        .body(&body)
        .icon("dialog-error")
        .show();
}

pub fn send_cycle_alert(project_name: &str, cycle_count: usize) {
    let summary = format!("🔄 Ciclo Detectado - {}", project_name);
    let body = format!(
        "Se encontraron {} dependencias cíclicas críticas.",
        cycle_count
    );

    let _ = Notification::new()
        .summary(&summary)
        .body(&body)
        .icon("dialog-warning")
        .show();
}

pub fn send_success_notification(project_name: &str) {
    let _ = Notification::new()
        .summary(&format!("✅ Arquitectura OK - {}", project_name))
        .body("Tu proyecto cumple con las reglas arquitectónicas.")
        .icon("dialog-information")
        .show();
}
