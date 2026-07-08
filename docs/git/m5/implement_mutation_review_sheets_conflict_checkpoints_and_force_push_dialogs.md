# Cherry-pick/revert review sheets, patch-apply review sheets, conflict-checkpoint cards, and force-push review dialogs (M05-960)

This lane closes batch **B113** by narrowing the four remaining risky-mutation
review components frozen in
[`freeze_the_m5_git_history_sequence_component_matrix`](freeze_the_m5_git_history_sequence_component_matrix.md)
into an implemented, export-safe row contract:

- `cherry_pick_revert_review_sheet`
- `patch_apply_review_sheet`
- `conflict_checkpoint_card`
- `force_push_review_dialog`

The implementation lives in
`crates/aureline-git/src/implement_mutation_review_sheets_conflict_checkpoints_and_force_push_dialogs`
and produces a single [`GitMutationReviewPacket`] holding four co-equal vectors —
one per component — plus a shared downgrade vocabulary, consumer surfaces, trust
review, consumer projection, and proof-freshness block. The matrix vocabulary
([`M5GitHistoryComponent`], [`GitHistoryDowngradeState`],
[`ComponentConsumerSurface`], and [`MutationReviewClass`]) is reused directly, so
downgrades, parity, and each distinct confirm read the same everywhere.

## Honesty axes

The acceptance criteria for M05-960 are:

1. **No claimed M5 risky Git flow uses one ambiguous confirm button or hides the
   exact target ref/worktree being mutated.** Three of the four components drive a
   history-mutating verb (cherry-pick/revert, patch apply, force push). Each is
   validated through a shared `RiskySurfaceView` that requires:
   - the surface confirms as its own distinct mutation-review class
     (`explicit_verb_confirm`, `patch_apply_confirm`, or `force_push_confirm`) —
     never a shared confirm (`mutation_verb_confirm_collapsed`);
   - the exact target ref *and* worktree are named (`target_ref_worktree_missing`);
   - the publish/rewrite consequence is disclosed (`publish_consequence_missing`);
   - a rollback action and a reachable recovery checkpoint stay explicit
     (`rollback_action_missing`, `mutation_recovery_unreachable`).
   Cherry-pick and revert additionally must both be represented so the two verbs
   stay visibly distinct (`cherry_verb_coverage_missing`); the force-push dialog
   must name both tips and a recovery ref for the overwritten remote tip
   (`force_push_tips_missing`, `force_push_recovery_ref_missing`).

2. **Conflict and publish consequences remain explicit even when the same change
   also affects hosted review state.** Every risky surface carries a
   `HostedReviewImpact`. When the impact is anything other than `local_only`, the
   surface must disclose the approval consequence
   (`approval_consequence_missing`) instead of silently invalidating it — while
   the publish consequence is always required. The `conflict_checkpoint_card`
   keeps `base`/`ours`/`theirs`(/`result`) context, the unresolved count, and the
   reopen/restore behavior visible regardless of any provider-linked review; a
   card with unresolved conflicts must still offer a reopen path
   (`unresolved_conflict_not_reopenable`).

`resolve_mutation_review_disclosure(component, hosted_impact)` is the single
derivation that decides which of these disclosures are required, so the rules
cannot drift between surfaces.

## Recovery and read-only truth

The conflict-checkpoint card is the one read-only surface: it must claim
`display_only_no_mutation` (`conflict_card_claims_mutating_class` otherwise) and
never a mutating class. It preserves the captured conflict sides, keeps the
unresolved count within the total (`conflict_counts_inconsistent`), and always
discloses its checkpoint and reopen/restore behavior. This keeps local recovery
context alive after a risky mutation even when the mutation also updated a hosted
review.

## Artifacts

- Boundary schema: [`schemas/ui/m5-git-mutation-review-recovery-component.schema.json`](../../../schemas/ui/m5-git-mutation-review-recovery-component.schema.json)
- Release proof: `artifacts/release/m5-git-mutation-review-recovery-components-proof/`
  (`support_export.json` + `summary.md`)
- Protected fixtures: `fixtures/ui/m5-git-mutation-review-recovery-components/`

Regenerate the checked-in export, summary, and fixtures with:

    GEN_GIT_MUTATION_REVIEW_ARTIFACTS=1 cargo test -p aureline-git --lib \
      implement_mutation_review_sheets_conflict_checkpoints_and_force_push_dialogs::tests::generate_artifacts

The generator is the source of truth; do not hand-edit the export or fixtures.
