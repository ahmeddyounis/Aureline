# M5 Git-history and sequence component matrix (design)

Design-side inventory for the frozen Git-history and risky-mutation component
family (task M05-956, batch B113). The canonical, machine-readable truth is the
schema + support export; this note records the design intent and consumer map.

- Schema: `schemas/ui/m5-git-history-sequence-component-matrix.schema.json`
- Support export: `artifacts/release/m5-git-history-sequence-proof/support_export.json`
- Contract doc: `docs/git/m5/freeze_the_m5_git_history_sequence_component_matrix.md`
- Fixtures: `fixtures/ui/m5-git-history-sequence-components/`

## Component inventory (Appendix C alignment)

Twelve reusable components, each mapped to one canonical source contract so no
surface re-derives Git-history truth:

1. **commit_graph_header** — commit-graph view header (repo root, ref anchor, range).
2. **history_graph_row** — one commit row within the history graph.
3. **branch_comparison_chip** — ahead/behind comparison of two exact refs + base.
4. **worktree_row** — one linked worktree, its path, branch, and repo root.
5. **stash_entry** — one stash shelf entry with its restore scope.
6. **reflog_recovery_banner** — the reflog-based recovery destination surface.
7. **rebase_todo_row** — one interactive-rebase todo line and its verb.
8. **sequence_editor_header** — the interactive-rebase session frame.
9. **cherry_pick_revert_review_sheet** — pre-run review for cherry-pick or revert.
10. **patch_apply_review_sheet** (beta) — pre-run review for patch/mailbox apply.
11. **conflict_checkpoint_card** — conflict checkpoint captured mid-mutation.
12. **force_push_review_dialog** (preview) — ref-rewrite review before a force push.

## Truth each component must preserve

Every row carries four named rules, enforced by `validate`:

- **Identity** — exact repo/worktree/ref/commit; cherry-pick vs revert and
  worktree-vs-worktree never collapse.
- **Recovery** — the checkpoint or reflog-only destination that stays reachable;
  display components reference, risky components own, the recovery.
- **Approval invalidation** — surfaced explicitly, never as a generic warning.
- **Browser/provider handoff** — the explicit boundary where local truth hands
  off to a provider view; local-only recovery stays explicit.

## Consumer map

All twelve components are consumed, unchanged, by: review, shell, help, support,
support/export, provider overlay, CLI, and AI context. Help/support/export
packets point at this one family and its shared downgrade vocabulary
(`stale_provider_overlay`, `detached_or_missing_ref`,
`dirty_or_conflicted_worktree`, `shallow_or_partial_topology`,
`reflog_only_fallback`, `approval_invalidated`, `offline_local_only`).

## Maturity

Ten components are Stable. `patch_apply_review_sheet` is Beta and
`force_push_review_dialog` is Preview while their risky-mutation review flows
harden; both are reusable today but carry the narrowest claims, and stale review
auto-narrows the family per the freeze posture.
