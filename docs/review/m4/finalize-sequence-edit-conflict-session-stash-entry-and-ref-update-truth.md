# Finalize sequence-edit, conflict-session, stash-entry, ref-update, and recovery-checkpoint truth

**Status:** Stable risky-VCS truth lane — implemented in `crates/aureline-git`.

## Goal

Risky VCS work is represented by durable objects that can be reopened after restart, executed by CLI/headless flows, handed to provider/browser paths, and exported to support without reconstructing meaning from raw logs or one-off dialogs.

## Stable object vocabulary

| Object | Stable truth |
| --- | --- |
| Conflict session | Repo/worktree ref, operation kind, base/ours/theirs refs, affected path set, unresolved count, structured/raw mode, previous-session and start/update lineage. |
| Sequence-edit session | Target ref, ordered operation list, todo version state, raw todo ref, structured card ref, protected-branch posture, unresolved blockers, and checkpoint ref. |
| Stash entry | Entry ID, creator, message, path scope, untracked-state posture, source repo/worktree, checkpoint refs, and distinct apply/pop/drop/branch-from command classes. |
| Recovery checkpoint | Trigger kind, source refs, root-state hash, expiry, restore options, and explicit reflog-only disclosure when a checkpoint is not possible. |
| Ref-update proposal | Local and remote ref sets, divergence summary, approval state, check invalidation state, publish mode, explicit target selection, protected-branch posture, rollback hint, and rollback checkpoint ref. |

## Invariants

- All objects carry stable IDs and timestamps so restart, support export, and audit lanes can reopen the same truth.
- Conflict sessions preserve base/ours/theirs provenance and do not treat structured-to-raw downgrade as resolution.
- Raw todo text and structured sequence cards must reference the same sequence session.
- Stash apply, pop, drop, and branch-from remain distinct commands against one stable entry.
- Destructive mutations are checkpoint-backed or explicitly labelled as reflog-only before apply.
- Ref-update proposals cannot be ready/applied while target selection is ambiguous or approvals/checks are invalidated.
- Support export includes every object ID in lineage and keeps all raw export flags false.

## Files

- Implementation: `crates/aureline-git/src/finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth/mod.rs`
- Schema: `schemas/review/sequence-edit-conflict-session-stash-entry-ref-update.schema.json`
- Fixtures: `fixtures/review/m4/finalize-sequence-edit-conflict-session-stash-entry-and-ref-update-truth/`
- Tests: `crates/aureline-git/tests/finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth.rs`
- Proof packet: `artifacts/review/m4/finalize-sequence-edit-conflict-session-stash-entry-and-ref-update-truth.md`

## Verification

```bash
cargo test -p aureline-git --test finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth
```
