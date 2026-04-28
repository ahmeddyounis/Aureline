# Splitter and Resizable-Pane Contract

This document freezes the accessibility, collapse, restore, and
persistence contract for shell splitters and resizable panes. Splitters
are interactive controls that preserve comprehension of a workspace; they
are not disposable layout decoration.

Companion artifacts:

- [`/schemas/ux/splitter_state.schema.json`](../../schemas/ux/splitter_state.schema.json)
  defines the cross-tool packet shape for splitter controls,
  collapse decisions, and persistence restore decisions.
- [`/fixtures/ux/splitter_cases/`](../../fixtures/ux/splitter_cases/)
  contains worked cases for keyboard resize, guarded collapse, visible
  recovery, and topology-safe restore.

Normative sources projected here:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` requires larger
  interactive hit targets than visible splitter lines, hover/focus
  reinforcement, keyboard coarse and fine resizing, reset/equalize
  actions, recoverable collapsed panes, proportional or preset-based
  persistence, no silent collapse for durable or policy-critical panes,
  and screen-reader labels that name the controlled regions.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` repeats the
  splitter control requirements and the instruction to persist intent
  instead of brittle pixel positions.
- [`/docs/ux/shell_zone_and_density_contract.md`](./shell_zone_and_density_contract.md)
  and [`/artifacts/ux/shell_metrics.yaml`](../../artifacts/ux/shell_metrics.yaml)
  own shell zones, density modes, adaptive classes, minimum widths, and
  resize-handle metric floors.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  owns pane-tree identity, skeleton-first restore, placeholder
  hydration, and the separation between window topology and workspace
  authority.
- [`/docs/adr/0016-shell-windowing-input-accessibility-boundary.md`](../adr/0016-shell-windowing-input-accessibility-boundary.md)
  owns shell focus routing, command dispatch, accessibility-tree
  publication, and platform-adapter boundaries.

If this contract and the UI/UX spec disagree, the spec wins and this
contract, schema, and fixtures MUST update in the same change. If a later
shell surface implements private splitter behavior that disagrees with
this document, the surface is non-conforming.

## 1. Scope

This contract freezes:

- splitter anatomy: visible line, interaction hit target, hover and focus
  reinforcement, disabled/degraded states, and accessibility publication;
- keyboard resizing: focusability, fine and coarse movement, reset to
  default, equalize siblings, bounds clamping, and narration;
- collapse and restore behavior: visible recovery affordances,
  command-backed reopen routes, focus return, and rules preventing panes
  with durable jobs, blocking problems, active debug state, dirty
  recovery, or policy-critical state from disappearing silently;
- persistence: proportional intent, named presets, explicit overrides,
  topology and density changes, and the rule that raw pixel snapshots are
  never durable authority.

Out of scope:

- final visual polish, animation easing, and platform-specific drag
  physics;
- exact native accessibility API mappings beyond the shared semantic
  requirements here;
- implementation crate names or production storage bytes.

## 2. Boundary

Splitters live in window topology. They may affect the presentation of
workspace authority, but they do not own workspace authority.

| Concern | Owner | Splitter responsibility | Must not imply |
| --- | --- | --- | --- |
| Pane ids, split ids, group order, focus chain, visible inspectors | Window topology | Resize, collapse, restore, and persist layout intent using stable ids | Closing or resizing a pane mutates buffers, jobs, trust, or policy truth |
| Open buffers, dirty journals, durable jobs, debug sessions, policy state | Workspace authority and owning subsystem | Detect collapse barriers and keep recovery routes visible | A neighboring pane may erase or hide active authority |
| Density, adaptive class, hit-target floors, minimum useful widths | Shell metrics and density contract | Clamp and degrade layouts before violating floors | Density may reduce accessibility hit targets |
| Command routes and keyboard shortcuts | Command graph and shell input boundary | Expose resize, reset, equalize, collapse, and reopen as commands | Pointer-only resizing is enough |
| Accessibility semantics | Shell accessibility tree | Publish each adjustable splitter as a named focusable control | Anonymous separator with no controlled-region name |

Rules:

1. A splitter record MUST reference stable split and pane or region refs.
   It MUST NOT carry raw paths, raw commands, capability tickets, raw
   logs, credentials, or workspace content.
2. Resizing is window-local layout state. It MUST NOT affect another
   window unless the user invokes an explicit multi-window layout action.
3. A splitter MAY collapse a pane only through a collapse decision that
   records recoverability and collapse barriers.
4. A collapse that would hide active authority, blocking state, or a
   required recovery path MUST be denied, staged for explicit review, or
   converted into a visible summary route.

## 3. Anatomy

Every interactive splitter has two sizes:

| Part | Requirement |
| --- | --- |
| Visible line | The painted divider MAY be thin and visually quiet. It should remain stable across hover and focus so nearby editor content does not jump. |
| Hit target | The logical interaction area MUST be larger than the visible line. It inherits the shell metric floor: minimum 4 logical px, preferred 6-8 logical px at 100% zoom. Density modes MUST NOT reduce this floor. |
| Reinforcement | Hover, drag, and keyboard focus strengthen contrast, thickness, or local handle marks. They MUST NOT create a heavy rail that competes with editor text or hides adjacent content. |
| Accessibility node | A focusable adjustable splitter publishes orientation, current value, min/max, controlled-region refs, and a screen-reader name that identifies both controlled regions. |

Required invariants:

1. The hit target is measured in logical pixels after OS and application
   scaling. At high zoom, it remains operable even if the visible line is
   still thin.
2. Hover and focus reinforcement MUST NOT change the split ratio, alter
   the pane tree, or trigger delayed collapse.
3. If a splitter is disabled because a surface is locked, policy-bound,
   or below a safe width, the disabled reason remains visible through a
   tooltip, focus popover, status row, or command-disabled reason.
4. Splitters are not hover-only controls. Keyboard focus and command
   routes must reach them.

## 4. Accessibility Naming

Screen-reader names describe the relationship controlled by the
splitter. Names such as `separator`, `divider`, or `splitter` alone are
non-conforming.

| Splitter location | Required naming pattern | Example |
| --- | --- | --- |
| Between editor and terminal panel | `Resize <primary region> and <secondary region>` | `Resize editor and terminal` |
| Between sidebar and workspace | `Resize <navigation region> and <workspace region>` | `Resize Explorer and editor` |
| Between workspace and inspector | `Resize <workspace region> and <detail region>` | `Resize editor and inspector` |
| Nested compare/editor group splitter | Include compare role or group label when needed | `Resize source compare and target compare` |

Rules:

1. Names come from redaction-aware region labels. Raw paths, raw project
   names that policy hides, and raw provider identifiers do not appear.
2. The accessibility description SHOULD include the current proportional
   sizes, bounds posture, and available reset/equalize actions.
3. When a controlled region is collapsed, the splitter or recovery
   affordance announces that state and the reopen command path.
4. Right-to-left layout may change visual direction, but it MUST NOT
   erase which two regions are controlled.

## 5. Keyboard Resize

Every adjustable splitter has the same semantic command set. Keybindings
may vary by platform or user profile, but the command meanings do not.

| Command class | Required behavior |
| --- | --- |
| Fine decrease / increase | Move the splitter by a small predictable step, clamped by minimum useful sizes and policy barriers. The seed value is 2% of the available span, with a logical-pixel floor chosen by implementation. |
| Coarse decrease / increase | Move the splitter by a larger predictable step for power users. The seed value is 10% of the available span, clamped the same way as fine movement. |
| Reset | Restore the default size or the last named default preset for this splitter family. Double-click or equivalent pointer behavior routes to the same command when supported. |
| Equalize siblings | Divide available space evenly across siblings in the same split node, unless minimum useful sizes or collapse barriers require a partial result. |
| Collapse / restore focused side | Collapse or restore the focused side only when recoverability rules are satisfied. |

Rules:

1. Keyboard resize commands MUST use the same layout engine and bounds
   checks as pointer drag. Keyboard and pointer paths may not disagree on
   minimums, barriers, or persistence.
2. Fine and coarse movements operate on proportional intent, not raw
   device pixels. Pixel deltas MAY exist inside the input adapter but
   must resolve to a proportional or preset decision before crossing the
   schema boundary.
3. The shell MUST announce when a resize clamps at a minimum useful
   width, when equalize is partial, or when collapse is denied because a
   protected pane carries active state.
4. Focus remains on the splitter after resize unless a collapse or
   restore explicitly moves focus to a visible recovery target.
5. A splitter that cannot satisfy both controlled panes' minimum useful
   sizes MUST select an explicit fallback: sheet an optional pane,
   overflow secondary tabs, convert side-by-side compare to tabbed or
   staged compare, or deny the operation with a typed reason.

## 6. Collapse And Restore

Collapse narrows presentation. It does not delete pane identity, discard
authority, or hide required status.

| Collapse source | Required behavior |
| --- | --- |
| User-invoked collapse | Preserve pane id, prior size intent, focus-return target, and at least one visible or command-backed recovery route. |
| Adaptive layout collapse | Collapse optional detail surfaces before identity, trust, recovery, or active task surfaces. Do not overwrite the user's saved explicit layout intent. |
| Neighbor requests more space | Check collapse barriers on the pane that would lose space. Deny or stage when barriers are present. |
| Restore-time unavailable dependency | Preserve surrounding split structure and replace only the unavailable pane with a placeholder or evidence-only surface. |

Recovery routes:

- visible splitter stub or grab handle;
- zone toggle, panel tab, sidebar/inspector toggle, or status item;
- placeholder card with safe actions;
- command-palette or menu entry with the same command descriptor as the
  visible route;
- remembered-state or restore-provenance detail when the pane was
  restored as context only.

At least one route is required for every collapsed pane. Panes carrying
durable jobs, blocking problems, active debug sessions, dirty recovery,
trust or reauthorization requirements, policy-critical state, or
collaboration/follow control state require both a visible route and a
command-backed route.

No-silent-collapse barriers:

| Barrier | Required result |
| --- | --- |
| Durable job or live run | Keep an activity/status route visible; do not hide running, paused, failed, or awaiting-review work. |
| Blocking problem or failed validation | Keep the problem count and owning pane route visible. |
| Active debug/session control | Deny silent collapse or show a debug-control summary with reopen route. |
| Dirty recovery or unsaved journal | Deny silent collapse unless the dirty/recovered state remains visible elsewhere. |
| Policy-critical state or trust prompt | Deny silent collapse; required policy/trust labels remain visible. |
| Reauth or remote reconnect required | Preserve route and boundary label; do not make the pane look absent or ready. |

Rules:

1. `collapsed`, `hidden by adaptive fallback`, `sheeted`, `placeholder`,
   and `removed by user` are distinct states.
2. A pane removed by explicit user action records intent. A pane
   collapsed by layout pressure remains recoverable.
3. Focus never lands on a collapsed or placeholderless pane. It moves to
   the nearest visible owner and records the focus-return target.
4. A collapse decision must be inspectable in support export without
   exporting workspace content.

## 7. Persistence

Splitter persistence saves intent. It does not save raw pixels as durable
truth.

Allowed durable persistence modes:

| Mode | Use when | Required contents |
| --- | --- | --- |
| `proportional_intent` | User resized a split and both sides remain ordinary panes | Stable split id, controlled region refs, normalized weights, minimum-policy refs, and last applied adaptive class |
| `named_preset` | Default, equalized, editor-dominant, terminal-tall, inspector-narrow, or presentation-oriented layouts | Preset name, split family, controlled region refs, and version |
| `explicit_user_override` | User knowingly forced a narrow or unusual split | User-choice evidence ref, normalized weights, warnings shown, and recovery routes |
| `temporary_session_only` | Transient drag, presentation, adaptive fallback, or recovery state should not overwrite user intent | Session ref, reason, and expiry or reset rule |

Raw pixel values MAY be captured as machine/display hints for diagnostics
or same-topology warm restore. They MUST NOT be the durable source of
truth. When topology, zoom, density, platform chrome, safe area, or
monitor scale changes, restore follows this order:

1. Rebuild the pane tree by stable split and pane ids.
2. Apply named preset or proportional weights to the current available
   span.
3. Clamp by minimum useful widths, hit-target floors, and policy
   barriers.
4. Sheet or overflow optional surfaces before starving the main
   workspace or hiding required identity.
5. Preserve collapsed-pane recovery routes and record any partial
   restoration as a visible restore or persistence note.

Rules:

1. Adaptive fallback is not a new user preference. It MUST NOT overwrite
   a saved explicit layout unless the user accepts that change.
2. Equalize writes a named or proportional intent only after the command
   succeeds or completes with a visible partial-result note.
3. Reset chooses the current default for the splitter family and density
   context, then stores that preset rather than a stale pixel width.
4. Cross-platform restore may produce different pixel sizes. It must
   preserve relative intent, active focus, minimum useful regions, and
   recovery affordances.
5. Persistence records must be redaction-safe and support-exportable.

## 8. Fixture Coverage

The worked cases under
[`/fixtures/ux/splitter_cases/`](../../fixtures/ux/splitter_cases/)
validate against
[`/schemas/ux/splitter_state.schema.json`](../../schemas/ux/splitter_state.schema.json)
and cover:

- an editor/terminal splitter with a thin visible line, larger hit
  target, focus reinforcement, screen-reader naming, fine/coarse
  keyboard resize, reset, and equalize;
- a collapse decision denied because a bottom-panel pane carries a
  durable job and blocking problems;
- a recoverable collapsed inspector with visible and command-backed
  reopen routes; and
- a topology, zoom, and density change that restores proportional
  layout intent without trusting stale pixel positions.

New fixtures MUST cite the contract sections they exercise and MUST
avoid raw paths, raw logs, raw command lines, raw provider payloads,
credentials, live authority handles, and task or planning identifiers.
