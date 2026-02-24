// vscode/src/extension.ts
import * as vscode from 'vscode';
import { execFile } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';

const execFileAsync = promisify(execFile);

interface ArchitectReport {
  health_score?: {
    total: number;
    grade: string;
  };
  violations: Array<{
    file: string;
    line: number;
    category: string;
    rule: { from: string; to: string };
    import: string;
  }>;
  summary: {
    total_violations: number;
    circular_dependencies: number;
  };
}

let statusBarItem: vscode.StatusBarItem;
let diagnosticCollection: vscode.DiagnosticCollection;

export function activate(context: vscode.ExtensionContext) {
  statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Left,
    100
  );
  statusBarItem.command = 'architect.analyze';
  statusBarItem.tooltip = 'Click to run architecture analysis';
  statusBarItem.text = '$(loading~spin) Architect...';
  statusBarItem.show();
  context.subscriptions.push(statusBarItem);

  diagnosticCollection = vscode.languages.createDiagnosticCollection('architect-linter');
  context.subscriptions.push(diagnosticCollection);

  context.subscriptions.push(
    vscode.commands.registerCommand('architect.analyze', () => runAnalysis(context))
  );

  // Run on save if configured
  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument(() => {
      const config = vscode.workspace.getConfiguration('architectLinter');
      if (config.get<boolean>('runOnSave')) {
        runAnalysis(context);
      }
    })
  );

  // Initial analysis on activation
  runAnalysis(context);
}

async function runAnalysis(_context: vscode.ExtensionContext) {
  const workspaceFolders = vscode.workspace.workspaceFolders;
  if (!workspaceFolders || workspaceFolders.length === 0) {
    statusBarItem.text = '$(warning) Architect: No workspace';
    return;
  }

  const projectRoot = workspaceFolders[0].uri.fsPath;
  const config = vscode.workspace.getConfiguration('architectLinter');
  const binary = config.get<string>('binaryPath') || 'architect-linter-pro';

  statusBarItem.text = '$(loading~spin) Architect: Analyzing...';

  try {
    const { stdout } = await execFileAsync(binary, ['--report', 'json', projectRoot], {
      timeout: 30000,
    });
    const report: ArchitectReport = JSON.parse(stdout);
    updateStatusBar(report);
    updateDiagnostics(report, projectRoot);
  } catch (err: unknown) {
    // Binary exits with code 1 when violations found — stdout still has JSON
    const error = err as { stdout?: string; message?: string };
    if (error.stdout) {
      try {
        const report: ArchitectReport = JSON.parse(error.stdout);
        updateStatusBar(report);
        updateDiagnostics(report, projectRoot);
        return;
      } catch {
        // JSON parse failed, fall through to error state
      }
    }
    statusBarItem.text = '$(error) Architect: Error';
    statusBarItem.tooltip = `Error: ${error.message ?? 'unknown'}. Is architect-linter-pro in PATH?`;
  }
}

function updateStatusBar(report: ArchitectReport) {
  const score = report.health_score;
  if (!score) {
    statusBarItem.text = '$(info) Architect: No score';
    return;
  }
  const icons: Record<string, string> = {
    A: '$(check)', B: '$(check)', C: '$(warning)', D: '$(warning)', F: '$(error)',
  };
  const icon = icons[score.grade] ?? '$(info)';
  statusBarItem.text = `${icon} Arch: ${score.grade} (${score.total})`;
  statusBarItem.tooltip = `Architecture Health: ${score.grade} — ${score.total}/100. Click to re-analyze.`;
}

function updateDiagnostics(report: ArchitectReport, projectRoot: string) {
  diagnosticCollection.clear();
  const byFile = new Map<string, vscode.Diagnostic[]>();

  for (const v of report.violations) {
    const filePath = path.isAbsolute(v.file) ? v.file : path.join(projectRoot, v.file);
    const line = Math.max(0, v.line - 1); // VS Code is 0-based
    const range = new vscode.Range(line, 0, line, 999);
    const severity =
      v.category === 'blocked'
        ? vscode.DiagnosticSeverity.Error
        : vscode.DiagnosticSeverity.Warning;
    const message = `Architecture Violation: '${v.rule.from}' cannot import from '${v.rule.to}' (found: ${v.import})`;
    const diagnostic = new vscode.Diagnostic(range, message, severity);
    diagnostic.source = 'architect-linter';
    diagnostic.code = 'ARCH_VIOLATION';

    const uri = vscode.Uri.file(filePath);
    const existing = byFile.get(uri.fsPath) ?? [];
    existing.push(diagnostic);
    byFile.set(uri.fsPath, existing);
  }

  for (const [filePath, diagnostics] of byFile) {
    diagnosticCollection.set(vscode.Uri.file(filePath), diagnostics);
  }
}

export function deactivate() {
  diagnosticCollection?.dispose();
  statusBarItem?.dispose();
}
