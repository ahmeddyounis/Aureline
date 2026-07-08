# Git Mutation Review Sheets, Conflict Checkpoints, and Force-Push Dialogs: Target and Recovery Truth

- Packet: `m5-git-mutation-review-recovery-component:stable:0001`
- Surface: `Git mutation review sheets, conflict checkpoints, and force-push dialogs: target and recovery truth`
- Cherry-pick/revert sheets: 2; patch-apply sheets: 1; conflict cards: 2; force-push dialogs: 1
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Cherry-pick / revert review sheets

- `cherry_pick` `a1b2c3d` "fix: guard review-queue projection" onto `main` — updates_hosted_review (pre_mutation_captured)
- `revert` `b2c3d4e` "feat: risky telemetry probe" onto `release/1.4` — local_only (reflog_fallback_only)

## Patch-apply review sheets

- `mailbox_series` via `three_way_merge` onto `feature/import` — 3 files / 2 commits (pre_mutation_captured)

## Conflict-checkpoint cards

- **Revert of b2c3d4e on release/1.4** on `release/1.4` — 1/1 unresolved (reopenable)
- **Merge of feature/review-lane into main** on `main` — 0/2 unresolved (resolved_applied)

## Force-push review dialogs

- `force_with_lease` → `origin/feature/review-lane` overwrites `e5f6a7b` with `f6a7b8c` (2 commits, invalidates_approval)
