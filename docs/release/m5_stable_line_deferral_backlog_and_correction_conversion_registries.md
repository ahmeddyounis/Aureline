# M5 stable-line deferral-backlog and correction-conversion registries

This lane turns leftover launch-time "may slip to v1.0.x" caveats into explicit post-stable truth over the frozen
[M5 stable-line-protection matrix](./m5-stable-line-ops.md). It records the *deferral-backlog* grammar (how a
supported line records every bounded launch-era deferral it still owns — a bounded-feature deferral, a
performance-posture deferral, a migration-path deferral, a compatibility-caveat deferral, a known-limit deferral,
or a documentation-gap deferral — with its exact affected lines, current correction status, linked claim rows,
rollback target, and required narrow / defer / ship decision) and the *correction-conversion* grammar (the
release-room report emitted when a bounded deferral reaches a decision, recording whether it became a shipped
correction, an explicit defer to a named later train, or a visible claim narrowing when it missed its promised
correction train, and naming the active conversion reason) into registry resolvers that produce export-safe,
honest projections, so the shiproom, release-center, executive-steering, program-governance, diagnostics, docs,
CLI, support, and public-proof surfaces resolve one canonical supported-line backlog truth instead of letting a
launch-era caveat linger as folklore. The bounded deferral and the correction conversion are separated in runtime
and serialized state: the deferral item, affected rows, correction status, rollback target, and freshness posture
live on the deferral backlog, while the resolved line identity, affected-claim reference, target-train reference,
conversion-scope state, narrowed-claim state, active conversion reason, and last conversion revision live on the
correction-conversion report, and a line's correction posture stays preserved so support language never runs ahead
of a completed correction.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_stable_line_deferral_backlog_and_correction_conversion_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-stable-line-deferral-backlog-and-correction-conversion-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-supported-line-defect-ledger.schema.json`](../../schemas/program/m5-supported-line-defect-ledger.schema.json)
  (reused from the frozen matrix, the supported-line backlog ledger) and
  [`schemas/program/m5-correction-conversion-report.schema.json`](../../schemas/program/m5-correction-conversion-report.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-stable-line-deferral-backlog-and-correction-conversion-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:**
  `fixtures/release/m5-stable-line-deferral-backlog-and-correction-conversion-registries/`
  (`deferral_backlog_beta_narrowed.json`, `correction_conversion_preview_narrowed.json`).

## Two registries

1. **Deferral backlog** (`resolve_deferral_backlog_entry`) — records one typed backlog object per bounded
   launch-era deferral: the deferral item and its canonical mode, the affected repo / journey rows, the bundle
   IDs, the install topology, the toolchain envelope, the known limits, the rollback target, and the diagnostics
   posture. A clean entry names a canonical registry token, a classified deferral item, and a
   stable-line-protection role, covers the canonical / accessible / audit resolution forms, publishes a complete
   object, preserves its correction posture before a claim widens, and keeps a public-facing deferral's public /
   support claim matched to a shipped correction. Otherwise it degrades honestly — a line widening its claim while
   a bounded deferral is still open, or a public-facing deferral running its support language ahead of its shipped
   correction, degrades to
   `deferral_backlog_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason a widen-over-open-scope-debt attempt must surface.
2. **Correction-conversion report** (`resolve_correction_conversion_entry`) — emits the machine-readable
   release-room report when a bounded item reaches a decision. A clean entry names a classified conversion scope
   (a shipped correction, an explicit defer, or a claim narrowing) and provides the complete line-identity /
   affected-claim / target-train / conversion-scope / narrowed-claim / active-conversion-reason /
   last-conversion-revision report object; a report that would keep support language ahead of a completed
   correction, hide the conversion, or let unresolved scope debt masquerade as shipped degrades to
   `correction_conversion_runs_support_ahead_of_proof_or_drops_correction_conversion`.

## Per-entry backlog reference

The deferral item carries its canonical mode, and the resolver publishes the full backlog object, so the registry
— never a launch-era caveat assumed to have quietly resolved — is the single source of truth.
`deferral_backlog_object_is_complete` rejects an object missing any backlog field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening while a bounded deferral is
open or public support language running ahead of a shipped correction, and `correction_conversion_stays_honest`
rejects a report that has kept support language ahead of a completed correction.

## Acceptance criteria (proven by resolved examples)

- **There is a checked-in supported-line backlog view for launch-era deferrals and v1.0.x corrections, with exact
  affected lines, decision state, and claim impact.** Clean deferral-backlog entries cover the canonical
  bounded-feature / performance-posture / migration-path / compatibility-caveat / known-limit / documentation-gap
  deferrals and the first release-center / shiproom / executive-steering / program-governance / support surfaces,
  an object-incomplete example degrades, and no clean deferral-backlog entry published an incomplete object.
- **An unresolved or overdue "may slip to v1.0.x" item cannot remain invisible: it either appears as a shipped
  correction, an explicit defer, or a visible claim narrowing on the affected line.** A widen-over-open-scope-debt
  example and an unbound example degrade, a clean bounded deferral-backlog entry is present, and no clean entry is
  unbounded or unbound.
- **Operators can export one report showing which supported-line claims narrowed because a bounded correction
  missed its target train.** Clean correction-conversion report entries cover the shipped-correction /
  explicit-defer / claim-narrowing conversion scopes with full resolution-form coverage while providing the
  complete report object — the resolved line identity and the active conversion reason — and a report that would
  keep support language ahead of a completed correction or drop the conversion degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- support-export
cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- csv
cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- report
cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- deferral-backlog-table
cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- fixture-deferral-backlog-beta-narrowed
cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- fixture-correction-conversion-preview-narrowed
```
