# M5 retirement-closure-ledger and propagation-blocker-gate registries

This lane propagates retirement manifests, tombstones, and last-supported archive refs into mirror metadata, offline
bundle manifests, self-hosted registry / catalog paths, policy bundles, and managed new-tenant / new-workspace gates —
projected across the first consumer surfaces (release-center, help / docs, support, marketplace / registry,
install / update, and partner / procurement) — over the frozen
[M5 retired-state matrix](./m5-retired-state-ops.md), so mirrors, offline bundles, self-hosted registries, and managed
tenant gates all converge on the same closed retired-state truth rather than one profile quietly keeping a retired line
or capability selectable after another has closed it. It maintains one export-safe *retirement closure ledger* per
retiring object per deployment profile — recording the migration outcome, disable evidence, support-note closure,
archival note, propagation status, and any remaining carve-outs, joined to the exact build, retirement manifest, and
review packet — and one typed *propagation-blocker gate* per object that blocks final retirement certification while a
claimed profile still lags its propagation, diverges from the closed profiles, or keeps advertising a retired line or
capability after another profile has closed it. It records the *retirement-closure-ledger* grammar (one classified
closure field per recorded fact — migration outcome, disable evidence, support-note closure, archival note,
propagation status, or a remaining carve-out — carrying its owning team and joined to the exact build) and the
*propagation-blocker-gate* grammar (the cross-profile blocker a retirement is stopped by — profile-propagation-lag,
profile-retired-state-mismatch, or still-advertising-after-closure, naming the active block reason) into registry
resolvers that produce export-safe, honest projections, so a managed consumer and a mirror / offline / self-hosted
consumer agree on retired-state truth for the same object.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_retirement_closure_ledger_and_propagation_blocker_gate_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-retirement-closure-ledger-and-propagation-blocker-gate-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-retirement-closure-ledger.schema.json`](../../schemas/program/m5-retirement-closure-ledger.schema.json)
  (reused from the frozen retired-state matrix — the retirement closure ledger each retiring object is recorded against)
  and
  [`schemas/program/m5-propagation-blocker-gate.schema.json`](../../schemas/program/m5-propagation-blocker-gate.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-retirement-closure-ledger-and-propagation-blocker-gate-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first machine-readable
  closure-ledger / propagation-blocker loop — it demonstrates one closure-ledger / propagation-blocker-gate loop end to
  end for the first retirement-bearing surfaces.
- **Narrowed fixtures:**
  `fixtures/release/m5-retirement-closure-ledger-and-propagation-blocker-gate-registries/`
  (`retirement_closure_ledger_beta_narrowed.json`, `propagation_blocker_gate_preview_narrowed.json`).

## Two registries

1. **Retirement closure ledger** (`resolve_retirement_closure_ledger_entry`) — records one closure field per retiring
   object per deployment profile: the classification (migration outcome, disable evidence, support-note closure,
   archival note, propagation status, remaining carve-out) and its canonical mode, the exact-build joins (repo rows,
   bundle IDs, install topology, toolchain envelope), the migration / disable state, the archival / successor route, and
   the owning team. A clean entry names a canonical registry token, a classified closure field, and a retirement role,
   covers the canonical / accessible / audit resolution forms, publishes a complete object joined to its exact build,
   and keeps a public-facing archival / successor field matched to the archived successor. Otherwise it degrades
   honestly.
2. **Propagation-blocker gate** (`resolve_propagation_blocker_gate_entry`) — surfaces a retirement's cross-profile
   blockers before final certification. A clean entry names a classified propagation scope (profile-propagation-lag,
   profile-retired-state-mismatch, or still-advertising-after-closure) and provides the complete gate object; a gate
   that fires while a profile still lags, mismatches the closed profiles, or keeps advertising a retired line after
   closure degrades.

## Acceptance criteria (proven by resolved examples)

- **At least one managed consumer and one mirror / offline or self-hosted consumer agree on retired-state truth for the
  same seeded object.** Clean closure entries cover the canonical migration-outcome / disable-evidence /
  support-note-closure / archival-note / propagation-status / remaining-carve-out fields and the first release-center /
  help-docs / support / marketplace-registry / install-update surfaces, an object-incomplete example degrades, and no
  clean closure entry published an incomplete object.
- **The closure ledger records propagation success / failure by profile and can block final retirement certification
  when a claimed profile still diverges.** A closure entry whose exact-build joins are not preserved degrades, an
  unbound example degrades, a clean bounded closure entry is present, and no clean entry is unbounded or unbound.
- **Retired-state propagation does not leak internal-only detail while still naming the archival / successor path needed
  by each profile.** Clean propagation-blocker-gate entries cover the profile-propagation-lag /
  profile-retired-state-mismatch / still-advertising-after-closure scopes with full resolution-form coverage while
  providing the complete gate object, and a gate that would keep a profile diverging or drop its manifest binding
  degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries -- support-export
cargo run -p aureline-ui --example dump_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries -- csv
cargo run -p aureline-ui --example dump_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries -- report
cargo run -p aureline-ui --example dump_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries -- retirement-closure-ledger-table
cargo run -p aureline-ui --example dump_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries -- fixture-retirement-closure-ledger-beta-narrowed
cargo run -p aureline-ui --example dump_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries -- fixture-propagation-blocker-gate-preview-narrowed
```
