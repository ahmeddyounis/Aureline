# Rebase Todo Rows and Sequence-Editor Headers: Ordered-Plan and Checkpoint Truth

- Packet: `m5-rebase-todo-sequence-editor-component:stable:0001`
- Surface: `Rebase todo rows and sequence-editor headers: ordered-plan and checkpoint truth`
- Todo rows: 5 (1 dropped, 1 with unresolved blockers); sequence headers: 2
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Rebase todo rows

- `pick` [orig #0 → #0] `a1b2c3d` "add review-queue projection scaffold" — unchanged (captured)
- `squash` [orig #1 → #1] `b2c3d4e` "fold: review-queue projection tests" — squashed_into_previous (per_step_captured)
- `reword` [orig #2 → #3] `c3d4e5f` "rename queue projection to review lane" — reordered (captured)
- `pick` [orig #3 → #2] `d4e5f6a` "wire review lane into shell" — reordered (captured)
- `drop` [orig #4 → #4] `e5f6a7b` "spike: throwaway telemetry probe" — dropped (reflog_fallback_only)

## Sequence-editor headers

- **Interactive rebase: tidy review-lane history** onto `main` (recover from `feature/review-lane@{1}`) — 5 commits: 2 reordered / 1 squashed / 1 dropped
- **Interactive rebase: review only, no edits** onto `main` (recover from `feature/docs@{2}`) — 3 commits: 0 reordered / 0 squashed / 0 dropped
