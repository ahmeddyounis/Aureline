# Cache / storage-class governance lineage — contract

This document describes the cache / storage-class governance lineage
record: the workspace's governed, export-safe projection that
finalizes how Aureline manages caches, derived stores, and durable
workspace state — and proves that the eviction rules and cleanup
surfaces never lose user state silently.

Where the portable-state lineage proves *how restored state preserves
provenance* and the reactive-state lineage proves *how materialized
views stay aligned with their authority epoch*, this record proves the
*storage layer* underneath both: which storage classes exist, which
user-state class each one carries, which eviction policy each one
declares, which cleanup surfaces the user can reach to clear them, and
which inspection / repair hooks must fire before any destructive
cleanup commits.

The record is the single artifact every consuming surface (workspace
cache / storage status, settings storage panel, command-palette
cleanup actions, support cleanup tool, Help/About, headless CLI)
ingests instead of cloning status text.

## Input

The projection ingests a live
[`CacheStorageClassInputs`](../../../crates/aureline-workspace/src/cache_storage_class_lineage/mod.rs)
envelope verbatim. The envelope carries one
[`CacheStorageObservation`](../../../crates/aureline-workspace/src/cache_storage_class_lineage/mod.rs)
per governed storage class (ephemeral memory cache, local disk cache,
derived index, durable workspace state, recovery checkpoint, local
history, prebuild artifact cache, support-export staging). Each
storage row records the user-state class, eviction policy, claimed
durability tier, bound cleanup surfaces, the
silent-clear-blocked / requires-pre-action-inspection /
preserve-on-quota-pressure flags, and the support-export projection.

For determinism and replay, the projection accepts the same envelope
shape the fixtures and the headless emitter consume.

## What the record proves

- **Storage-class coverage truth.** Every governed storage class ships
  a row bound to one closed
  `storage_class_kind` (`ephemeral_memory_cache`, `local_disk_cache`,
  `derived_index`, `durable_workspace_state`, `recovery_checkpoint`,
  `local_history`, `prebuild_artifact_cache`,
  `support_export_staging`). The corpus seeds one row per required
  class so the user never lands on a release where a
  user-state-bearing class slipped past governance.
- **Eviction-policy truth.** Each row declares one closed
  `eviction_policy_class`; the projection re-derives the worst-case
  durability tier from the user-state class and the eviction class so
  an `lru`/`ttl_age`/`quota_pressure` policy cannot ride on
  user-authored or user-derived content.
- **No user-state loss honesty.** Every storage class carrying user
  state declares an eviction policy from the closed safe set
  (`never`, `manual_only`, `manual_after_export`) and explicitly
  marks `silent_clear_blocked`, `requires_pre_action_inspection`, and
  `preserve_on_quota_pressure`. Each user-state-bearing row binds at
  least one cleanup surface so the user can reach it.
- **Cleanup-surface coverage truth.** The union of cleanup surfaces
  observed across the corpus covers the required set (`settings_panel`,
  `command_palette`, `support_cleanup_tool`, `help_about`,
  `headless_cli`). The `crash_recovery_panel` is optional and only
  appears on recovery-touching rows.
- **Claimed durability tier truth.** Each row declares one closed
  `claimed_durability_tier`; the projection re-derives the worst-case
  tier from the eviction policy and user-state class so a
  `stable_durable_user_state` claim cannot ride on an `lru` policy.
- **Pre-action inspection-hook honesty.** A controlled set of
  pre-action inspection / repair hooks (`inspect_cache`,
  `compare_before_clear`, `export_before_clear`, `rollback_clear`,
  `export`, `repair`) is reachable so destructive cleanups stay
  reviewable.
- **Support-export honesty.** Each row's support-export projection
  preserves the storage class, user-state class, eviction policy,
  claimed tier, cleanup surfaces, and silent-clear-blocked flag while
  excluding raw cache content, raw secrets, approval tickets,
  delegated credentials, and live authority handles.
  User-state-bearing rows must declare a non-`local_only` posture so
  support bundles can preserve the state.

In addition the record carries the producer ref, the schema version,
the capture timestamp, and an integrity hash so import / replay
surfaces can pin the source producer before applying.

## Closed vocabularies

| Field                       | Tokens                                                                                                                                  |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `storage_class_kind`        | `ephemeral_memory_cache`, `local_disk_cache`, `derived_index`, `durable_workspace_state`, `recovery_checkpoint`, `local_history`, `prebuild_artifact_cache`, `support_export_staging` |
| `user_state_class`          | `no_user_state`, `regenerable_with_cost`, `user_authored`, `user_derived`                                                               |
| `eviction_policy_class`     | `never`, `manual_only`, `manual_after_export`, `lru`, `ttl_age`, `quota_pressure`, `restart_drop`                                       |
| `cleanup_surface_kind`      | `settings_panel`, `command_palette`, `support_cleanup_tool`, `help_about`, `headless_cli`, `crash_recovery_panel`                       |
| `claimed_durability_tier`   | `stable_durable_user_state`, `stable_regenerable_cache`, `stable_ephemeral_cache`, `narrowed_below_stable`                              |
| `support_export_posture`    | `local_only`, `metadata_safe_export`, `held_record`                                                                                     |
| `inspection_hook_class`     | `inspect_cache`, `compare_before_clear`, `export_before_clear`, `rollback_clear`, `export`, `repair`                                    |

## Narrow reasons

When a claim cannot be proven on the captured corpus the record
auto-narrows below Stable with a named reason.

| Narrow reason                              | Fires when                                                                                                              |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- |
| `corpus_empty`                             | The envelope carried no storage observations                                                                            |
| `required_storage_class_missing`           | The corpus omitted at least one of the six required storage classes                                                     |
| `required_cleanup_surface_missing`         | The union of cleanup surfaces does not cover `settings_panel`, `command_palette`, `support_cleanup_tool`, `help_about`, `headless_cli` |
| `user_state_with_unsafe_eviction`          | A user-state-bearing storage row declared an eviction policy outside `never` / `manual_only` / `manual_after_export`    |
| `user_state_cleanup_not_blocked`           | A user-state-bearing storage row did not declare `silent_clear_blocked = true`                                          |
| `user_state_inspection_not_required`       | A user-state-bearing storage row did not declare `requires_pre_action_inspection = true`                                |
| `user_state_quota_pressure_unsafe`         | A user-state-bearing storage row did not declare `preserve_on_quota_pressure = true`                                    |
| `user_state_cleanup_surface_missing`       | A user-state-bearing storage row did not bind at least one cleanup surface                                              |
| `durability_tier_mismatch_derived`         | The declared durability tier did not match the worst-case re-derived tier                                               |
| `inspection_hook_unavailable`              | A required pre-action inspection / repair hook was unavailable                                                          |
| `support_export_fields_dropped`            | A storage row's support-export projection dropped one of the required cache / storage fields                            |
| `support_export_redaction_unsafe`          | A storage row declared `raw_cache_content_excluded = false`, `raw_secrets_excluded = false`, `approval_tickets_excluded = false`, `delegated_credentials_excluded = false`, or `live_authority_handles_excluded = false` |
| `support_export_posture_unsafe`            | A user-state-bearing storage row declared `local_only` support export                                                   |
| `producer_attribution_incomplete`          | Producer attribution fields were empty (producer ref / captured-at)                                                     |
| `lineage_export_unsafe`                    | Workspace ref or corpus ref was empty (would break support export)                                                      |

## Inspection hooks

A destructive cleanup never fires without an inspection hook the user
can reach first.

| Hook class             | Default action id                            | Purpose                                                                                                                       |
| ---------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `inspect_cache`        | `cache_storage.inspect_cache`                | Opens the cache inspector with the storage class, user-state class, eviction policy, and the captured cleanup surfaces        |
| `compare_before_clear` | `cache_storage.compare_before_clear`         | Renders the diff between the live storage row and the baseline so the user can review what cleanup will drop before it fires |
| `export_before_clear`  | `cache_storage.export_before_clear`          | Exports the storage row's user-visible state into a support-safe artifact before any cleanup commits                          |
| `rollback_clear`       | `cache_storage.rollback_clear`               | Captures a one-step rollback so the user can revert a cleanup if a downstream surface relied on the cleared row              |
| `export`               | `cache_storage.export`                       | Exports the lineage record (support-safe, no raw cache content, secrets, approval tickets, or delegated credentials)         |
| `repair`               | `cache_storage.repair`                       | Opens the repair sheet for a degraded storage class rather than clearing as a shortcut                                       |

## Replay gate

Every fixture under
[`/fixtures/workspace/m4/cache_storage_class_lineage/`](../../../fixtures/workspace/m4/cache_storage_class_lineage/)
carries the posture inputs and the expected projected record. The
replay gate at
[`/crates/aureline-workspace/tests/cache_storage_class_lineage_replay.rs`](../../../crates/aureline-workspace/tests/cache_storage_class_lineage_replay.rs)
re-projects each input and asserts the result equals the checked-in
`expected`, so the projection cannot drift from the canonical record
without failing CI. The gate also asserts each fixture is
support-export safe and that the corpus covers Stable plus a
narrowed-below-Stable posture (specifically: a user-state-bearing row
that misfiles its eviction policy, and a posture missing a required
inspection hook).
