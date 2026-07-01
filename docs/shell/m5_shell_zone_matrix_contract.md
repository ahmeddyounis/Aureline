# M5 Shell-Zone, Responsive-Class, and Multi-Window Continuity Matrix

This document is the contract for the frozen M5 matrix that names the canonical
Aureline live-shell object model. The matrix is the single M5 source of truth for
whether a claimed shell surface may assert desktop/shell maturity: the shell
frame, windowing, layout, and status subsystems, plus docs/help and release-proof
packets, ingest the checked-in packet rather than re-inventing local slot,
collapse, or multi-window prose. A new M5 surface cannot claim shell maturity
without mapping its canonical slot, fallback slot, dependency-missing placeholder,
responsive collapse ladder, window classes, occupant transitions, owning-window
routing, and workspace-global continuity truths into this matrix.

- Record kind: `freeze_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix`
- Schema: [`schemas/shell/m5-shell-zone.schema.json`](../../schemas/shell/m5-shell-zone.schema.json)
- Responsive-class companion schema: [`schemas/shell/m5-responsive-class.schema.json`](../../schemas/shell/m5-responsive-class.schema.json)
- Canonical release-proof support export: [`artifacts/release/m5-shell-continuity-proof/support_export.json`](../../artifacts/release/m5-shell-continuity-proof/support_export.json)
- Governance summary: [`artifacts/release/m5-shell-continuity-proof/governance.md`](../../artifacts/release/m5-shell-continuity-proof/governance.md)
- Matrix CSV: [`artifacts/release/m5-shell-continuity-proof/matrix.csv`](../../artifacts/release/m5-shell-continuity-proof/matrix.csv)
- Human-readable matrix: [`artifacts/shell/m5-shell-zone-matrix.md`](../../artifacts/shell/m5-shell-zone-matrix.md)
- Fixtures: [`fixtures/ui/m5-shell-layouts/`](../../fixtures/ui/m5-shell-layouts/)
- Producer: `aureline_shell::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::current_stable_m5_shell_zone_matrix_export`
- Headless emitter: `aureline_shell_m5_shell_zone_matrix`

## Canonical shell zones

The eight canonical shell zones a surface may attach to — frozen upstream by the
`shell:stabilize_shell_zoning_and_responsive_fallback:v1` contract, mirrored here
so a new surface can never invent its own slot:

| Zone token | Zone |
| --- | --- |
| `title_context_bar` | Title / Context Bar — workspace, trust, target, profile, route identity |
| `activity_rail` | Activity Rail — durable top-level route rail |
| `left_sidebar` | Left Sidebar — structural navigation and query collections |
| `main_workspace` | Main Workspace — editor groups, review surfaces, primary working sets |
| `right_inspector` | Right Inspector — contextual detail and inspectable evidence |
| `bottom_panel` | Bottom Panel — execution, output, problems, terminal, timeline |
| `status_bar` | Status Bar — persistent instrumentation and compact recovery/status truth |
| `transient_overlay` | Transient Overlay — window-local palettes, dialogs, sheets, overlays |

## Governed surface families

| Family | Qualification | Canonical slot | Fallback slot | Placeholder behavior |
| --- | --- | --- | --- | --- |
| `notebook` | Stable | `main_workspace` | `main_workspace` | `in_slot_identity_preserved` |
| `data_grid` | Stable | `main_workspace` | `main_workspace` | `reconnect_remote_or_provider` |
| `profiler` | Stable | `bottom_panel` | `transient_overlay` | `reconnect_remote_or_provider` |
| `pipeline` | Stable | `main_workspace` | `bottom_panel` | `reconnect_remote_or_provider` |
| `docs` | Stable | `main_workspace` | `transient_overlay` | `in_slot_identity_preserved` |
| `preview` | Stable | `right_inspector` | `transient_overlay` | `in_slot_identity_preserved` |
| `review` | Stable | `main_workspace` | `right_inspector` | `reconnect_remote_or_provider` |
| `incident` | Beta | `main_workspace` | `right_inspector` | `reconnect_remote_or_provider` |
| `companion` | Beta | `right_inspector` | `transient_overlay` | `install_or_enable_dependency` |
| `operator` | Beta | `bottom_panel` | `transient_overlay` | `recentered_on_topology_drift` |

Each surface row binds a qualification class to its required fields; its canonical
and fallback shell slots; the dependency-missing placeholder behavior; the
controlled state vocabularies it carries with their concrete tokens; the
responsive classes it must survive; the window classes it may live in; the allowed
occupant transitions; the ordered responsive collapse ladder; the owning-window
routing expectations; the workspace-global continuity truths every window
preserves; its evidence requirement; the proof packet refs that keep it current;
its downgrade triggers; its rollback posture; its source contracts; and the
consumer surfaces that must project its slot metadata.

## Controlled vocabularies

The self-describing `vocabulary_set` freezes every canonical token, validated
against the typed `ALL` arrays in the Rust producer so the vocabulary cannot
silently drift:

- **Responsive class** — `compact_desktop`, `standard_desktop`, `expanded_desktop`.
- **Window class** — `primary_workspace_window`, `secondary_detached_window`,
  `floating_utility_window`, `companion_overlay_window`.
- **Occupant transition** — `side_by_side`, `tabbed`, `sheeted`, `overflowed`,
  `solo_docked`.
- **Fallback placement** — `docked`, `sheet`, `overflow`, `placeholder`.
- **Owning-window routing** — `route_to_owning_window_object`,
  `preserve_object_anchor_on_return`, `no_focus_theft`, `no_orphan_on_detach`.
- **Continuity truth** — `workspace_global_trust`, `remote_target`,
  `deployment_profile`, `recovery_state`.
- **Placeholder behavior** — `in_slot_identity_preserved`,
  `reconnect_remote_or_provider`, `install_or_enable_dependency`,
  `recentered_on_topology_drift`.

## Invariants (validated in `crates/aureline-shell`)

The Rust validator is the authoritative gate. It fails or narrows on any of:

- A surface attached outside a declared shell slot (`slot_undeclared`).
- A responsive collapse ladder that does not terminate in `placeholder`, so
  identity and the reopen path are always preserved.
- A family that does not admit the `primary_workspace_window`, does not survive
  every responsive class, does not declare every owning-window routing
  expectation, or does not preserve every workspace-global continuity truth.
- A declared vocabulary carrying no tokens, or tokens present for an undeclared
  vocabulary.
- A Stable family missing proof packet refs; a family missing downgrade triggers
  or consumer surfaces; drift in the frozen vocabulary set; missing source
  contracts; or raw boundary material (`://`, tokens, credentials) in the export.

The continuity review block asserts the track invariant as hard flags: new
surfaces attach only to declared shell slots; responsive collapse never changes
task identity or hides critical state; every window preserves workspace-global
trust, remote, profile, and recovery truth while layout stays local; dialogs,
notifications, and approvals route back to the owning window and object without
focus theft or orphaning; one shell-zone matrix is consumed rather than local
layout prose; and an unmapped surface blocks a shell-maturity claim.

## Downgrade and rollback

A finding narrows the claim rather than hiding the surface. The
`profiler_remote_held` and `companion_overlay_narrowed` fixtures under
`fixtures/ui/m5-shell-layouts/` prove that a held or narrowed family stays mapped
into the matrix — the family remains present with a lowered qualification and a
recommended (rather than required) evidence posture.

## Source contracts

The matrix mirrors, and cites by id, the upstream contracts it builds on rather
than minting parallel tokens:

- `shell:stabilize_shell_zoning_and_responsive_fallback:v1` — frozen shell zones
  and responsive fallback.
- [`schemas/workspace/window_topology_snapshot.schema.json`](../../schemas/workspace/window_topology_snapshot.schema.json) — multi-window / display topology truth.
- [`schemas/activity/m5-attention-routing.schema.json`](../../schemas/activity/m5-attention-routing.schema.json) — owning-window attention routing.
- [`schemas/ux/notification_envelope.schema.json`](../../schemas/ux/notification_envelope.schema.json) — routed-action envelope truth.
- [`schemas/recovery/session-restore-fidelity.schema.json`](../../schemas/recovery/session-restore-fidelity.schema.json) — recovery / restore truth.
- [`schemas/design-system/m5-reference-layout.schema.json`](../../schemas/design-system/m5-reference-layout.schema.json) — design-token / slot fidelity.
