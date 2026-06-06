# Stable risky-VCS truth fixtures

These fixtures are the canonical corpus for
`schemas/review/sequence-edit-conflict-session-stash-entry-ref-update.schema.json`
and `crates/aureline-git::finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth`.

- `stable_risky_vcs_lineage.json` covers a full lineage across conflict session, sequence-edit session, stash entry, recovery checkpoint, ref-update proposal, command bindings, and support export.
- `reflog_only_reset_disclosed.json` proves missing checkpoints stay labelled as reflog-only disclosure instead of being relabelled as captured checkpoints.
- `blocked_invalidated_ref_update.json` proves invalidated approvals/checks block provider publication while support export remains reconstructable.
