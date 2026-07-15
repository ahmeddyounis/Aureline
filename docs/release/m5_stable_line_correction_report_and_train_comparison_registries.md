# M5 post-launch correction-report and train-comparison registries

This lane makes post-launch learning durable over the frozen
[M5 stable-line-protection matrix](./m5-stable-line-ops.md). It publishes a post-launch correction report per
release train so adoption blockers and field signals tie back to supported-line corrections, freshness gaps, and
narrowed claims instead of staying trapped in incident chat or ticket dashboards. It records the *correction-report*
grammar (the post-launch correction report published per release train — one typed section per operating signal: a
top adoption-blocker section, a crash / support-signal section, a compatibility-report freshness-delta section, a
bundle-drift section, a public-truth-delta section, and a backport-exception / deferral section — each linked to its
correction packets, supported-line defect-ledger entries, and current claim rows) and the *train-comparison* grammar
(the cross-train record naming whether a supported-line issue is a corrected issue, a remaining narrowed claim, or an
open exception still needing explicit closure, naming the active comparison reason) into registry resolvers that
produce export-safe, honest projections, so release / help, support, shiproom, executive-steering,
program-governance, and public-proof surfaces resolve one canonical operating truth instead of rereading raw
incident tickets. The correction report and the cross-train comparison are separated in runtime and serialized
state: the published section, affected rows, linked correction packets / defect-ledger entries / claim rows, rollback
target, and correction posture live on the correction-report entry, while the resolved line identity, linked
correction reference, compared-train reference, comparison-scope state, narrowed-claim state, active comparison
reason, and last revision live on the train-comparison entry, and a train's rollback posture stays preserved so
onboarding / migration / support language never runs ahead of the linked correction evidence.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_stable_line_correction_report_and_train_comparison_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-stable-line-correction-report-and-train-comparison-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-supported-line-defect-ledger.schema.json`](../../schemas/program/m5-supported-line-defect-ledger.schema.json)
  (reused from the frozen matrix — the supported-line defect ledger each report section links back to)
  and
  [`schemas/program/m5-train-comparison.schema.json`](../../schemas/program/m5-train-comparison.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-stable-line-correction-report-and-train-comparison-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first post-launch correction
  report — it demonstrates one full post-stable operating loop end to end for at least one stable-line train.
- **Narrowed fixtures:**
  `fixtures/release/m5-stable-line-correction-report-and-train-comparison-registries/`
  (`correction_report_beta_narrowed.json`, `train_comparison_preview_narrowed.json`).

## Two registries

1. **Correction report** (`resolve_correction_report_entry`) — publishes one typed correction-report section per
   operating signal, per release train: the report section and its canonical mode, the affected line rows, the
   linked correction packets, defect-ledger entries, and claim rows, the freshness delta, the rollback /
   reversibility target, and the owning roster. A clean entry names a canonical registry token, a classified report
   section, and a stable-line-protection role, covers the canonical / accessible / audit resolution forms, publishes
   a complete object, preserves its rollback posture before a claim widens, and keeps a public-facing section's
   compatibility / known-issues / support claim matched to the linked correction evidence. Otherwise it degrades
   honestly — a train widening its claim while its correction is unresolved, or a public-facing section running its
   compatibility / known-issues language ahead of the linked correction, degrades to
   `correction_report_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason a widen-over-unresolved-train attempt must surface.
2. **Train comparison** (`resolve_train_comparison_entry`) — lets operators compare trains and tell corrected issues
   from open exceptions. A clean entry names a classified comparison scope (corrected-issue,
   remaining-narrowed-claim, or open-exception-closure) and provides the complete line-identity / linked-correction /
   compared-train / comparison-scope / narrowed-claim / active-reason / last-revision comparison object; a comparison
   that would keep support language ahead of the linked correction, hide the comparison, or let a gap masquerade as
   covered degrades to
   `train_comparison_runs_support_ahead_of_proof_or_drops_train_comparison`.

## Per-entry report reference

The published section carries its canonical mode, and the resolver publishes the full report object, so the registry
— never a signal assumed to have been resolved — is the single source of truth.
`correction_report_object_is_complete` rejects an object missing any report field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening while a train's correction is
unresolved or compatibility / known-issues language running ahead of the linked correction, and
`train_comparison_stays_honest` rejects a comparison that has kept support language ahead of the linked correction.

## Acceptance criteria (proven by resolved examples)

- **A checked-in post-launch correction report exists for at least one stable-line train and includes adoption
  blockers, crash/support signals, freshness deltas, public-truth drift, and backport exceptions.** Clean
  correction-report entries cover the canonical adoption-blocker / crash-support-signal / compatibility-freshness /
  bundle-drift / public-truth-delta / backport-exception sections and the first release-center / shiproom /
  executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no clean
  correction-report entry published an incomplete object.
- **The report links directly to correction packets, defect-ledger entries, and claim rows rather than summarizing
  them with uncoupled prose.** A widen-over-unresolved-train example and an unbound example degrade, a clean
  correction-report entry is present, and no clean entry is unbounded or unbound.
- **Operators can compare trains and identify which supported-line issues were corrected, which narrowed claims
  remain, and which exceptions still need explicit closure.** Clean train-comparison entries cover the
  corrected-issue / remaining-narrowed-claim / open-exception-closure comparison scopes with full resolution-form
  coverage while providing the complete comparison object — the resolved line identity and the active comparison
  reason — and a comparison that would keep support language ahead of the linked correction or drop the comparison
  degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_stable_line_correction_report_and_train_comparison_registries -- support-export
cargo run -p aureline-ui --example dump_m5_stable_line_correction_report_and_train_comparison_registries -- csv
cargo run -p aureline-ui --example dump_m5_stable_line_correction_report_and_train_comparison_registries -- report
cargo run -p aureline-ui --example dump_m5_stable_line_correction_report_and_train_comparison_registries -- correction-report-table
cargo run -p aureline-ui --example dump_m5_stable_line_correction_report_and_train_comparison_registries -- fixture-correction-report-beta-narrowed
cargo run -p aureline-ui --example dump_m5_stable_line_correction_report_and_train_comparison_registries -- fixture-train-comparison-preview-narrowed
```
