# State-class recovery packet

The canonical state-class recovery packet is implemented in
[`crates/aureline-reactive-state/src/state_class_recovery/mod.rs`](../../crates/aureline-reactive-state/src/state_class_recovery/mod.rs)
and serialized to
[`artifacts/state/state_class_recovery.json`](./state_class_recovery.json).

It is the checked-in truth source for:

- state-family routing, placeholder continuity, and support-safe recovery
  summaries in
  [`crates/aureline-support/src/state_class_recovery/mod.rs`](../../crates/aureline-support/src/state_class_recovery/mod.rs)
- fixture replay in
  [`crates/aureline-reactive-state/tests/state_class_recovery.rs`](../../crates/aureline-reactive-state/tests/state_class_recovery.rs)
- metadata-safe support export in
  [`crates/aureline-support/tests/state_class_recovery_support_export.rs`](../../crates/aureline-support/tests/state_class_recovery_support_export.rs)

## Frozen evidence

The packet proves:

- one explicit recovery route per covered state class instead of a broad reset
- one placeholder continuity plan per family that preserves layout slot and
  logical identity
- one support-safe explanation per family describing what stayed intact and why
  the chosen route is safest
- one fixture corpus covering partial corruption, missing dependency, stale
  generated overlays, broken cache shards, journal-preserved unsaved work, and
  quarantined trust state

## Fixture corpus

The fixture corpus under
[`fixtures/state/state_class_recovery/`](../../fixtures/state/state_class_recovery/)
pins seven scenarios:

| Fixture | Family | Failure mode | Primary route |
| --- | --- | --- | --- |
| `provider_local_draft_partial_corruption.json` | provider local draft | `partial_corruption` | `rollback_to_preserved_artifact` |
| `request_workspace_missing_dependency.json` | request workspace | `missing_dependency` | `guided_repair` |
| `notebook_workspace_partial_corruption.json` | notebook workspace | `partial_corruption` | `guided_repair` |
| `preview_cache_broken_cache_shard.json` | preview cache | `broken_cache_shard` | `rebuild_automatically` |
| `generated_artifact_stale_overlay.json` | generated artifacts | `stale_derived_overlay` | `rebuild_automatically` |
| `sync_journal_preserved_unsaved_work.json` | sync journal | `journal_preserved_unsaved_work` | `guided_repair` |
| `trust_policy_quarantined.json` | trust policy | `quarantined_trust_state` | `fail_closed_privileged_operations` |

## Export posture

Every support-export row produced from this packet keeps:

- `raw_payload_excluded = true`
- `ambient_authority_excluded = true`
- explicit `preserved_context`, `blocked_capabilities`, and
  `actions`
- support-safe summaries for both `intact_state_summary` and
  `safest_route_rationale`
