# M5 retirement-countdown and pre-retirement-safety-gate registries

This lane turns retirement from a hidden date in release notes into a visible, inspectable countdown over the frozen
[M5 retired-state matrix](./m5-retired-state-ops.md). It emits one export-safe *retirement countdown* per affected
surface — the grace-window countdown a retiring M5 line or stable-facing surface exposes on install / update,
settings / help, docs, marketplace, and support surfaces, carrying the version it was first deprecated in, the cutoff
version / date after which it is Retired, the successor route it forwards to, any remaining overlap window during which
the old and new surfaces coexist, and a no-surprises explanation of what changes at retirement — and one typed
*pre-retirement safety gate* per candidate that blocks final closure while the candidate is still missing a declared
safe-exit route. It records the *retirement-countdown* grammar (one classified countdown field per published fact —
first-deprecated version, cutoff version / date, remaining overlap window, successor route, fallback action, or
no-surprises explanation — carrying its owning team and joined to the current compatibility / public-proof state and
the successor path or manual fallback) and the *pre-retirement-safety-gate* grammar (the typed readiness-check scope a
pre-cutoff blocker sits in — missing-rollback-or-export-path, missing-archive-bundle, or
missing-successor-or-fallback-route, naming the active gate reason) into registry resolvers that produce export-safe,
honest projections, so a product surface and an operator / support surface open the same cutoff and successor data
without contradiction instead of re-synthesizing the countdown by hand.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_retirement_countdown_and_safety_gate_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-retirement-countdown-and-safety-gate-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-retirement-countdown.schema.json`](../../schemas/program/m5-retirement-countdown.schema.json)
  (minted by this lane — the grace-window countdown each affected surface is recorded against)
  and
  [`schemas/program/m5-pre-retirement-safety-gate.schema.json`](../../schemas/program/m5-pre-retirement-safety-gate.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-retirement-countdown-and-safety-gate-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first machine-readable
  retirement countdown — it demonstrates one visible-countdown / pre-cutoff-guardrail loop end to end for the first
  retirement-bearing surfaces.
- **Narrowed fixtures:**
  `fixtures/release/m5-retirement-countdown-and-safety-gate-registries/`
  (`retirement_countdown_beta_narrowed.json`, `safety_gate_preview_narrowed.json`).

## Two registries

1. **Retirement countdown** (`resolve_retirement_countdown_entry`) — publishes one countdown field per affected
   surface: the classification (first-deprecated version, cutoff version / date, remaining overlap window, successor
   route, fallback action, no-surprises explanation) and its canonical mode, the exact-build joins (repo rows, bundle
   IDs, install topology, toolchain envelope), the compatibility / known-limits state, the successor path / rollback
   target, and the owning team. A clean entry names a canonical registry token, a classified countdown field, and a
   retirement role, covers the canonical / accessible / audit resolution forms, publishes a complete object, preserves
   its rollback / export route before a claim widens, and keeps a public-facing successor / fallback field matched to
   the current compatibility / public-proof state. Otherwise it degrades honestly.
2. **Pre-retirement safety gate** (`resolve_safety_gate_entry`) — surfaces a candidate's pre-cutoff blocker list
   before closure. A clean entry names a classified gate scope (missing-rollback-or-export-path, missing-archive-bundle,
   or missing-successor-or-fallback-route) and provides the complete gate object; a gate that would run support
   language ahead of the closed support note, hide the blocker, or let a gap masquerade as covered degrades.

## Acceptance criteria (proven by resolved examples)

- **Affected surfaces can show an active retirement countdown with stable cutoff truth, successor routing, and a
  no-surprises explanation of what will change at retirement.** Clean countdown entries cover the canonical
  first-deprecated-version / cutoff-version-or-date / remaining-overlap-window / successor-route / fallback-action /
  no-surprises-explanation fields and the first release-center / help-docs / support / marketplace-registry /
  install-update surfaces, an object-incomplete example degrades, and no clean countdown entry published an incomplete
  object.
- **A retirement candidate cannot pass to final closure while missing its declared rollback / export / archive path or
  successor / fallback route.** A widen-without-route example and an unbound example degrade, a clean countdown entry
  is present, and no clean entry is unbounded or unbound.
- **At least one product surface and one operator / support surface open the same countdown and successor data without
  contradiction.** Clean pre-retirement-safety-gate entries cover the missing-rollback-or-export-path /
  missing-archive-bundle / missing-successor-or-fallback-route gate scopes with full resolution-form coverage while
  providing the complete gate object, and a gate that would keep support language ahead of the closed support note or
  drop the blocker degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_retirement_countdown_and_safety_gate_registries -- support-export
cargo run -p aureline-ui --example dump_m5_retirement_countdown_and_safety_gate_registries -- csv
cargo run -p aureline-ui --example dump_m5_retirement_countdown_and_safety_gate_registries -- report
cargo run -p aureline-ui --example dump_m5_retirement_countdown_and_safety_gate_registries -- retirement-countdown-table
cargo run -p aureline-ui --example dump_m5_retirement_countdown_and_safety_gate_registries -- fixture-retirement-countdown-beta-narrowed
cargo run -p aureline-ui --example dump_m5_retirement_countdown_and_safety_gate_registries -- fixture-safety-gate-preview-narrowed
```
