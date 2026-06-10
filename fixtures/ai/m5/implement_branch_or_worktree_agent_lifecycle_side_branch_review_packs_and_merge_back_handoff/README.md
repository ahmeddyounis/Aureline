# Branch or Worktree Agent Lifecycle Fixtures

This directory contains fixture files for the branch or worktree agent
lifecycle, side-branch review pack, and merge-back handoff lane.

## Files

- `valid_packet.json` — A fully valid lifecycle packet that passes all
  validation invariants. Mirrors the checked-in support export.
- `merge_back_without_approval.json` — A packet whose merge-back reached its
  destination (`merged_by_human`) without human approval, triggering
  `merge_back_without_approval`.
