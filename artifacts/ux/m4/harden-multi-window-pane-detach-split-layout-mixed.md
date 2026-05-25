# Multi-window, pane-detach, split-layout, mixed-DPI, and multi-monitor restore — release evidence

Reviewer-facing evidence packet for the lane that hardens **multi-window, pane
detach, split-layout, mixed-DPI, and multi-monitor restore** on claimed-stable
desktop shell surfaces: one canonical record per window reopen that binds
authority / topology separation, a versioned pane tree with stable pane IDs,
skeleton-first / hydrate-second restore, restore-no-rerun honesty,
display-topology and downgrade provenance, a public claim ceiling, an automatic
narrow-below-Stable verdict, recovery and route parity across the activity
center / command palette / status bar / menus, accessibility across normal /
high-contrast / zoomed layouts, and rows that stay available without an account
or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed/`](../../../fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed/)
- Schema: [`/schemas/ux/harden-multi-window-pane-detach-split-layout-mixed.schema.json`](../../../schemas/ux/harden-multi-window-pane-detach-split-layout-mixed.schema.json)
- Companion doc: [`/docs/ux/m4/harden-multi-window-pane-detach-split-layout-mixed.md`](../../../docs/ux/m4/harden-multi-window-pane-detach-split-layout-mixed.md)
- Typed source: `aureline_shell::window_topology_restore_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_window_topology_restore_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/window_topology_restore_stable_fixtures.rs`

## The claimed-stable matrix

| Record | Posture | Claim | Surface marker | Fidelity | Narrowing reason |
| --- | --- | --- | --- | --- | --- |
| `exact_single_window.json` | Single window, exact reopen | **stable** | stable | Exact | — |
| `mixed_dpi_multi_monitor_compatible.json` | Mixed-DPI dock, compatible reopen | **stable** | stable | Compatible | — |
| `monitor_removed_placeholder_backed.json` | Monitor removed, placeholder-backed recovery | **stable** | stable | Placeholder-backed | — |
| `help_about_preview_surface.json` | Exact reopen, Help/About surface in preview | preview (narrowed) | preview | Exact | `surface_not_yet_stable` |
| `authority_topology_leak_drill.json` | Authority/topology fusion drill | beta (narrowed) | stable | Exact | `authority_topology_not_separated` |

Coverage verdict: **3 Stable, 2 narrowed**. Each narrowed row names a reason and
drops below the launch cutline rather than inheriting an adjacent green row.

## Acceptance criteria → evidence

- **Window-topology restore is separated from session-scoped execution restore.**
  `authority.workspace_authority_classes[]` (dirty buffers, recovery journals,
  trust/policy, VFS identity, attached execution contexts) stays centralized while
  `authority.window_local_topology_classes[]` (pane-tree layout, focus history,
  zoom/follow state, visible surfaces) stays window-local. The fusion drill proves
  the lane detects and narrows a reopen that fuses them — guarded by
  `authority_and_topology_are_separated_in_the_model`.
- **Split, move, float, pin, and close-pane mutate a versioned pane tree with
  stable pane IDs.** `pane_tree.pane_tree_schema_version` is current; every leaf
  resolves to a slot, every slot is in the tree, pane IDs are unique, and no split
  carries a zero weight — guarded by `pane_tree_is_versioned_with_stable_ids`.
- **Restore is skeleton-first / hydrate-second with no silent rerun or hidden
  authority reacquisition.** No session-scoped pane hydrates live; placeholder
  panes forbid command rerun and authority reacquire, carry a restore-no-rerun
  state, and offer a recovery action.
  `monitor_removed_placeholder_backed.json` keeps five session panes
  (`transcript_restored` / `session_ended` / `reconnect_available` /
  `rerun_required` / `context_unavailable`) as placeholder cards rather than
  collapsing the panes — guarded by `restore_is_skeleton_first_with_no_silent_rerun`.
- **Restore fidelity and display-topology adjustment provenance are recorded on
  every reopen.** `restore_provenance.resulting_fidelity` is one of Exact /
  Compatible / Layout-only / Placeholder-backed; any non-exact reopen carries a
  `downgrade` with a named reason and matching target fidelity, and
  `display_topology` records the topology change classes and the adjustments
  applied. The mixed-DPI row records a Compatible downgrade with scale/bounds
  adjustments; the monitor-removed row records a placeholder-backed downgrade —
  guarded by `restore_fidelity_and_topology_provenance_is_consistent`.
- **Diagnostics and support exports explain the reopen without scraping localized
  UI copy.** The desktop restore review, CLI inspect, Help/About, and support
  export all read the shared record; `surface_projections[]` carries each
  surface's marker and a deterministic summary line — guarded by
  `surfaces_bind_the_shared_record`. The `plaintext` emitter renders an
  export-safe truth block per scenario.
- **Surfaces still lacking stable qualification are automatically narrowed.** The
  Help/About-preview and authority-fusion rows drop below Stable with named
  reasons — guarded by `narrowed_rows_drop_below_cutline_and_name_a_reason`,
  `matrix_spans_stable_and_narrowed_rows`, and `claim_ceiling_never_overclaims`.
- **Discover / operate / recover from keyboard and mouse without account
  requirements or toast-only truth.** The recovery routes (Open restore details,
  plus conditional Reconnect / Rerun / Install, plus Compare / Export) and the
  four entry-route surfaces are keyboard-reachable across three layout modes;
  every row stays available without an account or managed services — guarded by
  `recovery_routes_are_complete_and_keyboard_reachable`,
  `routes_reach_every_surface_keyboard_first`,
  `accessibility_holds_in_every_layout`, and
  `rows_stay_available_without_account_or_managed_services`.
- **Minted refs are canonical durable-object refs.** Workspace-authority,
  restore-provenance, reopen-target, diagnostics, support, evidence, and narrative
  refs are opaque `aureline://<class>/<id>` refs — guarded by
  `minted_refs_are_canonical_durable_objects`.

## Verification

```sh
# Unit + projection invariants
cargo test -p aureline-shell --lib window_topology_restore_stable

# Fixture replay + acceptance-criteria invariants
cargo test -p aureline-shell --test window_topology_restore_stable_fixtures

# Refresh fixtures from the in-code projection
cargo run -q -p aureline-shell \
  --bin aureline_shell_window_topology_restore_stable -- emit-fixtures \
  fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed

# Reviewer index / plaintext support export
cargo run -q -p aureline-shell --bin aureline_shell_window_topology_restore_stable -- index
cargo run -q -p aureline-shell --bin aureline_shell_window_topology_restore_stable -- plaintext
```

The fixtures are a literal projection of the in-code corpus, which is itself a
projection of the live windows workspace-management page; the replay gate fails
on any drift.
