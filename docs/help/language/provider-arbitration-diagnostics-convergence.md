# Provider Arbitration and Diagnostics Convergence

## What this means

Aureline converges diagnostics from many sources — compilers, language servers, linters, framework analyzers, runtime tests, and policy engines — into one honest model. Instead of flattening everything into a single anonymous warning or error, every surface shows:

- **Which provider found the issue** (compiler, LSP, linter, framework, runtime, policy)
- **How fresh the evidence is** (current, stale, imported from a scan)
- **Whether providers disagree** (severity conflicts, incompatible fixes)
- **Whether a fix is safe to apply** (safe, preview required, blocked)
- **Whether suppression hides the issue for everyone or just one provider**

## How to read a converged diagnostic

In the **Problems** panel and **editor gutters**, you will see:

- The dominant severity (error, warning, notice, hint)
- A display-state label when the cluster is degraded:
  - `CurrentExactLive` — all providers agree, evidence is fresh
  - `CurrentWithSeverityConflict` — providers disagree on how serious this is
  - `DowngradedForProviderDisagreement` — providers disagree on the fix or the rule
  - `StaleOrSuperseded` — the evidence is from an older edit or test run
  - `ImportedSnapshotOnly` — the finding came from an imported scan, not live analysis
  - `SuppressedOrBaselinedGoverned` — the issue is suppressed under a governance policy

Hovering the diagnostic opens a **detail panel** with a table of contributing providers. Each row shows the provider name, its original severity, freshness, and whether its quick fix is safe to apply.

## Quick-fix safety

When you see a lightbulb or quick-fix gesture, the safety class tells you what will happen:

- **SafeToApply** — the fix is narrow and providers agree
- **PreviewRequired** — review the change before applying
- **BlockedForDisagreement** — another provider proposes a different fix; applying either without review may introduce inconsistencies
- **BlockedForStaleState** — the evidence is stale; rerun the task or test first
- **BlockedForPartialScope** — the fix only covers part of the workspace
- **BlockedForGeneratedOrReadOnly** — the fix touches generated or protected files
- **InspectOnly** — no automatic fix is available

## Batch fix-all

**Fix all** is grouped by acting provider and validation availability:

- Same-rule, same-file fixes from one provider can be batched
- Same-provider, workspace-wide fixes can be batched when safety is `SafeToApply`
- Mixed-provider batch fixes are **blocked** when providers disagree, to prevent incompatible edits from being applied as one opaque batch

## Suppression behavior

Suppressing a diagnostic can be:

- **NotSuppressed** — visible to all surfaces
- **ProviderSpecific** — hidden for one provider but still visible from others
- **TimeBounded** — suppressed until a review date
- **PolicyGoverned** — suppressed by a workspace policy or baseline
- **BaselineWaived** — waived through a governed review process

Provider-specific suppression is useful when one tool is noisy but you still want to see the issue from a more authoritative source.

## For support and debugging

If you need to report a diagnostics issue, the support bundle includes the convergence packet JSON. Each cluster preserves:

- Per-provider claims with provider IDs and families
- Freshness classes and anchor remap states
- Suppression and quick-fix safety labels
- Policy context and redaction posture

This lets support see exactly why a diagnostic appeared, disappeared, or was downgraded without needing access to your source code.
