# Multi-window beta

This page describes the beta-grade workspace-management projection
that lives in `aureline-shell`. It freezes the acceptance states for
split-window, pane detach, cross-window move, and restore after
mixed-DPI / monitor / docking changes so the live shell, the review
packets in `fixtures/ux/m3/window_topology/`, and the support export
report the same workspace-management truth.

The projection is the page-level surface above the cross-window
transfer fixtures
([`docs/ux/cross_window_transfer_contract.md`](../cross_window_transfer_contract.md)),
the layout serialization records
([`docs/workspace/layout_serialization_contract.md`](../../workspace/layout_serialization_contract.md)),
the display-topology contract
([`docs/ux/window_display_contract.md`](../window_display_contract.md)),
and the restore-placeholder matrix
([`docs/ux/restore_placeholder_and_recenter_matrix.md`](../restore_placeholder_and_recenter_matrix.md)).
It does not re-derive any of that truth; it projects the four record
classes a beta review needs to inspect daily.

## Contract surface

The beta projection ships four record kinds, all under the shared
contract ref `shell:windows_beta:v1`:

- `shell_windows_beta_split_intent_record` — split-window intent.
  Carries the active window and pane, the split axis and side weights,
  the new pane ref, the focus owner after the split, the continuity
  cues that must remain visible (workspace authority, trust badge,
  host/remote badge, profile, recovery cues, role badge), the
  command-fallback ref, and the no-orphan invariants the surface must
  enforce (`no_silent_buffer_fork`, `focus_owner_in_split_tree`).
- `shell_windows_beta_detach_intent_record` — pane detach into a new
  window. Carries the source window/pane, the surface kind, the new
  window role and ref, the restore posture preserved, the continuity
  cues, and the no-orphan invariants
  (`dirty_state_visible_in_new_window`, `no_implicit_close_history`).
  A detach record must set `source_close_attributed_to_detach` to
  `true` so the source window's close history does not classify the
  detach as an accidental tab close.
- `shell_windows_beta_cross_window_move_intent_record` — cross-window
  move for tabs, editor groups, diff/review surfaces, and inspectors.
  Carries the source and target window refs, the surface kind, the
  source surface ref, the target slot, the continuity cues, the
  command-fallback ref, and the no-orphan invariants
  (`no_authority_swap`, `no_silent_restore_state_loss`,
  `no_orphan_on_source_close`). The record's
  `canonical_workspace_truth_preserved` field must be `true`.
- `shell_windows_beta_restore_topology_outcome_record` — restore
  outcome after a topology change. Carries the topology classes
  detected (`display_removed`, `safe_bounds_changed`, `scale_changed`,
  `wake_or_reconnect`, `docking_changed`), the typed adjustments
  applied (`snapped_to_safe_bounds`, `moved_to_primary_display`,
  `scale_normalized`, `fullscreen_cleared`, `dialog_recentered_to_owner`,
  `layout_only_fallback`), the resulting restore fidelity, the
  per-pane outcomes (live authority vs evidence-only placeholder),
  the recovery-critical chrome assurance, and the no-rerun invariants
  for any live-capability surface that did not hydrate.

A `shell_windows_beta_page_record` aggregates the four record lists
together with a summary banner (counts, layout-only outcomes,
layout-adjusted notes, recovery-chrome reachability). A
`shell_windows_beta_support_export_record` wrapper quotes the page
plus every `case_id` in stable order so support reviewers can pivot
from a row to the page without separate query plumbing.

## Acceptance posture

The beta projection delivers the M3 multi-window acceptance gates:

- **Cross-window move preserves canonical workspace truth** — tabs,
  editor groups, diff/review surfaces, and inspectors all carry the
  same workspace authority, trust state, host/remote state, profile,
  recovery cues, and role badge before and after a move. The
  validator (`validate_windows_beta_page`) rejects any move that
  drops canonical truth.
- **Restore preserves layout intent and clamps safely** — every
  restore outcome below `exact_restore` carries the user-visible
  "layout adjusted" note, an explicit topology-change class, and the
  typed adjustment that was applied. The validator rejects a
  downgraded restore that omits the note or an exact restore that
  still records adjustments.
- **Mixed-DPI / monitor removal / docking does not strand panes or
  hide recovery chrome** — every restore outcome promises that title
  context, restore details, command palette, keyboard focus, and the
  activity center remain reachable. Live-capability panes restored as
  evidence-only placeholders must require an explicit user rerun so
  the surface does not silently reattach authority. The validator
  rejects either failure.

## Headless consumers

The beta projection is exercised through the `aureline_shell_windows`
binary. The bin is the only mint-from-truth path for the JSON checked
in under `fixtures/ux/m3/window_topology/`, so the live shell records,
the CLI rows, and the support-export rows cannot drift.

```sh
cargo run -q -p aureline-shell --bin aureline_shell_windows -- page
cargo run -q -p aureline-shell --bin aureline_shell_windows -- split
cargo run -q -p aureline-shell --bin aureline_shell_windows -- detach
cargo run -q -p aureline-shell --bin aureline_shell_windows -- move
cargo run -q -p aureline-shell --bin aureline_shell_windows -- restore
cargo run -q -p aureline-shell --bin aureline_shell_windows -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_windows -- validate
```

`validate` exits non-zero (status `3`) if any acceptance invariant is
violated; it is wired so CI can fail closed on a regression in any of
the four record kinds.

## Fixtures

Reviewable fixtures live under
[`fixtures/ux/m3/window_topology/`](../../../fixtures/ux/m3/window_topology/):

- `split_intents.json` — vertical split of an editor pane with focus
  on the new pane and no silent buffer fork.
- `detach_intents.json` — detach a terminal pane into a new secondary
  window with the dirty buffer authority preserved.
- `move_intents.json` — move a dirty tab to the primary window, move a
  diff/review surface into a review window, and move the settings
  effective-value inspector into an inspector window.
- `restore_outcomes.json` — restore after mixed-DPI normalization,
  restore after display detach with terminal evidence-only fallback,
  and restore after a dock/undock cycle that clears fullscreen state.
- `page.json` — full beta page with the aggregate summary banner.
- `support_export.json` — support-export wrapper that quotes the page
  and every case id.

## Verification

```sh
cargo test -p aureline-shell --test windows_beta_fixtures
cargo test -p aureline-shell --lib windows
cargo run -q -p aureline-shell --bin aureline_shell_windows -- validate
```

The fixture test in
[`crates/aureline-shell/tests/windows_beta_fixtures.rs`](../../../crates/aureline-shell/tests/windows_beta_fixtures.rs)
replays every JSON fixture through the Rust types and asserts the
contract invariants. The fixture test also asserts that the
checked-in `page.json` is bit-for-bit equal to the page returned by
the seeded builder, so regenerating with the headless bin is the only
mint-from-truth path.
