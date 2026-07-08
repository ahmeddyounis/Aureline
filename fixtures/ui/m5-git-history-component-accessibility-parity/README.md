# M5 Git-history component accessibility, headless, and export parity fixtures

Protected fixtures for the M05-962 shared Git-history component accessibility lane
(`implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_repo_topology_checkpoint_availability_or_provider_linked_recovery_truth_is_partial_or_stale_across_claimed_m5_git_history_components`).

Each fixture is a full `GitHistoryComponentAccessibilityPacket` that validates against
`schemas/ui/m5-git-history-component-accessibility-parity.schema.json` and proves that
every claimed component stays keyboard-reachable, screen-reader labelled, CLI/export
legible, and never semantically stronger on desktop than in CLI or support output —
while automatically narrowing its recovery / mutation-safety claim when repo topology,
checkpoint availability, or provider-linked recovery truth is partial or stale.

- `repo_topology_partial_and_checkpoint_unavailable_narrowed.json` — a commit-graph
  header under partial repo topology and a worktree row with checkpoint recovery
  unavailable, each auto-narrowed with the incomplete-history and reflog recovery
  destination kept spelled out.
- `provider_review_stale_and_offline_local_only_narrowed.json` — a commit-graph header
  with provider-linked review state stale and a force-push dialog dropping to offline /
  local-only, each auto-narrowed with local continuation kept explicit.

Regenerate with
`GEN_GIT_HISTORY_ACCESSIBILITY_ARTIFACTS=1 cargo test -p aureline-git --lib regenerate_git_history_component_accessibility_artifacts`.
