# Focus, Zoom, and Pointer-Independence Contract

Status: seeded

This contract freezes the shared keyboard and visual-orientation rules
for Aureline surfaces that must remain usable at high zoom, with larger
text, and without pointer-exclusive interaction. It applies to shell,
editor, terminal, review, dense-collection, docs/help, trust, remote,
collaboration, notebook, graph, and overlay surfaces whenever they are
part of a launch-critical workflow or a stable claim.

Contract identity:

- `focus_zoom_pointer_contract_id:
  aureline.accessibility.focus_zoom_pointer_independence`
- `focus_zoom_pointer_contract_revision: 1`
- `focus_owner_schema_version: 1`

Companion artifacts:

- [`/schemas/accessibility/focus_owner.schema.json`](../../schemas/accessibility/focus_owner.schema.json)
  defines focus-owner snapshots, focus targets, and zoom or
  pointer-disclosure rows.
- [`/fixtures/accessibility/focus_zoom_cases/`](../../fixtures/accessibility/focus_zoom_cases/)
  contains seed cases for high zoom, larger text, overlay focus return,
  cursor ergonomics, and no-drag keyboard equivalents.
- [`/artifacts/accessibility/no_drag_only_inventory.yaml`](../../artifacts/accessibility/no_drag_only_inventory.yaml)
  tracks remaining pointer-only or pointer-risk actions with explicit
  owner, status, and replacement path.
- [`/docs/accessibility/a11y_ime_packet_template.md`](./a11y_ime_packet_template.md)
  is the packet template that evidence rows use when citing this
  contract.
- [`/docs/accessibility/screen_reader_and_live_region_contract.md`](./screen_reader_and_live_region_contract.md)
  defines the live-region and assistive-technology announcement
  boundary this contract composes with.
- [`/artifacts/accessibility/shell_conformance_checklist.yaml`](../../artifacts/accessibility/shell_conformance_checklist.yaml)
  remains the launch-critical checklist that surface-specific packets
  cite.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` Section 11 requires keyboard access,
  visible focus, no critical drag-only interaction, configurable font
  size and line height, zoom from 50% to 400%, and cursor controls.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Sections 19.1-19.4,
  19.6, 19.12, Appendix G, Appendix EL, and Appendix EP define
  keyboard completeness, visible focus, focus return, dense-surface
  high-zoom behavior, and 400% responsive fallback.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` Sections 8.3,
  9.4, 12.6, 14.1, and 14.13 define line-height minimums, the 2 px
  focus ring, reflow-before-clipping behavior, focus/selection
  distinction, and drag-and-drop recovery expectations.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Section 12.7
  defines window-local focus history, zoom, visible surfaces, and
  primary-focus preservation across restore and display changes.
- `.t2/docs/Aureline_Technical_Design_Document.md` Sections 8.4 and
  8.13 require stable zoom/accessibility scaling and forbid
  launch-critical drag-only, hover-only, or mouse-only workflows.

## Scope

This contract is the citation surface for:

- primary focus ownership within each window;
- overlay, dialog, sheet, popover, and transient-surface focus return;
- visible focus indicators and focus/selection/current-item
  distinction;
- 50% to 400% zoom, OS scaling, larger text, increased line height,
  cursor thickness, and cursor blink controls;
- high-zoom simplification of minimap, overview, dense chrome, and
  secondary panels;
- keyboard equivalents for reorder, selection, pane resize, review,
  and drop/import flows;
- user-visible disclosure when a control is reduced, moved, simplified,
  or unavailable at the current zoom, contrast, or platform-scaling
  posture.

Out of scope:

- certifying every OS scale factor or monitor topology combination;
- replacing the screen-reader announcement contract;
- requiring every non-critical visual affordance to remain visible at
  400% zoom;
- implementing every future visual-designer or graph-canvas keyboard
  interaction before that surface is claimed stable.

## Focus Ownership

Every active top-level window has exactly one primary focus owner.
Secondary focusable targets may exist, but only one target receives
keyboard input and assistive-technology current-focus semantics at a
time.

Required focus-owner invariants:

| Invariant | Required behavior | Non-conforming behavior |
|---|---|---|
| One primary owner | Each window snapshot has one `primary_focus_owner` and zero hidden primary owners. | Two panels both claim active focus, or focus falls to an unowned document body. |
| Window-local focus | Focus history, return targets, zoom, and pane topology are window-local. | Dismissing a sheet in one window warps focus into another window. |
| Visible target | The primary owner is visible, recentered, or replaced by a visible placeholder with a return path. | Focus remains in a hidden, collapsed, off-screen, destroyed, or virtualized-out target. |
| Preserved return path | Closing a transient surface returns to the invoker, nearest safe ancestor, current batch/detail owner, or placeholder-announced owner. | Closing a dialog drops focus silently or picks the most recently painted pane. |
| No hidden targets | Hidden or destroyed targets are denied before they become focus owners. | A cached element keeps focus after its pane is removed or virtualized away. |
| Meaningful escape | Escape resolves the innermost transient state before leaving the larger surface. | Escape closes an editor group while a completion list or sheet is still active. |

The focus owner must be represented by a `focus_owner_snapshot_record`
whenever a packet needs machine-readable proof. The snapshot carries:

- window identity and surface family;
- primary focus owner;
- overlay stack, if any;
- focus return path and fallback;
- visible focus indicator facts;
- zoom/scaling context;
- pointer-independence checks;
- disclosure rows for any reduced or unavailable controls.

## Overlays and Transient Surfaces

Dialogs, sheets, popovers, menus, command palettes, completion lists,
peek panels, spotlight frames, guided-tour steps, and presentation
overlays must not create ambiguous focus.

Rules:

- Modal overlays own focus inside the originating window until they are
  dismissed or complete.
- Non-modal overlays may expose focusable content only when the user
  explicitly invokes or pins them by keyboard, command, touch, or
  pointer action.
- The invoker remains the preferred return target unless the action
  intentionally changes context.
- If the invoker disappears, the surface returns to the nearest visible
  safe ancestor, current batch/detail owner, or placeholder-announced
  owner.
- If no safe return target exists, the action is denied as
  `focus_loss_denied` and a durable re-entry affordance is required.
- Overlay dimming, spotlight masks, presentation frames, or reduced
  contrast modes may not obscure the only focus indicator.

Allowed focus-return states:

| State | Meaning | Conformance posture |
|---|---|---|
| `returned_exact` | Focus returned to the same logical control, row, range, or editor insertion point. | Passed. |
| `returned_nearest_safe_ancestor` | Original target is gone or hidden; focus moved to the closest visible owner. | Degraded but conforming when announced. |
| `returned_current_batch_or_detail_owner` | A review/detail surface now owns the scope; focus returned there intentionally. | Conforming when the ownership shift is visible. |
| `returned_placeholder_announced` | A placeholder replaced the unavailable target and announced why. | Degraded but conforming. |
| `focus_loss_denied` | The operation would strand focus. The close, remove, or transition is blocked until a safe target exists. | Required denial. |
| `focus_not_applicable_non_interactive` | The source is a non-interactive event and no focus return is expected. | Allowed only for passive events. |

## Focus Indicator

Focus indicators are product-critical chrome, not decorative styling.

Minimum visible-focus rules:

- The focus indicator is at least 2 px in effective rendered thickness.
- The indicator has at least 3:1 contrast against adjacent colors.
- The indicator uses a non-color cue such as outline, stroke, inset,
  caret, shape, label, or active-region boundary.
- Focus and selection remain visually and semantically distinct.
- Current item, selection, anchor, pressed state, hover, and activation
  may not reuse the same cue as keyboard focus.
- Focus remains visible in dark, light, high-contrast, dimmed overlay,
  reduced-motion, presentation, and dense modes.
- Custom-rendered canvases expose an equivalent focus owner through the
  accessibility bridge and a visible in-canvas cue.

The focus ring may use design-system focus tokens, but token use alone
does not satisfy this contract. Evidence must show the rendered result
at the active zoom, contrast, density, and platform scaling.

## Zoom, Scaling, and Larger Text

Aureline surfaces must remain functional from 50% to 400% zoom. OS text
scale, platform scaling, larger editor fonts, increased line height, and
presentation zoom all compose into the same effective layout contract.

Required behavior:

| Area | Required behavior | Failure to avoid |
|---|---|---|
| Reflow | Reflow before clipping, overlapping, or hiding meaningful labels. | Controls clipped to unreadable fragments while empty margins remain elsewhere. |
| Launch layouts | At 400% zoom, preserve at least one editor group and one critical side surface on launch-critical layouts. | The editor survives but all project, review, or recovery context disappears silently. |
| Text scale | Larger text increases control height, row height, and wrapping before truncating critical copy. | Text spills outside buttons, chips, rows, or headers. |
| Line height | Editor line height remains configurable and defaults to the design-system 1.55-1.65 range; UI line height does not collapse below 1.35. | Dense rows compress text until glyphs collide or cursor geometry becomes ambiguous. |
| Cursor | Cursor thickness, animation, and blink are configurable; reduced motion and power saving may suppress blink without hiding caret location. | Cursor blink or animation is the only cue for insertion point. |
| Dense chrome | Low-frequency chrome may move to overflow, sheet, command palette, or region navigation. | Secondary controls disappear without a keyboard route or disclosure. |
| Minimap/overview | Minimap and overview rendering may simplify, hide, or become text summary at high zoom or larger text. | A minimap-only diagnostic or navigation cue remains task-critical. |
| Virtualization | Virtualized lists preserve focused row identity, selected counts, hidden counts, and blocked reasons as rows recycle. | Focus jumps by row index or selected counts lose hidden/blocked truth. |

Zoom checkpoints:

| Effective zoom | Minimum checkpoint |
|---|---|
| 50% | Hit targets, focus ring, caret, and active region remain perceivable; text may be smaller only within user-requested zoom. |
| 100% | Standard design-system density and typography apply. |
| 150% | Row heights, code gutters, labels, and status surfaces reflow without losing focus order. |
| 200% | Core workflows expose primary state, focus, blocked reasons, and recovery actions. |
| 300% | Secondary panels collapse in a predictable order; region navigation reaches overflowed controls. |
| 400% | One editor group and one critical side surface remain usable, or the launch-critical claim is narrowed with a visible disclosure. |

Safe simplifications at high zoom:

- Minimap may collapse to a diagnostic/change summary plus open-command
  route.
- Overview boards may switch from multi-column canvas to list/table
  summary.
- Inline row actions may move behind a focused action menu when the row
  still exposes the same command routes.
- Secondary inspectors may become sheets if focus return and region
  navigation remain stable.
- Decorative badges, timestamps, avatars, and low-frequency filters may
  move to details if identity, state, count, blocker, and recovery
  remain visible.

Unsafe simplifications:

- Hiding blocked reasons, selected counts, trust/policy state, or
  recovery actions.
- Turning a named action into an unlabeled icon without a keyboard path.
- Making one surface pointer-only because the control no longer fits.
- Leaving a canvas-only visualization without list, table, breadcrumb,
  summary, or command route.
- Silently disabling a control because platform scaling made it hard to
  fit.

## Pointer Independence

No launch-critical workflow may depend on drag-only, hover-only,
mouse-only, motion-only, or color-only interaction.

Required keyboard equivalents:

| Interaction | Minimum keyboard equivalent |
|---|---|
| Reorder tabs, panes, rows, steps, worktree items, stashes, or sequence entries | Move before/after, move to group, move to top/bottom, or explicit order field through command palette, focused menu, or shortcut. |
| Selection and range selection | Arrow navigation, Space toggle, Shift+arrow range extension, select all visible, select all matching, clear selection, and inspect hidden-selected count where supported. |
| Pane resize and split adjustment | Focus splitter, grow/shrink by step, snap preset, reset size, move panel to sheet or side, and restore previous layout. |
| Review actions | Approve, reject, request changes, comment, resolve, apply, revert, export, and open evidence through focused controls or command routes. |
| Drag/drop import, attach, or move | Add/import/move command, preview consequence, choose destination, confirm/cancel, and undo or recovery path where supported. |
| Canvas, graph, timeline, or visual inspector navigation | Equivalent table/list/breadcrumb path, selected-node detail, range fields or keyboard stepping, and open-source route. |

Drag and pointer actions may remain as accelerators. They are
conforming only when:

- the target verb is visible before commit;
- keyboard or command path reaches the same action class;
- modifier behavior is exposed near the target or in the focused action
  surface;
- destructive or broad actions route through preview, confirmation, or
  checkpoint posture;
- support/release evidence can cite the equivalent command route.

The checked-in no-drag inventory is the debt ledger. A row marked as
`pointer_only_debt` is not allowed in a stable launch-critical path
until it has a replacement path and verification fixture.

## Disclosure States

When current zoom, larger text, high contrast, reduced motion, platform
scaling, density, or input modality reduces a control, Aureline must
disclose what changed and how to continue.

Disclosure states:

| State | Meaning | Required user-facing behavior |
|---|---|---|
| `control_moved_to_overflow` | Action still exists but moved into overflow, menu, palette, or region navigation. | The focused owner exposes the overflow route and the action remains command-searchable. |
| `control_reduced_to_icon_with_label` | Control is visually compact but keeps an accessible label and tooltip/focus description. | The label is available on focus and through the accessibility tree. |
| `control_hidden_until_region_focus` | Low-frequency control appears when its region receives focus. | Region navigation must reach it; it may not be hover-only. |
| `control_unavailable_at_current_scale` | Control cannot operate safely at the current scale. | Show reason, safe fallback, and how to change scale or open an alternate surface. |
| `minimap_simplified` | Minimap details are reduced. | Provide summary, open command, and equivalent diagnostics/change routes. |
| `overview_simplified` | Canvas/overview changed to list, table, summary, or sheet. | Preserve selected object, count, filter, and source/open routes. |
| `dense_chrome_compacted` | Secondary dense metadata moved out of the primary row. | Preserve identity, state, blocker, count, and next action in the row. |
| `platform_scaling_limited` | Platform or toolkit scaling limits exact rendering. | Name the limitation and preserve a usable fallback path. |

Silent clipping, silent disabling, silent command removal, or "just
resize the window" behavior is non-conforming.

## Evidence Rules

Any packet or fixture citing this contract must show:

- one focus owner per active window;
- rendered focus indicator width and contrast;
- focus-return state and fallback when a transient surface closes;
- effective zoom percent, text scale percent, line-height mode, cursor
  setting, contrast state, density state, and platform scale class;
- whether one editor group and one critical side surface remain visible
  on launch-critical high-zoom layouts;
- every pointer-accelerated action's keyboard or command equivalent;
- disclosure rows for controls that moved, reduced, simplified, or
  became unavailable.

Minimum fixture coverage:

- 400% zoom or equivalent high-scale layout preserving one editor group
  and one critical side surface;
- overlay focus return when the invoker remains valid;
- overlay focus return when the invoker vanished and the nearest safe
  ancestor or placeholder must be used;
- larger text with increased line height and cursor controls;
- keyboard-only reorder, range selection, pane resize, and review
  action routes;
- at least one explicit no-drag-only inventory row for remaining
  pointer-only debt, even if the debt is outside launch-critical scope.

## Change Discipline

Adding a new focus target role, disclosure state, zoom posture, pointer
equivalence status, or denial reason is additive-minor and requires the
schema, this document, and at least one fixture or inventory row to be
updated together. Repurposing an existing value is breaking and
requires a governed decision row.
