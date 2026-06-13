# State-class recovery routing and placeholder continuity

This document freezes the shared state-class recovery packet implemented by
[`crates/aureline-reactive-state/src/state_class_recovery/mod.rs`](../../crates/aureline-reactive-state/src/state_class_recovery/mod.rs),
consumed by the support-export envelope in
[`crates/aureline-support/src/state_class_recovery/mod.rs`](../../crates/aureline-support/src/state_class_recovery/mod.rs),
and composed with the existing restore-placeholder contract in
[`crates/aureline-shell/src/restore/provenance.rs`](../../crates/aureline-shell/src/restore/provenance.rs).

The checked-in truth lives in:

- [`artifacts/state/state_class_recovery.json`](../../artifacts/state/state_class_recovery.json)
- [`artifacts/state/state_class_recovery.md`](../../artifacts/state/state_class_recovery.md)
- [`fixtures/state/state_class_recovery/`](../../fixtures/state/state_class_recovery/)
- [`schemas/state/state_class_recovery.schema.json`](../../schemas/state/state_class_recovery.schema.json)

## Frozen vocabulary

The packet closes the vocabulary for the following axes:

- `state_class`: `durable_user_state`, `workspace_state`,
  `derived_cache_state`, `generated_artifact_state`,
  `recovery_journal_state`, `security_trust_state`
- `surface_family`: `notebook_workspace`, `request_workspace`,
  `provider_draft`, `preview_cache`, `generated_artifacts`,
  `sync_journal`, `trust_policy`
- `authority_class`: `user_authored_durable_truth`,
  `user_owned_recovery_state`, `admin_or_control_artifact`,
  `disposable_derived_cache`
- `primary_recovery_route` and `fallback_recovery_routes`:
  `rebuild_automatically`, `guided_repair`,
  `rollback_to_preserved_artifact`,
  `fail_closed_privileged_operations`
- `supported_failure_modes`: `partial_corruption`,
  `missing_dependency`, `stale_derived_overlay`,
  `broken_cache_shard`, `journal_preserved_unsaved_work`,
  `quarantined_trust_state`
- `placeholder_kind`: `context_only`, `stale_derived_artifact`,
  `recovered_draft`, `rollback_ready`, `privileged_blocked`
- `preserved_context`: `layout_slot`, `logical_identity`,
  `workspace_chrome`, `mutation_lineage`, `draft_edits`,
  `last_known_metadata`, `retained_evidence`
- `blocked_capabilities`: `live_execution`, `direct_write`,
  `managed_publish`, `regeneration_claims`, `background_refresh`,
  `privileged_apply`
- `placeholder_actions`: `start_repair_flow`,
  `compare_preserved_artifact`, `restore_from_rollback`,
  `retry_rebuild`, `reauthenticate_managed_surface`,
  `open_without_artifact`, `export_support_packet`,
  `remove_placeholder`

## Recovery model

The packet keeps three recovery questions distinct:

1. **Which class failed?**  
   Durable user state, workspace state, derived cache state, generated
   artifact state, recovery journal state, and trust or security state stay
   separate.
2. **Which route is safest first?**  
   Rebuild, guided repair, rollback, or fail-closed posture is selected per
   class instead of through a broad reset.
3. **What stays visible while recovery runs?**  
   Placeholder continuity preserves the layout slot, logical identity, and the
   right surrounding context for the affected family.

This separation is the key contract. It prevents generated-artifact drift from
being treated like durable user data loss, and it prevents trust-state damage
from pretending that routine local editing is also unsafe.

## Family coverage

The checked-in packet proves seven recovery families:

| Family | Class | Primary route | Placeholder posture |
| --- | --- | --- | --- |
| `provider_local_draft` | durable user state | `rollback_to_preserved_artifact` | `rollback_ready` |
| `request_workspace` | workspace state | `guided_repair` | `context_only` |
| `notebook_workspace` | workspace state | `guided_repair` | `context_only` |
| `preview_cache` | derived cache state | `rebuild_automatically` | `stale_derived_artifact` |
| `generated_artifacts` | generated artifact state | `rebuild_automatically` | `stale_derived_artifact` |
| `sync_journal` | recovery journal state | `guided_repair` | `recovered_draft` |
| `trust_policy` | security/trust state | `fail_closed_privileged_operations` | `privileged_blocked` |

## Contract rules

- Every covered family names one primary recovery route and at least one
  fallback route.
- Durable user state and workspace state never rebuild automatically.
- Derived caches and generated artifacts prefer rebuild because authoritative
  truth lives elsewhere.
- Recovery journals preserve drafts and compare targets instead of silently
  replaying or discarding queued work.
- Trust and policy corruption fail closed for privileged operations while
  keeping ordinary local editing and support export available.
- Placeholder continuity preserves the layout slot and logical identity for
  every family row so restore stays spatially honest even when the live family
  is missing, stale, quarantined, or under review.
