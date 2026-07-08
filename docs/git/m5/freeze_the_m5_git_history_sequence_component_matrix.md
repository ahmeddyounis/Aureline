# Git-History and Risky-Mutation Component Matrix (contract)

This document is the human-readable contract for the frozen Git-history and
history-surgery component family. The machine-readable boundary is the schema and
the checked support export:

- Schema: `schemas/ui/m5-git-history-sequence-component-matrix.schema.json`
- Support export: `artifacts/release/m5-git-history-sequence-proof/support_export.json`
- Summary: `artifacts/release/m5-git-history-sequence-proof/summary.md`
- Design note: `artifacts/design/m5-git-history-sequence-component-matrix.md`
- Typed model: `crates/aureline-git/src/freeze_the_m5_git_history_sequence_component_matrix/`

## Purpose

Commit graphs, worktrees, stashes, interactive rebases, cherry-picks/reverts,
patch applies, conflict checkpoints, and force pushes are distinct product
surfaces that had been redrawn per screen. This matrix makes the reusable
components those surfaces render — and the identity, recovery, approval, and
handoff truth they must preserve — one governed product family instead of
per-screen copy.

The matrix is the single source of truth for whether an M5 Git-history surface
may reuse a shared component. It references the canonical commit-history,
repository-topology, stash, recovery-checkpoint, sequence-edit, history-surgery,
conflict-session, and ref-update contracts by id instead of redefining them, so
review, shell, help, support, export, provider-overlay, AI-context, and CLI
flows all read one vocabulary.

## What the matrix governs

### Components

Each of the twelve components binds a canonical source contract, the exact
identity it must preserve, the recovery checkpoint/destination that stays
reachable, the approval-invalidation rule it honors, the browser/provider
handoff boundary it respects, the shared downgrade vocabulary it may surface,
and the mutation-review class its Git verb requires.

| Component | Binds contract | Mutation review |
| --- | --- | --- |
| Commit graph header | `git_history_review` | display only |
| History graph row | `git_history_review` | display only |
| Branch comparison chip | `repository-topology` | display only |
| Worktree row | `repository-topology` | display only |
| Stash entry | `stash_entry` | stash restore confirm |
| Reflog recovery banner | `recovery_checkpoint` | display only |
| Rebase todo row | `sequence_edit_session` | sequence rewrite confirm |
| Sequence editor header | `sequence_edit_session` | sequence rewrite confirm |
| Cherry-pick / revert review sheet | `history-surgery-review` | explicit verb confirm |
| Patch-apply review sheet (beta) | `history-surgery-review` | patch apply confirm |
| Conflict checkpoint card | `conflict_session` | display only |
| Force-push review dialog (preview) | `sequence-edit-...-ref-update` | force push confirm |

Each component names, in the frozen row:

- **Identity preservation** — the exact repo/worktree/ref/commit identity it
  keeps visible; verbs and worktrees never collapse.
- **Recovery checkpoint rule** — the checkpoint or reflog-only destination that
  stays reachable (display components reference the recovery surface rather than
  owning it).
- **Approval-invalidation rule** — how an invalidated approval is surfaced; it is
  never a silent or generic warning.
- **Browser/provider handoff rule** — the explicit boundary at which local truth
  hands off to a provider view; local truth is never presented as the provider's.

### Downgrade vocabulary

`stale_provider_overlay`, `detached_or_missing_ref`,
`dirty_or_conflicted_worktree`, `shallow_or_partial_topology`,
`reflog_only_fallback`, `approval_invalidated`, and `offline_local_only`. Each
state narrows a claim (it is never reduced to a single badge), and the
reflog-only fallback must stay visible after a risky mutation.

## Invariants enforced

The typed `validate` mirrors the schema and adds the cross-row rules a schema
cannot express:

- Every frozen component is present exactly once and binds its canonical source
  contract by id.
- Every component names its identity, recovery, approval, and handoff rules.
- A risky, history-mutating component carries a real (non-display) mutation-review
  class and keeps its Git verb distinct; a display component claims no risky verb.
- Every downgrade state narrows a claim; the reflog-only fallback stays visible
  after a mutation.
- Review, shell, help, support/export, CLI, and provider overlay can all express
  the family and its downgrade rules.
- The export carries no raw boundary material.

## Out of scope

This lane does not redesign Git execution backends or provider APIs. It freezes
the reusable component family the broader M5 Git-history surfaces build on.
