# Finalize Cache and Storage-Class Governance, Eviction Rules, and Cleanup Surfaces Without User-State Loss — proof packet

Reviewer-facing proof packet for the cache / storage-class governance
lane: every governed storage class (ephemeral memory cache, local
disk cache, derived index, durable workspace state, recovery
checkpoint, local history, plus the optional prebuild artifact cache
and support-export staging classes) is bound to one closed
user-state class and eviction policy. The projection re-derives the
worst-case durability tier from those declarations so a
`stable_durable_user_state` claim cannot ride on an `lru`, `ttl_age`,
or `quota_pressure` policy. User-state-bearing rows must declare
`silent_clear_blocked`, `requires_pre_action_inspection`, and
`preserve_on_quota_pressure`, and bind at least one cleanup surface
from the required set (settings panel, command palette, support
cleanup tool, Help/About, headless CLI). A destructive cleanup never
fires without the controlled inspection / repair hook table
(`inspect_cache`, `compare_before_clear`, `export_before_clear`,
`rollback_clear`, `export`, `repair`) being reachable; a missing
hook narrows the record below Stable with a named reason. This
packet is the stable-line anchor for this lane; dashboards, docs,
Help/About surfaces, and support exports should ingest the typed
sources below rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/cache_storage_class_lineage/`](../../../crates/aureline-workspace/src/cache_storage_class_lineage/)
- Schema:
  [`/schemas/workspace/cache_storage_class_lineage.schema.json`](../../../schemas/workspace/cache_storage_class_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_cache_storage_class_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_cache_storage_class_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/cache_storage_class_lineage/`](../../../fixtures/workspace/m4/cache_storage_class_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/cache_storage_class_lineage_replay.rs`](../../../crates/aureline-workspace/tests/cache_storage_class_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/finalize-cache-and-storage-class-governance-eviction-rules.md`](../../../docs/workspace/m4/finalize-cache-and-storage-class-governance-eviction-rules.md)
- Typed consumer: `aureline_workspace::project_cache_storage_class_lineage`

## What this packet proves

1. **Storage-class coverage truth.** Each record carries one
   `storage_class_coverage` row per governed storage class declaring
   one closed `storage_class_kind`. A corpus missing any of the six
   required classes (`ephemeral_memory_cache`, `local_disk_cache`,
   `derived_index`, `durable_workspace_state`, `recovery_checkpoint`,
   `local_history`) narrows the record with
   `required_storage_class_missing`. Worked example:
   [`baseline_user_state_safe_stable.json`](../../../fixtures/workspace/m4/cache_storage_class_lineage/baseline_user_state_safe_stable.json).

2. **Eviction-policy truth.** Every storage row declares one closed
   `eviction_policy_class` (`never`, `manual_only`,
   `manual_after_export`, `lru`, `ttl_age`, `quota_pressure`,
   `restart_drop`). The projection re-derives the worst-case
   durability tier from the user-state class and eviction class.
   Worked example:
   [`local_history_lru_eviction_narrowed.json`](../../../fixtures/workspace/m4/cache_storage_class_lineage/local_history_lru_eviction_narrowed.json)
   downgrades the local-history row to `narrowed_below_stable`
   because an `lru` policy cannot ride on a user-derived class.

3. **No user-state loss honesty.** Every storage row carrying user
   state (`user_authored` or `user_derived`) must declare an eviction
   policy from the safe set, mark `silent_clear_blocked = true`,
   `requires_pre_action_inspection = true`,
   `preserve_on_quota_pressure = true`, and bind at least one cleanup
   surface. Violations narrow with one of
   `user_state_with_unsafe_eviction`,
   `user_state_cleanup_not_blocked`,
   `user_state_inspection_not_required`,
   `user_state_quota_pressure_unsafe`, or
   `user_state_cleanup_surface_missing`.

4. **Cleanup-surface coverage truth.** The union of cleanup surfaces
   observed across the corpus must cover the required set
   (`settings_panel`, `command_palette`, `support_cleanup_tool`,
   `help_about`, `headless_cli`). The optional
   `crash_recovery_panel` rides on recovery checkpoints. A corpus
   that drops a required surface narrows with
   `required_cleanup_surface_missing`.

5. **Claimed durability tier truth.** Each row's declared tier
   (`stable_durable_user_state`, `stable_regenerable_cache`,
   `stable_ephemeral_cache`, `narrowed_below_stable`) is re-derived
   from the user-state and eviction classes. A mismatch narrows with
   `durability_tier_mismatch_derived`. Worked example:
   [`local_history_lru_eviction_narrowed.json`](../../../fixtures/workspace/m4/cache_storage_class_lineage/local_history_lru_eviction_narrowed.json)
   surfaces both `user_state_with_unsafe_eviction` and
   `durability_tier_mismatch_derived` after the downgrade.

6. **Inspection precedes destructive cleanup.** The controlled
   inspection / repair hook table must be available before any
   destructive cleanup commits. A missing hook narrows with
   `inspection_hook_unavailable`. Worked example:
   [`missing_export_before_clear_hook_narrowed.json`](../../../fixtures/workspace/m4/cache_storage_class_lineage/missing_export_before_clear_hook_narrowed.json)
   demonstrates the narrow path when `export_before_clear` is
   unavailable.

7. **Support-export honesty.** Each row's support-export projection
   must preserve `storage_class`, `user_state_class`,
   `eviction_policy`, `claimed_durability_tier`, `cleanup_surfaces`,
   and `silent_clear_blocked`, redact raw cache content, raw secrets,
   approval tickets, delegated credentials, and live authority
   handles, and (for user-state-bearing rows) declare a
   non-`local_only` posture. Dropping a field narrows with
   `support_export_fields_dropped`; raising raw material narrows with
   `support_export_redaction_unsafe`; a user-state-bearing row
   shipping `local_only` narrows with
   `support_export_posture_unsafe`.

8. **Producer attribution is pinnable for replay.** Each record
   carries the producer ref, the schema version, the capture
   timestamp, and an integrity hash derived from the input identities
   so replay and support pipelines can pin the source before
   applying. Incomplete attribution narrows with
   `producer_attribution_incomplete`.

9. **Lineage and export stay honest.** Every record sets
   `raw_payload_excluded = true` and carries only opaque refs to the
   source workspace, corpus, and producer. An empty workspace or
   corpus ref narrows with `lineage_export_unsafe`.

10. **The record is replay-gated.** The replay gate re-projects each
    fixture and asserts it equals the checked-in `expected`, so the
    projection cannot drift without failing CI.

## Fixture corpus

| Fixture                                                  | Workspace state covered                    | Qualification           | Proves                                                                                                                            |
| -------------------------------------------------------- | ------------------------------------------ | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `baseline_user_state_safe_stable`                        | Six required storage classes governed      | `stable`                | A baseline release-branch corpus can prove the full contract                                                                      |
| `extended_with_prebuild_and_support_staging_stable`      | Adds prebuild cache + support staging      | `stable`                | The optional classes ride safely on the same projection                                                                           |
| `local_history_lru_eviction_narrowed`                    | Local history downgraded to `lru` policy   | `narrowed_below_stable` | The projection refuses to let user-derived content sit behind an `lru` policy and surfaces both the eviction and tier narrows     |
| `missing_export_before_clear_hook_narrowed`              | `export_before_clear` hook unavailable     | `narrowed_below_stable` | The contract refuses to ship Stable when a required pre-action hook is missing                                                    |

## How to verify

```sh
# Unit + replay gate for the cache / storage-class lineage projection.
cargo test -p aureline-workspace --lib cache_storage_class_lineage
cargo test -p aureline-workspace --test cache_storage_class_lineage_replay

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_cache_storage_class_lineage -- --lines \
  fixtures/workspace/m4/cache_storage_class_lineage/baseline_user_state_safe_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and
replay gate above. The lineage record self-describes its stable
qualification: storage rows that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named
reason, so they never inherit an adjacent green row. No public scope
is widened from this row.
