# Stash & recovery review sheets

Stash/shelf entries and reflog/checkpoint recovery cannot be the part of the Git
story that disappears the moment a provider is stale or a risky mutation only
half-completes. This contract turns each verb — stash apply, pop, drop, and
create-branch, plus reflog-restore and checkpoint-restore — into a durable,
serde-serializable review sheet that names the exact repository-or-worktree
target it would act on, surfaces the pre-execution gate states a user must see,
keeps the restore surface (anchor expiry, retention, compare / open-diff)
inspectable, and keeps a recovery path visible. If a user can run a stash or
recovery verb, Aureline can preview it, explain why it was allowed, blocked, or
downgraded, and keep the local continue / abort / inspect / restore truth
reachable even offline.

- Schema: [`schemas/git/stash-recovery.schema.json`](../../../schemas/git/stash-recovery.schema.json)
- Canonical packet: [`artifacts/git/m5/stash_recovery/stash_recovery.json`](../../../artifacts/git/m5/stash_recovery/stash_recovery.json)
- Fixtures: [`fixtures/git/m5/stash-recovery/`](../../../fixtures/git/m5/stash-recovery)
- Code: `crates/aureline-git/src/stash_recovery/`
- Review projection: `crates/aureline-review/src/git_stash_recovery_review/`

The verbs stay **distinct**. Apply, pop, drop, and create-branch differ in
whether they consume the stash entry — apply preserves it; pop, drop, and
create-branch consume it — so collapsing them would hide what a user is about to
lose. Reflog-restore and checkpoint-restore are likewise distinct: one is a
best-effort, expiry-bounded fallback and the other is a durable, retained anchor.

## Review sheet

Each `git_stash_recovery_sheet` records one verb against one target rather than
transient modal state:

| Field | Purpose |
|-------|---------|
| `verb` | Which verb this sheet reviews (stash apply/pop/drop/create-branch, reflog-restore, checkpoint-restore). |
| `repo_ref` + `worktree_ref` + `target_kind` | Exact repository-or-worktree target truth. |
| `primary_target_ref` | The exact, unambiguous ref the verb would act on. |
| `stash_entry_ref` + `stash_index` | The exact stash/shelf entry, for stash verbs. |
| `new_branch_ref` | The branch created from the entry, for create-branch only. |
| `recovery_anchor` | The reflog/checkpoint restore surface: anchor kind, ref, expiry instant, retention class, and compare / open-diff actions (recovery verbs only). |
| `stash_availability_state` | Whether the stash entry is still present (stash verbs). |
| `anchor_expiry_state` | Whether the recovery anchor is fresh, expiring, or expired (recovery verbs). |
| `dirty_worktree_state` | Dirty-worktree gate. |
| `conflict_source_state` | Conflict-source gate (whether an apply would conflict). |
| `provider_overlay_state` | Provider-overlay freshness/availability gate. |
| `checkpoint_lineage_refs` + `reflog_only_fallback` | The recovery path: an explicit checkpoint or an acknowledged reflog-only fallback. |
| `restore_caveats` | The caveats preserved when recovery is reflog-only or an anchor is expiring. |
| `local_actions` | Local-first actions kept available offline (preview, continue, abort, inspect, restore, compare, open-diff). |
| `decision` | The derived allow/block/downgrade decision. |

## Derived decision

The `decision` is never a free-form badge. It is a deterministic function of the
gate states, so a stored decision can be re-derived and verified. Each gate
yields a verdict — *clear*, *downgrade*, or *block* — and the sheet's outcome is
the worst verdict across all gates:

- **Blocked** — at least one gate blocks: the stash entry is missing/consumed, the
  recovery anchor has expired, an unresolved conflict would stop the apply, a
  dirty worktree blocks the operation, or there is no recovery path at all.
- **Downgraded** — no gate blocks but at least one narrows: only a reflog-only
  fallback remains, the recovery anchor is expiring soon, the worktree needs an
  autostash, or the provider overlay is stale or unavailable.
- **Allowed** — every gate is clear and a recovery path is visible.

Three invariants are encoded in the derivation rather than left to prose:

- A verb is **never allowed** without a visible recovery path.
- When only a reflog-based recovery exists, the sheet **preserves its caveats**
  (the entry can expire; the index/untracked state may not restore), so a restore
  never silently pretends to be a full checkpoint.
- A **provider or auth outage never blocks** local truth — the provider-overlay
  gate can only downgrade a sheet to local-only, so local preview, continue,
  abort, stash inspection, and checkpoint restore stay available offline even when
  the provider is unreachable.

## Review, support, and export projection

The review crate's `git_stash_recovery_review` lane projects each sheet onto the
surfaces that must explain and reach a stash or recovery verb — the review pane
(which also backs the Git history view and the command palette), the CLI/headless
result packet, the redaction-safe support export, the provider overlay, and AI
context. Each `git_stash_recovery_review_decision_row` restates the same decision
(outcome, reason, contributing gates, recovery visibility) and records whether the
surface may execute the verb. Only a mutation surface (review or CLI/headless) may
execute, and only for an allowed decision; read-only surfaces restate the decision
but never mark it executable. Because every row is the deterministic projection of
its sheet and the embedded sheets are validated through this Git contract, the
review and support/export flows always agree on why a verb was allowed, blocked,
or downgraded.

## Boundary

Raw paths, raw patch/diff bodies, raw provider payloads, and credentials never
cross the support boundary; only redaction-safe refs do. The support export
retains the identity fields needed to reconstruct each sheet (verb, repo/worktree,
target, decision outcome and reason, recovery visibility) and asserts that raw
paths, patch bodies, and provider payloads are redacted. Durable receipts and
restore anchors are required: stash and reflog recovery are never transient
toast-only behavior.
