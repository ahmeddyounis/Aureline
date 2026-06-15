# Pin / retention manager and cleanup-history contract (M5 heavy artifact families)

The pin / retention manager is the operator-facing object the shell shows when
the user asks *why is this still on disk?* For every pinned or retained artifact
— crash evidence, review packs, docs / model / template packs, certified
templates, checkpoints, and incident bundles — it states:

- **the pin source** — the frozen `pin_source_class` that holds it;
- **who pinned it** — the `pin_actor` derived from that source;
- **the expiry / policy window** — the `retention_state` and, for a finite
  window, an `expires_at`;
- **the referenced object** — the object whose reference keeps it on disk;
- **the unpin path** — how the pin is released;
- **the export path** — the export-before-delete path offered before any delete.

The sibling **cleanup-history lane** keeps every past eviction attributable after
the fact: who ran it, what triggered it, which class and family it touched, how
many bytes it reclaimed, which pins blocked it, and the stale / reindex-needed
state it left behind — without ever capturing a raw payload.

The canonical product object is `m5_pin_retention_manager`, owned by
`crates/aureline-support/src/m5_pin_retention` and bound to the boundary schema
at `schemas/storage/m5_pin_retention.schema.json`. It mints no new storage
primitive: the storage-class, artifact-family, and pin-source vocabularies
re-export verbatim from `artifacts/runtime/storage_classes.yaml` and the frozen
matrix at `artifacts/storage/m5_artifact_family_storage_matrix.yaml`. The
matrix-backed composer `compose_manager` folds that matrix, so the manager and
the storage-governance lane can never disagree about which storage class or pin
sources a family carries. The runtime contract names the `pin_manager` and
`cleanup_history_lane` surfaces directly; every manager binds both.

## Pin invariants

A pin entry is admissible only when it holds every invariant below; the
validator in `m5_pin_retention` and the schema both enforce them, and the
scenario corpus under `fixtures/storage/m5_pin_retention_cases/` exercises them.

1. **The pin actor is derived from the pin source.** `pin_actor` is a pure
   function of `pin_source` (user, admin policy, release / case / review /
   offline / certified-template / support-export process, or retention policy).
2. **The unpin path is derived from the pin source.** An explicit user pin is
   unpinned directly; an admin / policy pin requires an admin / policy change; a
   reference (release / case / review / offline / certified / support) is cleared
   by releasing the referencing object; a retention-window pin auto-unpins at
   expiry.
3. **A finite window is the only state that carries an expiry.** `expires_at` is
   present exactly when `retention_state` is `in_retention_window`. The other
   states (`pinned_indefinite_while_referenced`, `retained_until_explicit_reset`,
   `pinned_by_explicit_user_choice`, `policy_window_managed`) carry no expiry.
4. **The export path tracks the protection posture.** A protected entry
   (evidence or user-owned recovery) requires export before delete; a disposable
   or rebuildable entry offers it; an entry already inside a support-export
   assembly reports `export_already_in_assembly`.
5. **Protection tracks the storage class.** `protected_continuity` is true
   exactly for the evidence and user-owned recovery classes.
6. **The referenced object matches the source.** `referenced_object_class` is
   consistent with the pin source (a review pin references a review packet, an
   offline pin an offline bundle, a retention pin a retention window, and so on).

## Cleanup-history invariants

7. **Cleanup is attributable and payload-free.** Every event carries an
   `actor_class`, `trigger_class`, `family_id`, `storage_class_id`,
   `disposition`, `reclaimed_bytes`, blocked-pin set, and `resulting_state`.
   `authoritative_state_touched` and `raw_payload_captured` are always `false`.
8. **Blocked pins are recorded, not hidden.** `blocked_pin_count` is at least the
   number of `blocked_pin_sources`, and the source list is non-empty exactly when
   a pin blocked the cleanup. A `blocked_no_op_pin_protected` event reclaims zero
   bytes, records at least one blocking pin, and leaves `partial_retained_pins`
   or `authoritative_state_untouched`.
9. **Storage pressure never deletes user-owned recovery state.** When a
   `low_disk_pressure` or `managed_quota_pressure` trigger targets the
   user-owned recovery class, the disposition is `blocked_no_op_pin_protected`
   and `reclaimed_bytes` is zero. The only cleanup that may reclaim recovery
   bytes is an explicit, `exported_then_deleted` user action.
10. **Evidence expiry is class- and trigger-scoped.** An
    `expired_unpinned_evidence_past_retention` event only targets the evidence
    class and only fires on retention expiry or storage pressure; pinned and
    in-window evidence is retained.
11. **Reindex / rebuild state is honest.** `reindex_needed` is true exactly when
    `resulting_state` is `reindex_needed`; a `trimmed_rebuildable_cache` event
    leaves `rebuild_pending` or `reindex_needed`.

## Surfaces and actions

12. **The manager binds both surfaces and offers a way forward.** `surfaces`
    lists both `pin_manager` and `cleanup_history_lane`, and every manager
    carries `open_inspector_action_ref = action.storage.open_inspector` and
    `open_clear_data_review_action_ref = action.storage.open_clear_data_review`,
    so pin / retention state is never a dead end and protected-state removal
    always routes through the class-selective review.

## Support export

`PinRetentionManagerCorpus::support_export` projects the corpus into a
metadata-safe envelope (`m5_pin_retention_support_export`) the support-bundle
pipeline quotes without leaking raw payloads, paths, or credentials. It counts
managers, pins, protected pins, cleanup events, blocked-pin events, and — always
zero — authoritative-state-loss and raw-payload-capture events. The checked-in
golden lives at `fixtures/storage/m5_pin_retention/support_export.golden.json`
and is regenerated with
`cargo run -p aureline-support --example dump_m5_pin_retention_support_export`.
The scenario corpus is regenerated from the seeded composer signals with
`cargo run -p aureline-support --example dump_m5_pin_retention_seeded_managers`.
