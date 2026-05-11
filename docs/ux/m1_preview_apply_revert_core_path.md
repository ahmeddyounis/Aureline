# M1 preview / apply / revert on one destructive core path

This page is the reviewer-facing landing page for the bounded prototype
wedge that proves the **worktree-is-sacred** rule end-to-end on one
destructive core path. The wedge lives at
[`crates/aureline-shell/src/review_preview/`](../../crates/aureline-shell/src/review_preview/)
and the integration test lives at
[`crates/aureline-shell/tests/destructive_core_preview.rs`](../../crates/aureline-shell/tests/destructive_core_preview.rs).

The wedge is bounded: it lands one destructive surface (multi-target
bulk replace), reuses the shared mutation / local-history vocabulary from
[`aureline_history`](../../crates/aureline-history/), and does not
generalize into a platform-wide mutation framework. Out-of-scope items
(AI apply flows, cross-surface preview standardization, share / export
boundary moves) are deliberately not addressed here; this page documents
what the seed proves and what it explicitly does not.

## What the wedge owns

- A single canonical `MutationPacket` record carrying:
  - `packet_id`, `proposal_id`, `preview_id`, `apply_id`,
    `mutation_group_id`, `local_history_group_id`, `validation_id`,
    `revert_id` — the stable lineage ids that survive every phase
    transition.
  - One per-phase record (`ProposalRecord`, `PreviewRecord`,
    `ApplyRecord`, `ValidateRecord`, `RevertRecord`) so a reviewer or
    support export can quote the exact basis, admission, and recovery
    claim the wedge advertised.
- An honest revert-class vocabulary that mirrors
  [`docs/ux/preview_apply_revert_contract.md`](preview_apply_revert_contract.md):
  - `restore_from_checkpoint` is the declared and realized class for the
    nominal apply path;
  - `compensating_action` is the realized class when the checkpoint can
    only partially restore the target world.
- A typed `ApplyAdmissibility` decision (`admitted`,
  `blocked_by_basis_drift { drifted_target_count }`,
  `blocked_by_target_missing { missing_target_count }`,
  `blocked_by_no_matches`) so a refused apply names the reason rather
  than silently widening scope.
- A `DestructiveCoreEngine` that runs the lifecycle, mints undo-group
  and checkpoint ids through `aureline_history::MutationJournalStore`
  and `aureline_history::LocalHistoryStore`, and persists one
  `MutationJournalEntryRecord` per target plus one
  `MutationGroupRecord` per apply / per revert. Each apply also writes a
  `LocalHistoryGroupRecord` checkpoint that the revert path restores
  byte-for-byte.
- A deterministic plaintext `MutationPacket::render_plaintext()` block
  that surfaces every phase, every lineage id, and the realized revert
  class so support exports never hide what changed.

## Protected walk

Open the destructive core wedge against three targets seeded by
[`fixtures/ux/preview_apply_cases/destructive_core_path/protected_walk.json`](../../fixtures/ux/preview_apply_cases/destructive_core_path/protected_walk.json):

1. **Propose** — declare scope (`src/launch.rs`, `src/router.rs`,
   `docs/intro.md`), capture the basis digest for each target, and
   advertise `consequence_class =
   destructive_reversible_with_checkpoint` plus `revert_class =
   restore_from_checkpoint`. Lineage:
   `packet_id`, `proposal_id` minted.
2. **Preview** — compute the per-target diff against the captured basis
   and confirm `apply_admissibility = admitted`,
   `overall_basis_drift_state = no_drift`,
   `total_match_count = 4` across the three rows. Lineage:
   `preview_id` minted.
3. **Apply** — mint `mutation_group_id` and `local_history_group_id`,
   write one `MutationJournalEntryRecord` per target (grouped by the
   undo-group id), write one `LocalHistoryEntryRecord` per target
   (grouped by the checkpoint group), then swap the world's bytes for
   each target to the post-apply text. Lineage: `apply_id`,
   `mutation_group_id`, `local_history_group_id` minted.
4. **Validate** — re-read every target and confirm its observed digest
   equals the digest the preview advertised. Lineage: `validation_id`
   minted, `all_targets_matched = true`.
5. **Revert** — restore each target from its pre-apply body object,
   write a reverse mutation-group record (`group_resolution = reverted`,
   `realized_revert_class = restore_from_checkpoint`), and downgrade the
   realized class to `compensating_action` if any target could not be
   restored. Lineage: `revert_id`, `reverse_mutation_group_id` minted.

The protected walk is exercised by
[`fixture_drives_full_protected_walk`](../../crates/aureline-shell/tests/destructive_core_preview.rs)
and by
[`protected_walk_propose_preview_apply_validate_revert_keeps_lineage_visible`](../../crates/aureline-shell/src/review_preview/tests.rs).

## Failure drill — basis-drift refuses to widen scope

After a clean preview, an external actor mutates one of the previewed
targets (`src/router.rs`). The protected walk reopens the preview
against the drifted basis and confirms:

- the row's `basis_drift_state` transitions to `drift_detected` and
  carries both the previous and current basis digests verbatim;
- the row's `blocked_reason` reads `basis_drifted`;
- the preview's overall `apply_admissibility` reads
  `blocked_by_basis_drift { drifted_target_count }` with the matching
  count;
- the call to `DestructiveCoreEngine::apply` returns
  `WedgeError::ApplyBlocked(BlockedByBasisDrift { drifted_target_count })`
  rather than silently writing.

The packet stays in `PreviewApplyRevertPhase::Preview` until the user
re-reviews the new mutation basis. This is the spec's protected promise:
**apply MUST NOT silently widen or materially change scope after
preview**.

The failure drill is exercised by
[`fixture_drives_scope_drift_failure_drill`](../../crates/aureline-shell/tests/destructive_core_preview.rs)
and by
[`failure_drill_blocks_apply_when_basis_drifts_after_preview`](../../crates/aureline-shell/src/review_preview/tests.rs).
The fixture lives at
[`fixtures/ux/preview_apply_cases/destructive_core_path/scope_drift_blocks_apply.json`](../../fixtures/ux/preview_apply_cases/destructive_core_path/scope_drift_blocks_apply.json).

## Adjacent failure drills

- `target_missing_blocks_admission_with_typed_reason` — apply is
  refused with `blocked_by_target_missing { missing_target_count }` and
  the row carries `target_missing` rather than collapsing the absence
  into the drift bucket.
- `no_matches_blocks_admission_without_widening_scope` — apply is
  refused with `blocked_by_no_matches` when the search pattern finds no
  occurrences, so the wedge cannot apply an empty diff.
- `apply_is_rejected_when_preview_is_skipped` — apply is refused with
  `NotInPhase { expected: preview }` so callers cannot bypass review.
- `keep_retires_a_validated_packet_without_a_revert` — calling
  `keep` after validate transitions the packet to
  `PreviewApplyRevertPhase::Keep`, and a subsequent `revert` is refused
  with `AlreadyKept`. Lineage stays bound; the recovery class is
  honoured.

## Shared contracts the wedge projects against

The seed reuses these existing truth sources without forking:

- [`docs/ux/preview_apply_revert_contract.md`](preview_apply_revert_contract.md)
  — the frozen `preview_apply_revert_phase`, `revert_class`, basis-drift,
  and consequence-class vocabulary the wedge mirrors verbatim.
- [`docs/ux/shell_interaction_safety_contract.md`](shell_interaction_safety_contract.md)
  — consequence class, phase, and the "no silent widening" rule.
- [`crates/aureline-history/`](../../crates/aureline-history/) — the
  shared mutation-journal and local-history vocabularies. The wedge
  writes through `MutationJournalStore` and `LocalHistoryStore`; it does
  not mint a parallel undo-group or checkpoint scheme.
- [`docs/ux/save_review_sheet.md`](save_review_sheet.md) — the adjacent
  destructive save-resolution surface the wedge does not duplicate; both
  surfaces consume the same `revert_class` vocabulary.

## Out of scope (deliberately)

- AI patch apply flows. The composer / context-inspector seed is the
  bounded AI wedge; this seed does not pull AI-apply into its surface.
- Cross-surface preview standardization across rename, refactor,
  scaffolding, package install, migration, and recovery restore. Those
  surfaces remain owned by their own future wedges and read the same
  frozen `revert_class` vocabulary.
- Provider-backed or remote-authoritative mutations. The wedge is
  workspace-local and writes through the local history store only.
- Share / export / publish boundary moves. The wedge never crosses a
  trust boundary.

## Validation command

```
cargo test -p aureline-shell --lib review_preview \
  && cargo test -p aureline-shell --test destructive_core_preview \
  && cargo test -p aureline-history
```
