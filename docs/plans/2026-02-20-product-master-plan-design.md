# Architect Linter Pro — Master Product Plan
**Date:** 2026-02-20
**Author:** Solo developer
**Model:** Open Core + GitHub App + SaaS Dashboard
**Budget:** $0 (free tiers)
**Horizon:** 6 months

---

## Context

Architect Linter Pro is a multi-language architecture linter written in Rust. The core (TS/JS import validation, circular dependency detection, health score) is solid and production-ready. Non-TS parsers exist but are untested — marked as `[beta]`. The security analysis module is currently broken (build errors) and was prematurely marketed. The AI auto-fix feature works but is fragile.

**Triple goal:** SaaS revenue + open source community/stars + investment/acquisition attractiveness.

---

## Strategic Approach: Open Core + GitHub App as Growth Engine

Three layers that reinforce each other:

```
DISTRIBUTION (free, open source)
  CLI in Rust — GitHub, Homebrew, cargo install
  GitHub App — comments on PRs automatically

          ↓ sends metrics/reports

SAAS PLATFORM (monetization)
  Web dashboard — health score history, trends
  API — receives data from CLI and GitHub App
  Supabase (DB + Auth) + Vercel (hosting)

          ↓ data for

INVESTOR VALUE
  Metrics: active repos, violations detected,
  health score trends, users, MRR
```

**Key principle:** The CLI never requires the SaaS to work. The SaaS is the natural upgrade when a team wants historical visibility, manager reports, or automatic PR comments.

---

## Tech Stack

| Layer | Technology | Cost |
|---|---|---|
| CLI | Rust (existing) | $0 |
| GitHub App | TypeScript + Vercel serverless | $0 |
| Dashboard | Next.js + Tailwind + Vercel | $0 |
| Database + Auth | Supabase (500MB free tier) | $0 |
| Payments | Stripe | $0 until revenue |

---

## Phases

### Phase 0 — Technical Debt (2-3 weeks)
**Goal:** Credible, honest project. No broken builds, no false claims.

- Fix build: resolve security module import path errors
- Mark non-TS/JS parsers as `[beta]` in CLI output and docs
- Remove/honest-down security claims in README (mark as "in development")
- Fix fragile AI JSON parsing (use `serde_json` properly)
- Add minimal integration tests for TS/JS core
- Update README to reflect actual state

**Deliverable:** Project that compiles, runs, and doesn't lie about its capabilities.

---

### Phase 1 — GitHub App MVP (4-6 weeks)
**Goal:** Viral distribution. Every PR comment is free advertising.

**How it works:**
1. Dev opens PR
2. GitHub App receives webhook (pull_request event)
3. Vercel serverless function downloads changed files
4. Runs analysis using precompiled Rust binary (linux/amd64)
5. Posts comment on PR with violations and health score
6. If user has SaaS account → saves scan to DB
7. If no account → shows signup link in comment

**PR Comment format:**
```
🏗️ Architect Linter Pro

Health Score: 78/100 (B)

⚠️ 3 violations found
• domain/UserService imports infra/DB
• presentation imports domain directly
• cycle detected: A → B → C → A

View full report → [dashboard link]
```

**Distribution mechanics:**
- Listed on GitHub Marketplace
- Free for public repos
- Private repos require account (free or pro)
- `architect.json` in repo controls rules; auto-detect if missing

**Deliverable:** GitHub App live on Marketplace. First real users.

---

### Phase 2 — SaaS Dashboard MVP (6-8 weeks)
**Goal:** First revenue and MRR metric for investors.

**Minimum screens:**
```
/login           → GitHub OAuth (Supabase Auth)
/dashboard       → connected repos with current health score
/repo/:id        → health score history by branch/PR + trend chart
/repo/:id/report → exportable manager report
/settings        → rules config, plan, CLI API key
/pricing         → plans
```

**Pricing:**

| FREE | PRO ($19/mo) | TEAM ($49/mo) |
|---|---|---|
| 1 repo | Unlimited repos | Everything in Pro |
| Last 30 days | Full history | Up to 10 members |
| TS/JS only | All languages | Shared dashboard |
| No export | Export PDF/Markdown | Slack notifications |
| Public GitHub App | Private GitHub App | Priority support |

**Conversion funnel:**
```
Free CLI
  → GitHub App comments on PR
    → Link to dashboard in comment
      → GitHub OAuth signup (1 click)
        → Free health score history (1 repo)
          → Wants more repos or export → PRO
```

**CLI integration (new `--report-url` flag):**
```bash
# With API key configured
architect-linter analyze
✓ Scan complete
✓ Report sent to dashboard
→ https://architectlinter.dev/repo/my-repo/scans/abc123
```

**Deliverable:** SaaS live with free + pro plans. First paying customers.

---

### Phase 3 — Growth and Credibility (ongoing)
**Goal:** Traction metrics for investor conversations.

- Validate Python parser with real projects (next language after TS/JS)
- GitHub Marketplace official listing with screenshots and description
- Product landing page with clear value proposition
- Technical blog posts for SEO and GitHub stars
- Developer relations: post on dev.to, HackerNews, Reddit r/programming

**Metrics to track for investors:**
- GitHub stars (credibility signal)
- Active repos (GitHub App installs)
- Weekly scans run
- MRR (Monthly Recurring Revenue)
- Violations detected (shows real usage)

---

## CLI Changes (minimal, additive)

**Phase 0 fixes:**
- Fix security module build error (import path)
- Fix AI response JSON parsing (use `serde_json`, not string `.find('{')`)
- Add `[beta]` label to non-TS output and docs

**Phase 2 addition:**
```toml
# architect.json
{
  "report_url": "https://api.architectlinter.dev",
  "api_key": "alp_xxxx"
}
```

```bash
ARCHITECT_API_KEY=alp_xxxx architect-linter analyze
```

**What does NOT change:**
- All languages remain (beta labeled)
- Full offline functionality without account
- `architect.json` format stays the same
- AI auto-fix stays (only JSON parsing improves)
- Watch mode, CI/CD integration, health score — all unchanged

---

## What is OUT OF SCOPE (YAGNI)

- VS Code extension
- Enterprise SSO/RBAC
- Custom CI/CD web dashboard
- Secrets detection
- Dependency vulnerability audit
- Web scraping / competitive monitoring

These are valid future features but would split focus for a solo developer at $0.

---

## Success Criteria (6 months)

| Metric | Target |
|---|---|
| GitHub stars | 500+ |
| GitHub App installs | 100+ repos |
| Weekly scans | 1,000+ |
| Free accounts | 200+ |
| MRR | $500+ |
| Build status | Green (no broken features) |

---

## Risk Mitigation

| Risk | Mitigation |
|---|---|
| Vercel function timeout on large repos | Analyze only PR diff files, not full repo |
| Supabase free tier limits | 500MB is plenty for MVP; upgrade when revenue covers it |
| AI auto-fix breaks user code | Already has rollback; improve test coverage in Phase 0 |
| SonarQube adds similar feature | Differentiate on multi-language single config + AI fix |
| Solo dev burnout | Strict phase gates — finish Phase 0 before touching Phase 1 |
