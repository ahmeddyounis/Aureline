# Proof packet: preview / apply / revert on one destructive core path

Purpose: anchor proof captures for the M1 bounded prototype wedge that
lands `propose -> preview -> apply -> validate -> revert` on one
destructive core path (multi-target bulk replace). The wedge mints a
named undo group plus a content-addressed checkpoint per apply, refuses
to widen scope when the basis drifts, and reuses the shared mutation /
local-history vocabulary from `aureline-history`.

Reviewer landing page:
[`docs/ux/m1_preview_apply_revert_core_path.md`](../../../../docs/ux/m1_preview_apply_revert_core_path.md).

## Canonical sources

- Crate (consumer + projection): `crates/aureline-shell/`
  - `src/review_preview/mod.rs` — `MutationPacket` record, lifecycle
    state machine (`DestructiveCoreEngine`), per-phase records,
    admission / revert / consequence vocabularies, deterministic
    plaintext render.
  - `src/review_preview/tests.rs` — unit tests for the protected walk,
    scope-drift failure drill, target-missing drill, no-matches drill,
    preview-skipped drill, and the keep path.
  - `tests/destructive_core_preview.rs` — fixture-driven integration
    test for the protected walk and the scope-drift failure drill.
- Crate (shared vocabulary): `crates/aureline-history/`
  - `src/lib.rs` — `body_object_id` helper used to mint stable basis /
    proposed-post-apply digests without persisting blobs first.
  - `src/checkpoints/mod.rs` — `LocalHistoryStore::objects_root_path`
    accessor used by the wedge to rehydrate pre-apply bytes on revert.
- Fixtures: `fixtures/ux/preview_apply_cases/destructive_core_path/`
  - `protected_walk.json`
  - `scope_drift_blocks_apply.json`
- Reviewer doc: `docs/ux/m1_preview_apply_revert_core_path.md`

## Upstream contracts the wedge projects against (without forking)

- `docs/ux/preview_apply_revert_contract.md` — frozen
  `preview_apply_revert_phase`, `revert_class`, basis-drift, and
  consequence-class vocabulary.
- `docs/ux/shell_interaction_safety_contract.md` — consequence class
  and the "no silent widening after preview" rule.
- `crates/aureline-history/` and the mutation-journal / local-history
  schemas — the wedge writes one `MutationJournalEntryRecord` per
  target plus one `MutationGroupRecord` per apply / per revert, and
  one `LocalHistoryGroupRecord` per checkpoint, all bound to the named
  undo-group and checkpoint-group ids.

## Protected walk

Three workspace-local targets seeded with text containing `legacy_fn`.
A `workspace.bulk_replace_in_files.apply` proposal renames `legacy_fn`
to `modern_fn` across all three targets. Lineage IDs (`packet_id`,
`proposal_id`, `preview_id`, `apply_id`, `mutation_group_id`,
`local_history_group_id`, `validation_id`, `revert_id`) bind through
every phase. After `validate` confirms `all_targets_matched = true`,
`revert` restores each target byte-for-byte from the checkpoint and
records a reverse `MutationGroupRecord` with
`group_resolution = reverted`.

Evidence:

- `crates/aureline-shell/src/review_preview/tests.rs::protected_walk_propose_preview_apply_validate_revert_keeps_lineage_visible`
- `crates/aureline-shell/tests/destructive_core_preview.rs::fixture_drives_full_protected_walk`
- Fixture: `fixtures/ux/preview_apply_cases/destructive_core_path/protected_walk.json`

## Failure drill — scope drift refuses to widen apply

A clean preview is computed across two targets. An external actor then
mutates one target (`src/router.rs`). Reopening the preview surfaces
the typed `basis_drifted` block reason on the drifted row, transitions
the row's `basis_drift_state` to `drift_detected`, and refuses
admission with `blocked_by_basis_drift { drifted_target_count = 1 }`.
A subsequent `apply` call returns
`WedgeError::ApplyBlocked(BlockedByBasisDrift { drifted_target_count: 1 })`
and the packet stays in `PreviewApplyRevertPhase::Preview` until the
user re-reviews.

Evidence:

- `crates/aureline-shell/src/review_preview/tests.rs::failure_drill_blocks_apply_when_basis_drifts_after_preview`
- `crates/aureline-shell/tests/destructive_core_preview.rs::fixture_drives_scope_drift_failure_drill`
- Fixture: `fixtures/ux/preview_apply_cases/destructive_core_path/scope_drift_blocks_apply.json`

## Adjacent failure drills

- `target_missing_blocks_admission_with_typed_reason` — apply is
  refused with `blocked_by_target_missing { missing_target_count = 1 }`
  and the row carries `target_missing` so the wedge cannot collapse
  absence into the drift bucket.
- `no_matches_blocks_admission_without_widening_scope` — apply is
  refused with `blocked_by_no_matches` when the search pattern finds
  zero occurrences, so the wedge never applies an empty diff.
- `apply_is_rejected_when_preview_is_skipped` — apply requires a
  preview; a caller cannot bypass review.
- `keep_retires_a_validated_packet_without_a_revert` — `Keep`
  transitions the packet to `PreviewApplyRevertPhase::Keep`, and a
  subsequent `revert` is refused with `AlreadyKept`.

## Validation command

```
cargo test -p aureline-shell --lib review_preview \
  && cargo test -p aureline-shell --test destructive_core_preview \
  && cargo test -p aureline-history
```

## Evidence storage

- Crate sources: `crates/aureline-shell/src/review_preview/`,
  `crates/aureline-shell/tests/destructive_core_preview.rs`,
  `crates/aureline-history/src/lib.rs`,
  `crates/aureline-history/src/checkpoints/mod.rs`
- Reviewer doc: `docs/ux/m1_preview_apply_revert_core_path.md`
- Fixtures: `fixtures/ux/preview_apply_cases/destructive_core_path/`
