# Restore Hydrator Beta

This document freezes the workspace-facing beta contract for the **restore
hydrator** — the runtime that turns a remembered window-topology snapshot back
into a usable layout. It deliberately separates *state serialization* from
*restore orchestration*: serialization records what was remembered; the
hydrator decides how to bring it back safely when extensions, remotes,
displays, or runtime-heavy panes are missing or changed.

Machine-readable boundaries:

- [`/schemas/workspace/window_topology_snapshot.schema.json`](../../../schemas/workspace/window_topology_snapshot.schema.json) — restore-hydration request bundle (input).
- [`/schemas/workspace/pane_tree.schema.json`](../../../schemas/workspace/pane_tree.schema.json) — pane-tree and layout-restore provenance vocabulary (output).

Runtime model:

- [`/crates/aureline-workspace/src/restore_hydrator/`](../../../crates/aureline-workspace/src/restore_hydrator/)

Fixtures:

- [`/fixtures/workspace/m3/restore_hydration/`](../../../fixtures/workspace/m3/restore_hydration/)

Support parity:

- [`/artifacts/support/m3/restore_hydration_report.md`](../../../artifacts/support/m3/restore_hydration_report.md)

## Two ordered passes

The hydrator runs two ordered passes per window. The order is normative.

1. **Skeleton first.** Window shells, the pane topology (splits, tab groups,
   and leaves), and focus anchors are recreated before any heavy dependency is
   touched, and window bounds are remapped into safe visible bounds for the
   current monitor topology. A window therefore stays visible and structurally
   recognizable even when nothing else can hydrate.
2. **Lazy hydration.** Remote sessions, terminals, notebooks, debuggers,
   preview servers, and extension panes hydrate second. When a dependency is
   missing, revoked, or unavailable the pane reopens as a truthful placeholder
   that preserves its slot — it never collapses the surrounding layout and never
   impersonates a live, ready surface.

The five restore phases (`chooser`, `skeleton`, `hydrate`, `rebind`,
`evidence_only_fallback`) are recorded in the emitted phase trace so startup,
diagnostics, and support flows read the same progression.

## Skeleton stays visible

| Guarantee | Rule |
| --- | --- |
| Windows recreated | Every window shell is restored (`shell_restored = true`) before hydration. |
| Topology preserved | Every leaf pane id from the snapshot is preserved in the restored window; no pane is dropped. |
| Focus anchor kept | The first focus-chain entry becomes the window focus anchor and survives placeholder insertion. |
| No trapped windows | Final bounds are always inside a connected display; off-screen or invalid saved bounds snap to safe bounds. |

## Missing surfaces are honest placeholders

A pane that cannot hydrate live reopens as a placeholder that keeps its slot.
The missing-dependency class uses the in-product `placeholder_reason_class`
vocabulary: `missing_extension`, `missing_remote`, `missing_remote_authority`,
`revoked_permission`, `unsupported_display_topology`,
`non_reentrant_live_surface`, `schema_migration_review_required`,
`manual_recovery_required`.

- A missing **non-live** dependency produces a `placeholder_result` carrying the
  reason and safe recovery actions.
- A missing **live** dependency produces both a `placeholder_result` (reason +
  safe actions) and a `live_surface_outcome` (the no-rerun contract, posture
  `placeholder_until_manual_rebind`).
- A live surface whose **process did not survive** restore reopens inert as
  `evidence_only_placeholder` (evidence retained) or `metadata_only_placeholder`,
  with reason `non_reentrant_live_surface`.
- A live surface whose session is **reattachable** reopens as
  `live_attach_visible` with `existing_authority_still_valid`.

Placeholders never carry the `live_attach_visible` posture, so they cannot imply
live readiness.

## Monitor-safe remap

Window bounds are remapped against the connected-display topology and every
material change is recorded as a `display_adjustment` with the affected pane ids:

| Condition | Adjustment |
| --- | --- |
| Saved display not connected | `moved_to_primary_display` |
| Saved scale bucket differs | `scale_normalized` |
| Saved bounds off-screen or invalid | `snapped_to_safe_bounds` |
| Window was fullscreen and policy clears it | `fullscreen_cleared` |

A display adjustment downgrades the window restore class to at most
`compatible_restore`; it never traps the window off-screen.

## No silent rerun

Mutating or privileged sessions are never replayed automatically. Every
`live_surface_outcome` records explicit no-rerun guardrails and always includes
`explicit_user_action_required`. Command-bearing surfaces (terminals, tasks,
pipelines, preview runtimes, notebooks, debuggers) additionally record
`no_command_rerun`; surfaces that show retained transcripts record
`transcript_or_snapshot_only`; surfaces held as placeholders record
`placeholder_preserved`. Authority is never reacquired silently
(`no_hidden_authority_reacquire`).

## Restore class and authority

The window **restore class** (`restore_level`) folds the worst pane outcome
together with any display adjustment:

- `exact_restore` — all panes live or lightweight, no placeholder, no adjustment.
- `compatible_restore` — a session reattached or a display adjustment applied.
- `layout_only` — at least one recoverable placeholder slot.
- `evidence_only` — a heavy pane fell back to retained evidence.

Workspace authority is resolved in the `rebind` phase and reported as
`bound_existing_authority`, `reevaluated_and_bound`, `degraded_local_only`, or
`missing_authority_placeholder`. The hydrator never serializes or replays live
authority, tokens, or delegated approvals to make a restore look exact.

## Diagnostics and support parity

`RestoreHydrationSummary` projects the restore class, missing-dependency
classes, remaining manual actions, and display-adjustment classes — the same
vocabulary shown in-product — along with stable diagnostics, support-export, and
crash-recovery refs. `render_plaintext` renders a support-safe view.

## Out of scope

This contract does not extend into full collaboration/session restore parity.
It is bounded to local-safe continuity: reopen a usable, honest layout without
replaying mutating or privileged sessions.
