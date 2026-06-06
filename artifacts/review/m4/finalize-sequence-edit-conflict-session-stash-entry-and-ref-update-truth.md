# Artifact: Stable risky-VCS truth across sequence edit, conflict, stash, ref update, and recovery lanes

**Status:** Implemented
**Verification class:** Automated functional + failure/recovery drill + conformance/interoperability + release evidence review

## Summary

This packet finalizes the stable object vocabulary for risky Git/VCS flows. The implementation defines one durable packet containing conflict-session, sequence-edit-session, stash-entry, recovery-checkpoint, ref-update-proposal, command-binding, support-export, and inspection records. UI, CLI/headless, provider handoff, restart restore, and support export consume the same packet rather than rebuilding state from logs or ad hoc dialogs.

## Deliverables

- Rust implementation: `crates/aureline-git/src/finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth/mod.rs`
- JSON schema: `schemas/review/sequence-edit-conflict-session-stash-entry-ref-update.schema.json`
- Fixtures: `fixtures/review/m4/finalize-sequence-edit-conflict-session-stash-entry-and-ref-update-truth/`
- Documentation: `docs/review/m4/finalize-sequence-edit-conflict-session-stash-entry-and-ref-update-truth.md`
- Tests: `crates/aureline-git/tests/finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth.rs`

## Acceptance Evidence

- Conflict-session objects preserve repo/worktree, operation, base/ours/theirs, affected paths, unresolved count, resolution mode, and start/update lineage.
- Sequence-edit objects bind raw todo text and structured cards to one session and enforce unique ordered operations.
- Stash entries preserve path scope, untracked posture, source repo/worktree, checkpoints, and distinct apply/pop/drop/branch-from commands.
- Recovery checkpoints preserve source refs, root-state hash, expiry, restore options, and explicit reflog-only disclosure when checkpoint capture is impossible.
- Ref-update proposals preserve local/remote ref sets, divergence, approval/check invalidation, publish mode, target selection, and rollback hints.
- Support export lineage includes all risky-VCS object IDs and forbids raw paths, branch names, patch bodies, reflog bodies, and stash bodies.

## Verification

```bash
cargo test -p aureline-git --test finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth
```

## Risks and Limits

- This is contract and validation code; it does not execute live Git mutations.
- The schema cannot express every cross-object invariant, so Rust validation remains the stricter executable contract.
- Provider-side enforcement still belongs to provider adapters; this packet preserves the target and rollback truth they must consume.
