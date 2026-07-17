# M5 change-intent lifecycle ops (M05-1284)

This doc is the human-readable contract for the frozen M5 change-intent, start-work-sheet,
linked-change-panel, ready-for-review-handoff, resolve-close-sheet, and blocked-escalate-card matrix.
It opens batch **B153**. The authoritative gate is the Rust validator in
`crates/aureline-ui/src/m5_change_intent_and_engineering_lifecycle_matrix`; this doc explains the shape and
the invariants so review, provider, Git, help, and support consumers reference one canonical component family
rather than surface-local prose.

## What is frozen

Six governed change-intent object classes, each a first-class, durable, provider-aware engineering object:

| Object class | Canonical schema |
| --- | --- |
| `change_intent_record` | `schemas/teamwork/m5-change-intent.schema.json` |
| `start_work_sheet` | `schemas/ui/m5-start-work-sheet.schema.json` |
| `linked_change_panel` | `schemas/ui/m5-linked-change-panel.schema.json` |
| `ready_for_review_handoff_sheet` | `schemas/ui/m5-ready-for-review-handoff-sheet.schema.json` |
| `resolve_close_sheet` | `schemas/ui/m5-resolve-close-sheet.schema.json` |
| `blocked_escalate_card` | `schemas/ui/m5-blocked-escalate-card.schema.json` |

The matrix also references `schemas/teamwork/m5-work-item-handoff-packet.schema.json` for offline handoff
continuity, and binds back to the already-landed `stable_proof_index` and `m5-migration-task-row` schemas.

## Controlled vocabularies

- **Change-intent roles** (`M5ChangeIntentRole`): `provider_ownership_disclosure`,
  `local_versus_provider_state_disclosure`, `linked_engineering_identity_disclosure`, `side_effect_disclosure`
  (the four hard-posture gates), plus `validation_evidence_disclosure`, `publish_later_fallback_disclosure`,
  and `final_resolution_authority_disclosure` (contextual).
- **Commit state** (`M5ChangeIntentCommitState`): `provider_committed` (the only provider-committed truth),
  `local_only_draft`, `queued_for_publish`, `publish_failed_retained`, `provider_unavailable`,
  `offline_handoff_packet`, `stale_relative_to_provider`. `is_provider_committed()` matches only
  `provider_committed`.
- **Relation source** (`M5ChangeIntentRelationSource`): `linked_by_provider`, `linked_locally`,
  `suggested_by_aureline`, `stale_or_broken_relation` — never flattened into one badge.
  `is_provider_linked()` matches only `linked_by_provider`.
- **Blocker state** (`M5ChangeIntentBlockerState`): `ready_to_resolve`, `blocked_by_engineering`,
  `escalation_open`, `awaiting_provider_write`, `resolution_authority_missing`.
  `is_blocked_or_unresolved()` matches everything except `ready_to_resolve`.

## Hard invariants (every row, MUST be false)

1. Start work never silently creates a branch, worktree, review draft, or provider link without separately
   disclosing each side effect.
2. A local handoff packet or queued publish never masquerades as a provider-committed update.
3. `linked_by_provider`, `linked_locally`, `suggested_by_aureline`, and `stale_or_broken_relation` are never
   flattened into one generic relation badge.
4. Tracked work is never auto-resolved while engineering blockers remain unresolved.
5. Local notes, handoff packets, or linked evidence are never dropped when a provider write fails.

## Regenerating the checked-in artifacts

The headless emitter is the only mint-from-truth path:

```text
cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- support-export
cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- csv
cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- report
cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- dashboard
cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- fixture-start-work-sheet-beta-narrowed
cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- fixture-blocked-escalate-card-preview-narrowed
cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- validate
```

Checked-in outputs:

- `artifacts/release/m5-change-intent-proof/support_export.json` — canonical support export.
- `artifacts/release/m5-change-intent-proof/matrix.csv` — machine-readable matrix.
- `artifacts/design/m5-change-intent-component-matrix.md` — Markdown design report.
- `dashboards/m5-change-intent-health.json` — change-intent-health dashboard.
- `fixtures/teamwork/m5-change-intent/*.json` — narrowed fixtures (start-work-sheet Beta,
  blocked-escalate-card Preview) that keep every object class visible.

The inline tests assert the checked-in support export, dashboard, and fixtures never drift from the seed
builders, so regenerate and re-run `cargo test -p aureline-ui` after any change.
