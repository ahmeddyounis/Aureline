# Stash Entries and Reflog-Recovery Banners: Restore-Scope and Checkpoint Truth

- Packet: `m5-stash-reflog-recovery-component:stable:0001`
- Surface: `Stash entries and reflog-recovery banners: restore-scope and checkpoint truth`
- Stash entries: 2 (1 include untracked content); reflog banners: 3 (2 still reachable)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Stash entries

- `stash@{0}` — "WIP: extract review-queue projection" from `feature/review-queue` [scope `tracked_and_staged`]: verbs apply/pop/drop/create_branch_from_stash
- `stash@{1}` — "spike: local-only telemetry buffer" from `main` [scope `tracked_staged_untracked`]: verbs apply/pop/drop/create_branch_from_stash

## Reflog-recovery banners

- **Force-push rewrote main** → `main@{1} (pre-force-push tip a1b2c3d)` [reachable, expiry `fresh`]
- **Interactive rebase squashed 4 commits** → `HEAD@{5} (pre-rebase tip e4f5a6b)` [reachable, expiry `expiring_soon`]
- **Amend replaced the previous checkpoint** → `` [superseded, expiry `expired`]
