# History-rewrite, stash, reflog, and conflict-session known limits

This document records the explicit limits of the beta history-rewrite
lane covered by the conformance corpus at
`fixtures/git/m3/history_rewrite_corpus/`. It is the authoritative
list of surfaces that MAY appear in the IDE but MUST NOT claim beta
conformance until the listed dependency lands.

## In scope for the beta corpus

- Local rebase, interactive rebase (including autosquash / fixup /
  squash / drop verbs), cherry-pick, revert, reset, patch-apply,
  stash apply / pop / drop / promote-to-branch.
- Continue / skip / abort lifecycle vocabulary on conflict sessions
  with a captured recovery checkpoint or an explicitly acknowledged
  reflog-only fallback.
- Sequence-edit session restart, alternate-worktree fallback, and the
  protected-branch / policy / collaboration block classes published by
  the ref-update-proposal vocabulary.
- Support-export packets that quote redaction-safe display labels and
  keep every `raw_*_export_allowed` flag false.

## Out of scope (do NOT claim beta)

- Hosted merge-queue execution and provider-side history mutation
  beyond the local / worktree boundary. Provider integrations remain
  read-only block-reason consumers; provider-side enforcement is the
  provider's responsibility.
- Cross-repository history sharing (subtree promotion, submodule
  rewrite, repository-wide filter-repo style operations).
- Multi-user collaborative editing of an in-flight rebase or
  cherry-pick sequence. A `collaboration_active_session` block on the
  ref-update proposal is the safe wedge until that lane lands.
- Provider-driven force-push approval flows beyond local approval
  ticket linkage (the `request_approval` next-safe-path class).
- Auto-resolution of conflicts via AI or rule-based merge tooling.
  External-tool handoff is the supported surface; auto-resolution
  remains a future track.
- Time-travel debugging on rewritten history. The recovery checkpoint
  contract is the supported rollback path; time-travel is explicitly
  deferred.

## Negative invariants the corpus guards

The corpus' negative drills exist to make these regressions visible
without manual review:

- A reflog-only acknowledgement MUST NOT be re-labeled as a captured
  recovery checkpoint. (`negative.reflog_relabeled_as_checkpoint`)
- A `promoted_to_branch` stash MUST keep its `promoted_branch_ref`
  populated. (`negative.lost_stash_provenance`)
- A force-move MUST NOT proceed without a captured recovery
  checkpoint. (`negative.force_move_applied_with_reflog_only`)
- A blocked ref-update proposal MUST always publish at least one
  next-safe path. (`negative.blocked_proposal_without_next_safe_path`)
- Raw paths, raw branch names, raw patch bodies, raw reflog bodies,
  and raw stash bodies MUST NOT cross the support-export boundary.
  (`negative.raw_patch_body_exported`)
- An `applied` ref-update proposal MUST NOT carry an active
  non-`no_block` block. (`negative.silent_widen_scope_apply_still_blocked`)

A drift in any of these invariants fails the conformance harness;
the corresponding drill report points at the exact regression class.

## Verification

```
cargo test -p aureline-qe --test git_history_rewrite_conformance
```

The harness exits non-zero with a per-drill failure list when the
corpus regresses. Treat any failure as a beta release blocker for the
risky Git lane.
