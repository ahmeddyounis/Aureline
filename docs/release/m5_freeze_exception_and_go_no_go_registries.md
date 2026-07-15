# M5 freeze-exception and go-no-go registries

This lane governs phase-level change budgets, freeze-exception packets, and explicit channel-widening go/no-go
decisions rather than tribal memory and chat-only approvals, over the frozen
[M5 launch-control matrix](./m5_launch_control_contract.md). It implements the matrix's two remaining domain
contracts — [`schemas/program/m5-freeze-exception-packet.schema.json`](../../schemas/program/m5-freeze-exception-packet.schema.json)
and [`schemas/program/m5-go-no-go-decision.schema.json`](../../schemas/program/m5-go-no-go-decision.schema.json) —
as registry resolvers that produce export-safe, honest projections. It turns the *freeze-exception* grammar (how
each governed change class carries its exception scope, rollback/narrowing path, docs/support/migration linkage,
and owner/risk capture so a freeze exception can never become undocumented scope widening) and the *go/no-go*
grammar (how a launch-bearing lane records the go / no-go / conditional-go decision with the preserved evidence
snapshot, ORR signoff, named on-call roster, and authorized widening stage that justified widening) into one
canonical launch-control truth the shiproom, release-center, executive-steering, program-governance, diagnostics,
docs, CLI, support, and public-proof surfaces resolve directly instead of restating change budgets and widening
approvals by hand.

- **Canonical Rust module:** `crates/aureline-ui/src/m5_freeze_exception_and_go_no_go_registries` (the
  authoritative validator).
- **Combined schema:** `schemas/program/m5-freeze-exception-and-go-no-go-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-freeze-exception-packet.schema.json`](../../schemas/program/m5-freeze-exception-packet.schema.json)
  and
  [`schemas/program/m5-go-no-go-decision.schema.json`](../../schemas/program/m5-go-no-go-decision.schema.json)
  as its canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-freeze-exception-and-go-no-go-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-freeze-exception-and-go-no-go-registries/`
  (`freeze_exception_beta_narrowed.json`, `go_no_go_preview_narrowed.json`).

## Two registries

1. **Freeze exception** (`resolve_freeze_exception_entry`) — publishes one typed freeze-exception packet per
   governed change class: the change class and its canonical mode, the exception scope reference, the
   rollback-or-narrowing reference, the docs/support/migration reference, the owner-capture reference, the
   risk-capture reference, the approved-exception / change-budget reference, and the expiry reference. The six
   canonical change classes are phase-allowed change, exception-required change, api/contract change,
   scope-widening change, migration/data change, and dependency/toolchain change (plus an unclassified sentinel).
   A clean entry names a canonical registry token, a classified change class, and a launch-control role, covers the
   canonical / accessible / audit resolution forms, publishes a complete packet, keeps the freeze exception
   documented before widening, and — for an exception-required change class — keeps the exception documented and
   approved. Otherwise it degrades honestly: a freeze exception that would widen scope without a documented,
   approved packet, or that runs a claim ahead of proof, degrades to
   `freeze_exception_widens_scope_undocumented_or_runs_claim_ahead_of_proof`, the structured blocker a
   widen-without-documentation attempt must surface.
2. **Go/no-go decision** (`resolve_go_no_go_entry`) — keeps the explicit channel-widening decision honest and
   queryable. A clean entry names a classified go/no-go decision (go, no-go, or conditional-go) and provides the
   complete resolved-decision-identity / evidence-snapshot-ledger / orr-signoff / on-call-roster / go-no-go-freshness
   / widening-stage / last-go-no-go-revision record; a record that would imply green while its evidence snapshot or
   ORR state is stale, drop the evidence, or let a gap masquerade as covered degrades to
   `go_no_go_drops_evidence_or_implies_green_while_stale`.

## Per-entry freeze-exception reference

The change class carries its canonical mode, and the resolver publishes the full packet object, so the registry —
never a chat-only approval — is the single source of truth. `freeze_exception_object_is_complete` rejects a packet
missing any field, `freeze_exception_stays_documented_before_widening` rejects an exception-required change class
that widens scope without a documented, approved exception, and `go_no_go_stays_honest` rejects a decision record
that would imply green while its evidence is dropped or a gap is unflagged.

| change class | change-class mode | exception scope reference | change-budget reference | expiry reference |
| --- | --- | --- | --- | --- |
| phase allowed change | phase_allowed_change_class | `repo.rows.core-team-canary-archetypes` | `rollback.target.canary-previous-stable` | `diagnostics.posture.full-telemetry` |
| exception required change | exception_required_change_class | `repo.rows.migration-alpha-archetypes` | `rollback.target.migration-previous-toolchain` | `diagnostics.posture.migration-telemetry` |
| dependency or toolchain change | dependency_or_toolchain_change_class | `repo.rows.certified-archetype-archetypes` | `rollback.target.certified-previous-stable` | `diagnostics.posture.certified-telemetry` |

A freeze exception that widens scope without a documented, approved packet degrades to
`freeze_exception_widens_scope_undocumented_or_runs_claim_ahead_of_proof`, an incomplete packet degrades to
`freeze_exception_object_incomplete`, and a decision record that drops the evidence or implies green while stale
degrades to `go_no_go_drops_evidence_or_implies_green_while_stale`, so a widen-without-documentation attempt, an
incomplete packet, or a stale-green decision can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **No item enters committed scope or widening readiness without the required B145 fields.** Clean
  freeze-exception entries cover the canonical phase-allowed / exception-required / api-or-contract /
  scope-widening / migration-or-data / dependency-or-toolchain change classes and the first release-center /
  shiproom / executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and
  no clean freeze-exception entry published an incomplete packet.
- **Freeze exceptions are exportable, attributable packets rather than chat-only approvals.** A
  widen-without-documentation example and an unbound example degrade, a clean documented-before-widening
  freeze-exception entry is present, and no clean entry is unbound or widens scope without a documented exception.
- **Milestone accounting can distinguish integrated work from done work on launch-bearing rows.** Clean go/no-go
  entries cover the go / no-go / conditional-go decisions with full resolution-form coverage while providing the
  complete record — the resolved decision identity, the preserved evidence snapshot, the ORR signoff, and the named
  on-call roster — and a record that would imply green while its evidence is stale or drop the evidence degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- support-export
cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- csv
cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- report
cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- freeze-exception-table
cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- fixture-freeze-exception-beta-narrowed
cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- fixture-go-no-go-preview-narrowed
```
