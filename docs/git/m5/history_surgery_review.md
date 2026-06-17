# History-surgery review sheets

Risky history mutations — rebase, cherry-pick, revert, reset, patch-apply, and
force-push — cannot stay command-only escape hatches that tell users to drop to
the shell. This contract turns each verb into a durable, serde-serializable review
sheet that names the exact repository-or-worktree target and target ref it would
mutate, surfaces the pre-execution gate states a user must see before
Continue / Skip / Publish, keeps the raw rebase-todo or patch text inspectable,
and keeps a recovery path visible. If a user can run a risky mutation, Aureline
can preview it, attribute it, and explain why it was allowed, blocked, or
downgraded.

- Schema: [`schemas/git/history-surgery-review.schema.json`](../../../schemas/git/history-surgery-review.schema.json)
- Canonical packet: [`artifacts/git/m5/history_surgery_review/history_surgery_review_sheets.json`](../../../artifacts/git/m5/history_surgery_review/history_surgery_review_sheets.json)
- Fixtures: [`fixtures/git/m5/rebase-cherry-pick-reset/`](../../../fixtures/git/m5/rebase-cherry-pick-reset)
- Code: `crates/aureline-git/src/history_surgery_review/`
- Review projection: `crates/aureline-review/src/git_history_review/`

The verbs are governed by the frozen M5 repository-topology and history-surgery
matrix; this lane is their reviewable, decision-bearing implementation. The verbs
stay **distinct**: a generic "rewrite history" sheet would hide the exact verb,
target, policy, and recovery semantics users need, so it is never produced.

## Review sheet

Each `git_history_surgery_review_sheet` records one risky verb against one target
rather than transient modal state:

| Field | Purpose |
|-------|---------|
| `verb` | Which risky verb this sheet reviews (rebase, cherry-pick, revert, reset, patch-apply, force-push). |
| `repo_ref` + `worktree_ref` + `target_kind` | Exact repository-or-worktree target truth. |
| `primary_target_ref` | The exact, unambiguous ref the mutation would move or rewrite. |
| `secondary_refs` | Verb-specific refs: rebase onto/base, cherry-pick/revert source commits, force-push remote ref. |
| `reset_mode` | Reset mode (soft/mixed/hard/keep/merge), for reset sheets only. |
| `force_lease_ref` + `divergence_class` | Force-with-lease expected old value and divergence class, for force-push sheets only. |
| `protected_branch_posture` | Protected-branch gate. |
| `stale_review_state` | Stale-review / approval-invalidation gate. |
| `merge_queue_state` | Merge-queue gate. |
| `dirty_worktree_state` | Dirty-worktree gate. |
| `conflict_source_state` | Conflict-source gate (whether unresolved conflicts block Continue). |
| `provider_overlay_state` | Provider-overlay freshness/availability gate. |
| `raw_source_text_ref` + `structured_cards_ref` | Refs to the inspectable raw todo/patch text and the structured cards derived from it. |
| `checkpoint_lineage_refs` + `reflog_only_fallback` | The recovery path: an explicit checkpoint or an acknowledged reflog-only fallback. |
| `local_actions` | Local-first actions kept available offline (preview, abort, restore, …). |
| `decision` | The derived allow/block/downgrade decision. |

## Derived decision

The `decision` is never a free-form badge. It is a deterministic function of the
gate states, so a stored decision can be re-derived and verified. Each gate
yields a verdict — *clear*, *downgrade*, or *block* — and the sheet's outcome is
the worst verdict across all gates:

- **Blocked** — at least one gate blocks: unresolved conflict, blocked/read-only
  protected branch or policy lock, merge-queue invalidation, an invalidated
  approval, a dirty worktree that blocks the operation, no recovery path at all,
  or missing raw source text for a verb that requires it.
- **Downgraded** — no gate blocks but at least one narrows: structured parsing
  failed so only the raw todo/patch is inspectable, only a reflog-only fallback
  remains, the worktree needs an autostash, the merge-queue entry will drop, or
  the provider overlay is stale or unavailable.
- **Allowed** — every gate is clear and a recovery path is visible.

Two invariants are encoded in the derivation rather than left to prose:

- A risky mutation is **never allowed** without a visible recovery path.
- A **provider outage never blocks** local truth — the provider-overlay gate can
  only downgrade a sheet to local-only, so local preview, abort, and restore stay
  available offline even when the provider is unreachable.

## Review, support, and export projection

The review crate's `git_history_review` lane projects each sheet onto the
surfaces that must explain a risky mutation — the review pane, the CLI/headless
result packet, the redaction-safe support export, the provider overlay, and AI
context. Each `git_history_review_decision_row` restates the same decision
(outcome, reason, contributing gates, recovery visibility) and records whether the
surface may execute the mutation. Only a mutation surface (review or CLI/headless)
may execute, and only for an allowed decision; read-only surfaces restate the
decision but never mark it executable. Because every row is the deterministic
projection of its sheet and the embedded sheets are validated through this Git
contract, the review and support/export flows always agree on why a risky
mutation was allowed, blocked, or downgraded.

## Boundary

Raw paths, raw patch/todo bodies, raw provider payloads, and credentials never
cross the support boundary; only redaction-safe refs do. The support export
retains the identity fields needed to reconstruct each sheet (verb, repo/worktree,
target, decision outcome and reason, recovery visibility) and asserts that raw
paths, patch bodies, and provider payloads are redacted.
