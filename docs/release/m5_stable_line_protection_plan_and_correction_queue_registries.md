# M5 stable-line protection-plan and correction-lane queue registries

This lane is the first implement lane over the frozen
[M5 stable-line-protection matrix](./m5-stable-line-ops.md). It turns the *protection-plan* grammar (how a
supported line binds each protected journey — crash recovery, rollback / update, support export, and migration /
import, plus other named launch-bearing flows — to its regression queue, publishing the queued-regression issue
IDs, the release line, the correction packet, the rollback target, and the delayed-breadth ledger it is
auditable by) and the *correction-lane queue* grammar (how a supported line proves which protected-path
regression is queued for correction and which breadth work is intentionally delayed while it stays open, keeping
every delayed-breadth claim bound to a recorded override or claim-narrowing action rather than to hand-edited
prose) into registry resolvers that produce export-safe, honest projections, so the shiproom, release-center,
executive-steering, program-governance, diagnostics, docs, CLI, support, and public-proof surfaces resolve one
canonical stable-line-protection truth instead of a per-line, hand-copied release-room convention. The
protection plan and the correction-lane queue are separated in runtime and serialized state: the protected
journey, protected-journey rows, queued-regression issue IDs, release line, correction packet, delayed-breadth
ledger, and diagnostics posture live on the protection plan, while the resolved line identity, queued-regression
ledger, rollback-target reference, correction-packet state, backport-decision state, delayed-breadth reference,
and last correction revision live on the correction-lane queue, and a line's rollback and diagnostics posture
stays preserved so breadth work never silently outranks an open crash / rollback / support-export / migration
regression.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_stable_line_protection_plan_and_correction_queue_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/program/m5-stable-line-protection-plan-and-correction-queue-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-stable-line-protection-plan.schema.json`](../../schemas/program/m5-stable-line-protection-plan.schema.json)
  and
  [`schemas/program/m5-correction-lane-queue.schema.json`](../../schemas/program/m5-correction-lane-queue.schema.json)
  as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-stable-line-protection-plan-and-correction-queue-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-stable-line-protection-plan-and-correction-queue-registries/`
  (`protection_plan_beta_narrowed.json`, `correction_queue_preview_narrowed.json`).

## Two registries

1. **Protection plan** (`resolve_protection_plan_entry`) — publishes one typed first-30-day protection-plan
   object per supported line: the protected journey and its canonical mode, the protected-journey rows, the
   queued-regression issue IDs, the release line, the correction packet, the delayed-breadth ledger, the
   rollback target, and the diagnostics posture. A clean entry names a canonical registry token, a classified
   protected journey, and a stable-line-protection role, covers the canonical / accessible / audit resolution
   forms, publishes a complete object, preserves its rollback and diagnostics posture before breadth work
   resumes, and keeps any delayed breadth work bound to a recorded override or claim-narrowing action. Otherwise
   it degrades honestly — a line resuming breadth work without a preserved rollback target and diagnostics
   posture, or breadth work outranking an open regression without a recorded override, degrades to
   `descriptor_lets_cohort_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker reason
   a breadth-over-regression attempt must surface.
2. **Correction-lane queue** (`resolve_correction_queue_entry`) — keeps the correction queue honest. A clean
   entry names a classified correction scope and provides the complete line-identity / queued-regression-ledger
   / rollback-target / correction-packet / backport-decision / delayed-breadth / last-correction-revision queue
   object; a queue that would run breadth work ahead of an open regression, hide the correction queue, or let a
   queued regression masquerade as covered degrades to
   `cohort_evidence_runs_support_ahead_of_proof_or_drops_cohort_evidence`.

## Per-entry protection-plan reference

The protected journey carries its canonical mode, and the resolver publishes the full protection-plan object, so
the registry — never a hand-copied per-line release-room convention — is the single source of truth.
`protection_plan_object_is_complete` rejects an object missing any field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects an unpreserved rollback / diagnostics posture
or breadth work outranking an open regression, and `correction_queue_stays_honest` rejects a correction queue
that has run breadth work ahead of an open regression.

A breadth-over-regression attempt degrades to
`descriptor_lets_cohort_widen_without_rollback_or_runs_support_ahead_of_proof`, an incomplete object degrades to
`cohort_descriptor_object_incomplete`, and a queue running breadth ahead of an open regression degrades to
`cohort_evidence_runs_support_ahead_of_proof_or_drops_cohort_evidence`, so a breadth-over-regression attempt, an
incomplete object, or a queue running breadth ahead of an open regression can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Release operators can open a stable-line protection view and see which protected journeys are guarded, which
  regressions are queued for correction, and which breadth items are intentionally delayed.** Clean
  protection-plan entries cover the canonical crash-recovery / rollback-update / support-export / migration-
  import / launch-bearing-flow / named protected journeys and the first release-center / shiproom /
  executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
  clean protection-plan entry published an incomplete object.
- **The first-30-day plan preserves exact linkage to issue IDs, release lines, correction packets, and rollback
  targets.** A breadth-over-regression example and an unbound example degrade, a clean bounded protection-plan
  entry is present, and no clean entry is unbounded or unbound.
- **Stable-line breadth work cannot silently outrank crash / rollback / support-export / migration regressions
  without a recorded override or claim-narrowing action.** Clean correction-lane queue entries cover the queued-
  regression / backport-decision / correction-report scopes with full resolution-form coverage while providing
  the complete queue object — the resolved line identity and the delayed-breadth reference — and a queue that
  would run breadth work ahead of an open regression or drop the correction queue degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- support-export
cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- csv
cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- report
cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- protection-plan-table
cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- fixture-protection-plan-beta-narrowed
cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- fixture-correction-queue-preview-narrowed
```
