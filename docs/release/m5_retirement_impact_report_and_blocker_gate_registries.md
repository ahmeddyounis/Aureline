# M5 retirement-impact-report and impact-blocker-gate registries

This lane makes retirement safe over the frozen
[M5 retired-state matrix](./m5-retired-state-ops.md) by proving who and what still depends on a retiring surface
before Aureline closes the support window. It emits one export-safe *retirement impact report* per retirement
candidate — a dependency scanner that enumerates the bundles, migration packs, commands / deep links, CLI aliases,
SDK contract rows, saved artifacts, profiles, recipes, marketplace entries, mirrors, and managed / new-tenant
offerings still relying on a soon-to-be-retired M5 line or surface — and emits a typed *impact blocker gate* that
blocks closure while a bundle, a tenant, or a schema / public artifact still points at the candidate. It records the
*retirement-impact-report* grammar (one classified finding per detected dependency — blocking, migration-required,
historical-only, mirror-only, tenant-gated, or informational — carrying its owning team and joined to the current
compatibility / public-proof state and the successor path or manual fallback) and the *impact-blocker-gate* grammar
(the typed gate scope a blocker sits in — a bundle-still-points-at-candidate, tenant-still-points-at-candidate, or
schema-or-public-artifact-still-points-at-candidate blocker, naming the active gate reason) into registry resolvers
that produce export-safe, honest projections, so review packets, support exports, and public-proof consumers resolve
one canonical retirement blast-radius report instead of re-synthesizing retirement impact by hand.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_retirement_impact_report_and_blocker_gate_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-retirement-impact-report-and-blocker-gate-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-retirement-impact-report.schema.json`](../../schemas/program/m5-retirement-impact-report.schema.json)
  (reused from the frozen matrix — the retirement impact report each retiring command / deep link or registry-visible package is recorded against)
  and
  [`schemas/program/m5-retirement-impact-blocker-gate.schema.json`](../../schemas/program/m5-retirement-impact-blocker-gate.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-retirement-impact-report-and-blocker-gate-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first machine-readable
  retirement-impact report — it demonstrates one retirement-blast-radius loop end to end for the first
  retirement-bearing classes.
- **Narrowed fixtures:**
  `fixtures/release/m5-retirement-impact-report-and-blocker-gate-registries/`
  (`retirement_impact_report_beta_narrowed.json`, `impact_blocker_gate_preview_narrowed.json`).

## Two registries

1. **Retirement impact report** (`resolve_retirement_impact_report_entry`) — classifies one detected dependency per
   retirement candidate: the classification (blocking, migration-required, historical-only, mirror-only,
   tenant-gated, informational) and its canonical mode, the exact-build joins (repo rows, bundle IDs, install
   topology, toolchain envelope), the compatibility / known-limits state, the successor path / rollback target, and
   the owning team. A clean entry names a canonical registry token, a classified finding, and a retirement role,
   covers the canonical / accessible / audit resolution forms, publishes a complete object, preserves its rollback /
   export route before a claim widens, and keeps a public-facing successor / fallback field matched to the current
   compatibility / public-proof state. Otherwise it degrades honestly.
2. **Impact blocker gate** (`resolve_impact_blocker_gate_entry`) — surfaces a candidate's blocker list before
   closure. A clean entry names a classified gate scope (bundle-still-points-at-candidate,
   tenant-still-points-at-candidate, or schema-or-public-artifact-still-points-at-candidate) and provides the complete
   gate object; a gate that would run support language ahead of the closed support note, hide the blocker, or let a
   gap masquerade as covered degrades.

## Acceptance criteria (proven by resolved examples)

- **The scanner detects at least one seeded dependency on a retirement candidate and classifies it with a typed
  reason and owning team.** Clean impact-report entries cover the canonical blocking / migration-required /
  historical-only / mirror-only / tenant-gated / informational classifications and the first release-center /
  help-docs / support / marketplace-registry / install-update surfaces, an object-incomplete example degrades, and no
  clean impact-report entry published an incomplete object.
- **Retirement candidates surface a blocker list before closure when bundles, tenants, schemas, or public artifacts
  still point at them.** A widen-without-route example and an unbound example degrade, a clean impact-report entry is
  present, and no clean entry is unbounded or unbound.
- **The generated impact report is reusable in review packets, support exports, and public-proof consumers without
  hand-editing.** Clean impact-blocker-gate entries cover the bundle-still-points-at-candidate /
  tenant-still-points-at-candidate / schema-or-public-artifact-still-points-at-candidate gate scopes with full
  resolution-form coverage while providing the complete gate object, and a gate that would keep support language ahead
  of the closed support note or drop the blocker degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_retirement_impact_report_and_blocker_gate_registries -- support-export
cargo run -p aureline-ui --example dump_m5_retirement_impact_report_and_blocker_gate_registries -- csv
cargo run -p aureline-ui --example dump_m5_retirement_impact_report_and_blocker_gate_registries -- report
cargo run -p aureline-ui --example dump_m5_retirement_impact_report_and_blocker_gate_registries -- retirement-impact-report-table
cargo run -p aureline-ui --example dump_m5_retirement_impact_report_and_blocker_gate_registries -- fixture-retirement-impact-report-beta-narrowed
cargo run -p aureline-ui --example dump_m5_retirement_impact_report_and_blocker_gate_registries -- fixture-impact-blocker-gate-preview-narrowed
```
