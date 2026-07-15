# M5 supported-line defect-ledger and backport-decision-timer registries

This lane turns supported-line servicing into measurable program truth over the frozen
[M5 stable-line-protection matrix](./m5-stable-line-ops.md). It records every defect an active supported line owns
and forces a timely backport decision on each, so the stable line proves it can service itself before any LTS
language widens. It records the *defect-ledger* grammar (how a supported line ledgers each defect it owns — a
crash-recovery defect, a rollback/update defect, a support-export defect, a migration/import defect, a
compatibility-regression defect, or a security/data-loss defect — with its affected line rows, defect class,
yes/no/defer backport decision and decision age, rollback target, correction-packet state, and owning release /
support roster) and the *backport-decision-timer* grammar (the machine-readable alert emitted when a defect's
backport decision is missing, still unrecorded past its declared SLA, or has forced a stable/LTS support claim to
narrow, naming the active alert reason) into registry resolvers that produce export-safe, honest projections, so
release / help, support, shiproom, executive-steering, program-governance, and public-proof surfaces resolve one
canonical servicing truth instead of reading a supported-line defect as silently serviced. The defect ledger and
the backport-decision alert are separated in runtime and serialized state: the recorded defect, affected rows,
backport decision, rollback target, and correction-packet posture live on the defect-ledger entry, while the
resolved line identity, affected-claim reference, target-train reference, alert-scope state, narrowed-claim state,
active alert reason, and last revision live on the backport-decision-timer entry, and a line's rollback posture
stays preserved so onboarding / migration / support language never runs ahead of a recorded backport decision.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_stable_line_defect_ledger_and_backport_decision_timer_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-stable-line-defect-ledger-and-backport-decision-timer-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-supported-line-defect-ledger.schema.json`](../../schemas/program/m5-supported-line-defect-ledger.schema.json)
  (reused from the frozen matrix, the supported-line defect ledger with correction owner and backport-decision SLA)
  and
  [`schemas/program/m5-backport-decision-timer.schema.json`](../../schemas/program/m5-backport-decision-timer.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-stable-line-defect-ledger-and-backport-decision-timer-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first correction-packet
  exercise — it demonstrates supported-line servicing and backport-path viability end to end even when no
  emergency stable fix is needed.
- **Narrowed fixtures:**
  `fixtures/release/m5-stable-line-defect-ledger-and-backport-decision-timer-registries/`
  (`defect_ledger_beta_narrowed.json`, `backport_decision_timer_preview_narrowed.json`).

## Two registries

1. **Defect ledger** (`resolve_defect_ledger_entry`) — records one typed defect-ledger entry per supported-line
   defect: the defect class and its canonical mode, the affected line rows, the defect IDs, the severity, the
   decision age, the correction-packet state, the rollback / reversibility target, and the owning roster. A clean
   entry names a canonical registry token, a classified defect class, and a stable-line-protection role, covers the
   canonical / accessible / audit resolution forms, publishes a complete object, preserves its rollback posture
   before a claim widens, and keeps a public-facing defect's onboarding / migration / support claim matched to a
   recorded backport decision. Otherwise it degrades honestly — a line widening its claim while a defect's backport
   decision is missing or overdue, or a public-facing defect running its onboarding / migration language ahead of
   its recorded decision, degrades to
   `defect_ledger_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason a widen-over-unresolved-defect attempt must surface.
2. **Backport-decision timer** (`resolve_backport_decision_timer_entry`) — emits the machine-readable alert when a
   defect's backport decision is missing or overdue. A clean entry names a classified alert scope
   (missing-backport-decision, overdue-backport-decision, or narrowed-support-claim) and provides the complete
   line-identity / affected-claim / target-train / alert-scope / narrowed-claim / active-reason / last-revision
   report object; an alert that would keep support language ahead of a recorded decision, hide the alert, or let a
   gap masquerade as covered degrades to
   `backport_decision_timer_runs_support_ahead_of_proof_or_drops_backport_decision_timer`.

## Per-entry ledger reference

The recorded defect carries its canonical mode, and the resolver publishes the full ledger object, so the registry
— never a defect assumed to have been serviced — is the single source of truth.
`defect_ledger_object_is_complete` rejects an object missing any ledger field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening while a defect's backport
decision is missing or overdue or onboarding / migration language running ahead of a recorded decision, and
`backport_decision_timer_stays_honest` rejects an alert that has kept support language ahead of a recorded
decision.

## Acceptance criteria (proven by resolved examples)

- **Every affected supported-line defect traces to an explicit backport decision, decision age, rollback target,
  and correction owner through a durable ledger or packet export.** Clean defect-ledger entries cover the canonical
  crash-recovery / rollback-update / support-export / migration-import / compatibility-regression /
  security-or-data-loss defect classes and the first release-center / shiproom / executive-steering /
  program-governance / support surfaces, an object-incomplete example degrades, and no clean defect-ledger entry
  published an incomplete object.
- **Missing or overdue backport decisions raise visible alerts and can block promotion or force narrowing of the
  relevant stable/LTS support claim.** A widen-over-unresolved-defect example and an unbound example degrade, a
  clean defect-ledger entry is present, and no clean entry is unbounded or unbound.
- **The first correction-packet exercise produces a checked-in proof artifact demonstrating supported-line
  servicing and backport-path viability on the active stable train.** Clean backport-decision-timer entries cover
  the missing-backport-decision / overdue-backport-decision / narrowed-support-claim alert scopes with full
  resolution-form coverage while providing the complete report object — the resolved line identity and the active
  alert reason — and an alert that would keep support language ahead of a recorded decision or drop the alert
  degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_stable_line_defect_ledger_and_backport_decision_timer_registries -- support-export
cargo run -p aureline-ui --example dump_m5_stable_line_defect_ledger_and_backport_decision_timer_registries -- csv
cargo run -p aureline-ui --example dump_m5_stable_line_defect_ledger_and_backport_decision_timer_registries -- report
cargo run -p aureline-ui --example dump_m5_stable_line_defect_ledger_and_backport_decision_timer_registries -- defect-ledger-table
cargo run -p aureline-ui --example dump_m5_stable_line_defect_ledger_and_backport_decision_timer_registries -- fixture-defect-ledger-beta-narrowed
cargo run -p aureline-ui --example dump_m5_stable_line_defect_ledger_and_backport_decision_timer_registries -- fixture-backport-decision-timer-preview-narrowed
```
