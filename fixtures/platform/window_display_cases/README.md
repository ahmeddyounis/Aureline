# Window/display verification cases

Reviewable multi-window, monitor-topology, mixed-DPI, suspend/resume,
and reopen scenarios that anchor the vocabulary frozen in
[`/docs/ux/window_display_contract.md`](../../../docs/ux/window_display_contract.md),
[`/docs/qa/multi_window_verification.md`](../../../docs/qa/multi_window_verification.md)
and
[`/artifacts/qa/window_display_matrix.yaml`](../../../artifacts/qa/window_display_matrix.yaml).
Each JSON fixture conforms to
[`/schemas/platform/window_state.schema.json`](../../../schemas/platform/window_state.schema.json).

These fixtures are not an automated replay suite. They freeze the case
shape release, support, accessibility, and future automation lanes cite
when they need to reason about:

- which continuity scenario is under review;
- which layout-intent, focus, and dialog-ownership truths must hold;
- which live surfaces keep context versus which must reconnect, rerun,
  or remain unavailable; and
- which native controls, recovery actions, restore-history details, and
  continuity cues must stay visible.

## Fixture rules

- Every fixture names one `scenario_id` from the matrix and at least
  one frozen `required_drill` id.
- Every fixture names the native titlebar/control projection, display
  topology classes, focus-return rule, restore-history requirement, and
  continuity cues it exercises.
- Every fixture preserves layout intent, visible focus, and safe
  recovery actions rather than assuming raw pixel replay is the goal.
- Fixtures that include terminal, debug, notebook, preview, AI, or
  remote panes MUST distinguish restorable context from live authority.
- Fixtures never claim silent hidden reruns, silent authority reuse, or
  unreachable dialogs as acceptable degradation.

## Index

| Fixture | What it exercises | Expected truth |
|---|---|---|
| `split_layout_detached_auxiliary_focus.json` | split layout plus detached auxiliary window | re-dock keeps pane identity, window role, and visible focus |
| `display_detach_dock_safe_bounds.json` | display detach/redock with windows on multiple monitors | windows move into reachable bounds and keep the working pane visible |
| `mixed_dpi_cross_monitor_reflow.json` | per-monitor DPI or scale-bucket change | readable scale, sheet ownership, and keyboard recovery remain intact |
| `fullscreen_snapped_restore_intent.json` | fullscreen/spaces/snap restore and presentation fallback after reopen | dominant work or presentation intent returns without off-screen stranding |
| `offscreen_dialog_owner_recenter.json` | dialog or sheet orphaned by a topology change | dialog recenters on the owner window and focus returns there |
| `suspend_resume_remote_rebind.json` | sleep/wake with remote/debug/task surfaces | local context survives; authority rebinding stays explicit |
| `restart_reopen_live_surface_rebind.json` | reopen of live terminal/task/debug/notebook surfaces | transcripts and lineage restore without hidden reruns |
| `restart_reopen_missing_dependency_placeholder.json` | reopen with missing extension/remote target/authority | pane slots become placeholders or evidence-only cards without collapsing layout |

## Coverage contract

The seeded fixture set keeps:

- at least one case for detached windows and split layouts;
- at least one case for live display-topology change and mixed-DPI
  reflow;
- at least one case for off-screen dialog ownership recovery;
- at least one case for fullscreen/snapped restore with presentation
  target fallback;
- at least one case for suspend/resume with explicit authority rebinding;
- at least one case for restart/reopen with no-hidden-rerun postures;
- at least one case for placeholder hydration that preserves topology.
