# Session-restore proposal cases

Reference fixtures that pin the canonical pre-rehydration restore proposal
shape produced by `aureline_recovery::session_restore::proposal::RestoreProposal`.

These cases anchor the truth vocabulary used by recovery surfaces to summarize
what an abnormal-termination relaunch will return as live state, what stays as
a placeholder skeleton, and what is retained as evidence only — before any
rehydration runs.

## Cases

- `no_restore_first_launch.json` — Empty state on first launch; no crash
  journal, no topology snapshot. The proposal classifies as `no_restore` and
  invents no counts.
- `layout_only_clean_relaunch.json` — Clean shutdown of a one-tab editor
  layout; class `layout_only` returns the skeleton without drafts.
- `recovered_drafts_after_crash.json` — Abnormal termination with one dirty
  editor buffer alongside a terminal pane; class `recovered_drafts`.
  Terminal is `blocked_side_effectful` (never auto-rerun).
- `evidence_only_corrupt_snapshot.json` — Abnormal termination with a
  checksum-mismatched frame; class narrows to `evidence_only` so the original
  on-disk truth and journaled evidence are preserved without auto-restore.

## Honesty invariants

Every case must keep these intact:

1. `auto_rerun_forbidden` is always `true` — restore must never silently
   rerun terminals, debuggers, notebook kernels, AI panels, or remote
   sessions.
2. `counts` reflects only what is *persisted*, never speculative state.
3. `restore_class` matches what `RestoreProposal::build` would return for the
   same persisted artifacts.
4. Side-effectful pane roles map to `blocked_side_effectful`; lightweight
   editor surfaces map to `live_skeleton`.
