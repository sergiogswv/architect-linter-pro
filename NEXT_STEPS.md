# Next Steps - Path to v4.5.0 (Pro Tier Preview)

**Goal:** Transition from stability hardening to advanced analysis features and preparation for the Pro tier.

---

## 🚀 Priority 1: Advanced Static Analysis

### 1. Code Smells Engine ⏰ 1 week
- [ ] Implement cyclomatic complexity analyzer for JS/TS.
- [ ] Implement "God Object" detection (large classes/files).
- [ ] Implement "Deep Nesting" detector.
- [ ] **Target:** Integrated into the Health Score (replaces current binary metrics).

### 2. Pro Feature Gating ⏰ 3-5 days
- [ ] Implement a basic license check mechanism (mocked for now).
- [ ] Separate "Free" and "Pro" features in the CLI output.
- [ ] Add `--pro` flag to enable advanced analysis.

---

## 📈 Priority 2: Reporting & Visualization

### 3. HTML Dashboard ⏰ 1 week
- [ ] Create a static HTML generator for analysis results.
- [ ] Use Chart.js for health score trends.
- [ ] Include detailed violation drill-down in the browser.

### 4. GitHub Annotations Polish ⏰ 2 days
- [ ] Refine the GitHub Action to provide more detailed PR comments.
- [ ] Add "How to fix" suggestions directly in the PR.

---

## 🔧 Priority 3: Developer Experience (DX)

### 5. Config Wizard v2 ⏰ 3 days
- [ ] Improve the interactive `--setup` wizard with better default rule suggestions.
- [ ] Add framework-specific architectural patterns (e.g., Clean Architecture for NestJS).

### 6. LSP Foundation ⏰ 1 week
- [ ] Setup `tower-lsp` infrastructure.
- [ ] Implement basic "diagnostics only" server.

---

## 🧹 Maintenance & Cleanup

- [ ] **Finalize README_ES.md updates** to match current features.
- [ ] **CI/CD Hardening**: Ensure benchmarks run on every PR to prevent performance regressions.
- [ ] **Documentation Website**: Start the Docusaurus project in the `docs/` directory.

---

**Next Major Milestone:** v4.5.0 - Early Adopter Beta with Pro Tier features.
