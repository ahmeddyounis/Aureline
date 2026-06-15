# Storage-pressure banner contract (M5 heavy artifact families)

The storage-pressure banner is the operator-facing object the shell shows when
low-disk or managed-quota pressure narrows a surface. It replaces a silent trim
with an honest, inspectable disclosure that states:

- **which pressure class fired** — the runtime governor tier (`constrained`,
  `degraded`, `protect_core`) that maps to the low-disk floor reached;
- **what is exerting the pressure** — a low-disk floor or a managed / workspace /
  per-class quota ceiling;
- **what background work paused** before any deletion;
- **the next eviction order** — the full frozen low-disk ladder, with each step's
  disposition and whether it was applied at this pressure;
- **which classes stay protected** and were left untrimmed;
- **the open-inspector and open-review actions** that move the user forward.

The canonical product object is `m5_storage_pressure_banner`, owned by
`crates/aureline-support/src/m5_storage_pressure` and bound to the boundary
schema at `schemas/storage/m5_storage_pressure.schema.json`. It mints no new
storage primitive: the storage-class and low-disk-ladder vocabularies re-export
verbatim from `artifacts/runtime/storage_classes.yaml`, and the eviction order
follows the frozen sequence drilled in `artifacts/runtime/low_disk_drills.yaml`.
The matrix-backed composer `compose_banner` folds the frozen artifact-family
matrix at `artifacts/storage/m5_artifact_family_storage_matrix.yaml`, so the
banner and the storage-governance matrix can never disagree about the ladder.

## Invariants

A banner is admissible only when it holds every invariant below; the validator
in `m5_storage_pressure` and the schema both enforce them, and the scenario
corpus under `fixtures/storage/m5_storage_pressure_cases/` exercises them.

1. **Eviction follows the frozen sequence.** `eviction_order` lists every
   ladder step exactly once, in the frozen order, with each step's
   `ladder_order`, `disposition`, `target_class_id`, and `protected` flag
   derived from the runtime contract. No step is skipped, reordered, or hidden.
2. **Pressure tiers auto-apply a bounded prefix.** `constrained` applies
   through `trim_interactive_hot_cache` (step 3), `degraded` through
   `trim_prebuild_environment_unpinned` (step 6), and `protect_core` through
   `expire_unpinned_evidence_past_retention` (step 7). No step past the tier's
   ceiling may carry `applied = true`. `current_ladder_step` equals the deepest
   applied step.
3. **User-owned recovery state is never auto-trimmed.** Step 8
   (`user_owned_recovery_state_only_under_explicit_review`) carries
   `requires_reviewed_escalation = true` and is never `applied`. Its
   `state_loss_guard` always reclaims zero bytes and uses
   `user_owned_recovery_state_never_auto_trimmed` or
   `escalation_required_not_auto_applied`.
4. **Evidence retains pinned and in-window entries.** Outside `protect_core`,
   the evidence guard is `protected_evidence_fully_retained` with zero reclaimed
   bytes. At `protect_core`, only unpinned evidence past retention may expire
   (`unpinned_evidence_expired_pinned_and_in_window_retained`).
5. **Protected classes are listed as not trimmed.** `user_owned_recovery_state`
   always appears in `protected_class_ids_not_trimmed`; `evidence_support_cache`
   appears whenever the tier did not reach the evidence-expiry step. Only
   protected classes may appear in that list.
6. **Paused work is disclosed.** `paused_work` always includes the two ladder
   pause steps (`speculative_fetch_and_prefetch`,
   `managed_replication_and_pack_refresh`) and carries no duplicates.
7. **No authoritative state loss.** `authoritative_state_loss` is always
   `false`, every `state_loss_guard` holds, and a pending escalation
   (`reviewed_escalation_required_not_yet_approved`) never coexists with
   reclaimed protected bytes — managed quota or disk pressure never silently
   deletes local user-owned state.
8. **The banner offers the inspector and review.** Every banner carries
   `open_inspector_action_ref = action.storage.open_inspector` and
   `open_clear_data_review_action_ref = action.storage.open_clear_data_review`,
   so pressure is never a dead end and protected state removal always routes
   through the class-selective review.

## Support export

`StoragePressureBannerCorpus::support_export` projects the corpus into a
metadata-safe envelope (`m5_storage_pressure_support_export`) the support-bundle
pipeline quotes without leaking raw payloads, paths, or credentials. It counts
pressure events, escalations pending review, and — always zero —
authoritative-state-loss events. The checked-in golden lives at
`fixtures/storage/m5_storage_pressure/support_export.golden.json` and is
regenerated with
`cargo run -p aureline-support --example dump_m5_storage_pressure_support_export`.
