# Accessibility Tree Contract

Status: seeded

This contract freezes the reusable accessibility-tree node taxonomy,
role/name/state mapping, virtualization rules, and inspector snapshot
shape for Aureline's custom-rendered surfaces. It applies to shell
zones, editor content, gutters, diagnostics, inline widgets, lists,
trees, tables, logs, notebooks, status items, notifications, and
support/export surfaces that need to be reachable by assistive
technology or diagnosable by support and quality engineering.

Contract identity:

- `accessibility_tree_contract_id:
  aureline.accessibility.tree_node_taxonomy`
- `accessibility_tree_contract_revision: 1`
- `tree_node_schema_version: 1`
- `a11y_inspector_snapshot_schema_version: 1`

Companion artifacts:

- [`/schemas/accessibility/tree_node.schema.json`](../../schemas/accessibility/tree_node.schema.json)
  defines individual accessibility-tree nodes and tree-node case
  records.
- [`/schemas/accessibility/a11y_inspector_snapshot.schema.json`](../../schemas/accessibility/a11y_inspector_snapshot.schema.json)
  defines support, developer, and quality-engineering captures of the
  active tree, focus chain, selection state, row-index truth, and
  announcement contract posture.
- [`/fixtures/accessibility/tree_node_cases/`](../../fixtures/accessibility/tree_node_cases/)
  contains seed cases for shell/editor nodes, virtualized collections,
  log/notebook/status surfaces, and inspector snapshots.
- [`/docs/accessibility/screen_reader_and_live_region_contract.md`](./screen_reader_and_live_region_contract.md)
  defines the live-region and announcement contract this tree carries.
- [`/docs/accessibility/collection_announcement_contract.md`](./collection_announcement_contract.md)
  defines the dense-collection announcement projection for row
  position, selected count, hidden members, blocked/read-only state,
  batch scope, stale query snapshots, and keyboard-help parity.
- [`/docs/accessibility/focus_zoom_and_pointer_independence_contract.md`](./focus_zoom_and_pointer_independence_contract.md)
  defines focus-owner and zoom rules that tree snapshots must expose.
- [`/artifacts/accessibility/accessibility_tree_coverage_rows.yaml`](../../artifacts/accessibility/accessibility_tree_coverage_rows.yaml)
  names the shell coverage rows that cite tree captures.
- [`/docs/adr/0016-shell-windowing-input-accessibility-boundary.md`](../adr/0016-shell-windowing-input-accessibility-boundary.md)
  freezes shell ownership of accessibility-tree lifecycle and platform
  bridge boundaries.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` Sections 3.9, 5.37, and 11 require the
  custom shell to bridge into OS accessibility APIs, preserve keyboard
  and screen-reader reachability, expose diagnostics and terminal
  semantics, and keep accessibility as an architecture constraint.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Sections 8.6,
  12.7, 23, Appendix AD, and Appendix BU require visible degraded
  states, window-local focus truth, semantic accessibility surfaces,
  and assistive-technology regression journeys.
- `.t2/docs/Aureline_Technical_Design_Document.md` Sections 4.4, 8.8,
  8.11, and 8.13 require screen-reader semantics across shell, tree,
  palette, diff, terminal, debugger, docs, learning, and notebook
  surfaces.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Section 19 requires
  keyboard-complete, screen-reader-addressable dense surfaces, stable
  announcements, semantic summaries, and honest stale/degraded truth.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` Section 28 and
  support-class badge guidance require dense surfaces to remain
  logically grouped, announce position and selected state, and use
  explicit support-class language when claims narrow.

## Scope

The accessibility tree is a host-owned semantic projection of current
product truth. It is not a paint tree, DOM clone, render list, or
platform-adapter cache. The shell and surface view models own semantic
identity; the renderer publishes node deltas from that truth; platform
adapters bridge the nodes to UIA, NSAccessibility, or AT-SPI.

In scope:

- semantic nodes for host-rendered shell chrome and custom-rendered
  content;
- stable role/name/state mappings for dense, dynamic, and virtualized
  surfaces;
- focus, selection, row, column, set-position, live-region, and
  relationship facts needed by assistive technology;
- degraded and support-class posture when a surface cannot yet provide
  full semantics;
- inspector and support snapshots that are safe to export without raw
  source text, raw paths, terminal bytes, prompts, secrets, or
  unredacted user identifiers.

Out of scope:

- implementing every OS adapter or assistive-technology automation
  runner;
- byte-exact vendor-specific screen-reader strings;
- exposing raw document, terminal, notebook, or preview contents in
  support bundles;
- requiring every decorative visual affordance to have its own node
  when it does not communicate task-critical meaning.

## Core Invariants

1. A visible host-owned control missing from the accessibility tree is a
   correctness bug.
2. Focus truth is independent of paint order, z order, render batching,
   and virtualization timing.
3. Accessible row, column, and position-in-set values come from the data
   model, not from currently mounted nodes.
4. A surface that cannot provide full live semantics must publish a
   truthful degraded or unsupported state; it must not silently omit
   the surface or claim normal assistive-technology parity.
5. Accessible names, descriptions, state labels, and announcements use
   stable message IDs or bounded labels. Logic never depends on
   localized prose.
6. Tree, focus, selection, command, and announcement records must reuse
   the same object refs and state vocabulary where they describe the
   same product truth.
7. Support and inspector exports are metadata-first. They may carry
   opaque refs, bounded labels, counts, state classes, row indices,
   schema refs, and redaction decisions; they may not carry raw private
   workspace material.

## Node Record Model

Every inspectable node uses `accessibility_tree_node_record`.

Minimum fields:

| Field | Meaning |
|---|---|
| `node_id` | Stable opaque node identity within the current tree epoch. It must not be a mount index or paint-order index. |
| `tree_epoch_ref` | Tree epoch that advances when structural meaning changes. Paint-only invalidations do not advance it. |
| `surface_family` | Owning surface family such as `editor_content`, `table_grid`, or `status_notification`. |
| `node_kind` | Product-level node taxonomy value. This is more specific than the platform role. |
| `role_mapping` | Generic role plus platform hints and role-confidence class. |
| `accessible_name` | Bounded label, source class, optional message ID, and placeholder names. |
| `states` | Focus, selection, disabled, read-only, expanded, checked, busy, invalid, current, stale, degraded, and virtualized state. |
| `position` | Position-in-set, row/column indices, counts, nesting level, and exactness class where applicable. |
| `relationships` | Parent/child refs plus labels, descriptions, controls, owns, details, error messages, flow, and source-anchor refs. |
| `virtualization` | Whether the node is mounted, offscreen, represented while unmounted, or summarized. |
| `support_status` | Support class, support state, degradation reasons, user-visible notice requirement, and recovery action. |
| `privacy` | Export redaction class, allowed export fields, and `raw_private_material_excluded: true`. |

Node IDs should be derived from stable surface/object identity and a tree
epoch, not from visible row position. For example, a virtualized table
row keeps the same `node_id` as it scrolls out and back in; only the
mount state and visible-window facts change.

## Node Taxonomy

The taxonomy below is closed for version 1. New node kinds are
additive-minor when they do not repurpose an existing value.

| Surface family | Required node kinds | Required mapping notes |
|---|---|---|
| `shell_zone` | `application_root`, `window_root`, `shell_zone`, `landmark_region`, `toolbar`, `tablist`, `tab`, `splitter`, `button`, `searchbox`, `status_bar`, `status_item`, `banner`, `placeholder` | Shell zones expose named regions for title/context, rail, sidebar, workspace, inspector, panel, status, and transient overlays. Adaptive collapse changes relationships, not identity. |
| `editor_content` | `editor_document`, `editor_line`, `editor_text_run`, `editor_caret`, `editor_selection_range`, `inline_widget`, `placeholder` | The editor exposes document, line, caret, selection, diagnostics, code-action, and inline-widget semantics without forcing screen readers through paint spans. |
| `editor_gutter` | `editor_gutter_lane`, `editor_gutter_marker`, `button`, `row_header`, `diagnostic_marker` | Gutter markers identify the controlled line or range, marker family, severity or breakpoint state, and action route. Color-only marker meaning is non-conforming. |
| `diagnostics` | `diagnostic_marker`, `diagnostic_row`, `list_container`, `list_row`, `button`, `live_region` | Diagnostics relate to editor ranges and problem rows through `error_message`, `described_by`, and `details` refs. Severity and stale/provider state are explicit state labels. |
| `inline_widget` | `inline_widget`, `button`, `menu_item`, `combobox`, `text_input`, `live_region` | Inline assists, hovers, code actions, and snippets name the command route, focus behavior, provider/source state, and whether mutation requires preview. |
| `list` | `list_container`, `list_row`, `button`, `status_item`, `placeholder` | Lists publish set size, position-in-set, selected/current/blocked/read-only state, hidden-selected counts, and disabled reasons. |
| `tree` | `tree_container`, `tree_row`, `button`, `placeholder` | Trees publish nesting level, expanded/collapsed state, position within parent, source/provenance state, and virtualization truth. |
| `table_grid` | `table_grid`, `row`, `column_header`, `row_header`, `cell`, `button`, `status_item` | Tables and grids publish row and column indices, counts, sort/filter scope, selected cells/rows, hidden counts, and raw-vs-rendered availability. |
| `log_terminal` | `log_region`, `log_entry`, `terminal_region`, `command_boundary`, `button`, `status_item` | Logs and terminals expose transcript/live-buffer state, command boundaries when known, local/remote host cue, exit status, and copy/export routes. |
| `notebook` | `notebook`, `notebook_cell`, `notebook_input`, `notebook_output`, `output_summary`, `table_grid`, `button`, `status_item` | Notebook cells expose cell index, type, execution state, kernel/session label, output trust/sandbox state, and textual or tabular output alternatives. |
| `status_notification` | `status_bar`, `status_item`, `notification`, `banner`, `live_region`, `button` | Status and notification surfaces name scope, severity, persistence, dismissal state, next action, and corresponding announcement event. |
| `diff_review` | `editor_document`, `editor_line`, `diagnostic_marker`, `review_hunk`, `list_row`, `button`, `status_item` | Diff and review rows preserve old/new side, hunk position, comment or finding state, reviewed/unreviewed state, and source anchor refs. |
| `support_export` | `support_downgrade_notice`, `unknown_degraded_summary`, `status_item` | Support projections expose what was omitted, why, support class, recovery path, and schema refs without raw private material. |

Canvas-like or graph-like surfaces are conforming only when they also
publish equivalent list, table, breadcrumb, or summary nodes for the
task-critical information.

## Role Mapping

`role_mapping.generic_role` uses the cross-platform vocabulary in the
schema. Platform adapters map those roles to UIA, NSAccessibility, or
AT-SPI. The generic role is the contract; native strings are adapter
projections.

Rules:

- Role mapping is based on semantic purpose, not visual shape. A row
  painted as a card is still a `listitem`, `treeitem`, `row`, or
  `gridcell` when it participates in those structures.
- Compound rows may expose child buttons or menus only when those child
  actions are independently focusable or have separate command routes.
- A node with a mutating action must expose the command route or
  disabled reason through relationships or state labels.
- Role confidence must be `exact` for stable claim-bearing surfaces.
  `degraded` or `summary_only` requires support-class narrowing.
- `none` is allowed only for grouping or decorative containers whose
  children carry all task-critical meaning.

## Names and Descriptions

Accessible names are short, stable, and bounded. Names come from:

- visible labels;
- command descriptors;
- document, symbol, or row identity;
- column or row headers;
- diagnostic or status message IDs;
- announcement message IDs;
- generated summaries for complex surfaces;
- degraded support labels when full semantics are unavailable.

Rules:

- Do not use raw paths, raw URLs, raw command lines, raw terminal bytes,
  raw prompts, secrets, or unredacted user identifiers as accessible
  names in exportable records.
- If a visible icon-only action exists, the tree name must carry the
  verb and target. The visual icon does not satisfy naming by itself.
- Descriptions carry reason and scope, not repeated chrome. For
  example, a disabled quick-fix row describes the provider, stale
  index, policy block, or preview requirement.
- Long, localized, or dynamic prose is referenced through `message_id`
  and placeholder names. Localized strings may change without changing
  node identity.

## State Mapping

State fields are explicit so support snapshots can distinguish similar
visual outcomes:

| State | Required when |
|---|---|
| `focused` | Node is the current focus owner or active descendant. |
| `focusable` | Node can receive keyboard focus directly. |
| `selected` | Node participates in selection, including offscreen selected rows. |
| `current` | Node is the current item, active line, active tab, current step, or active result without necessarily being selected. |
| `disabled` | Node is present but unavailable. A disabled node with user value must carry a disabled reason. |
| `read_only` | The object can be read or inspected but not edited in the current context. |
| `expanded` | Tree, disclosure, combobox, notebook cell, or row detail is expanded/collapsed. |
| `checked` | Checkbox, switch, toggle, breakpoint, or tri-state state applies. |
| `busy` | Surface is loading or refreshing and the busy state affects user action. |
| `invalid` | Input, diagnostic, config, or validation row has an error/warning state. |
| `stale` | The node's underlying data is cached, stale, imported, or awaiting refresh. |
| `degraded` | The node or surface has narrowed capability or semantic fidelity. |
| `virtualized` | The node participates in a virtualized collection or is represented while unmounted. |

State labels must reuse controlled severity, freshness, degraded, trust,
policy, support-class, and announcement vocabulary already used by the
owning surface. A state visible only through color, opacity, animation,
or hover is non-conforming.

## Position, Relationships, and Virtualization

Position truth survives recycling:

- `position_in_set` and `set_size` describe the logical collection, not
  the mounted window.
- `row_index`, `column_index`, `row_count`, and `column_count` are
  one-based user-facing values unless the schema field explicitly says
  it is zero-based.
- `index_truth_class` must be `exact`, `estimated`, `stale`, `degraded`,
  `unknown`, or `not_applicable`.
- `estimated` and `stale` values require a state label and support or
  recovery path when they affect user action.
- A selected row that is filtered out, paged out, or virtualized out
  remains represented through hidden-selected counts and stable object
  identity.

Relationship rules:

- `parent_node_ref` and `child_node_refs` describe semantic structure,
  not render containment.
- `labelled_by` and `described_by` connect controls to visible or
  generated labels and explanations.
- `controls` names the controlled panel, detail, editor range, output,
  or command target.
- `error_message` relates diagnostics or validation rows to their
  affected inputs, editor ranges, notebook cells, or config rows.
- `active_descendant` is required for composite widgets where focus
  remains on a container while a row, option, or cell is current.
- `details` links compact rows to durable inspectors or detail panes.
- `source_anchor_refs` use opaque object refs or source-anchor records,
  not raw file paths.

Virtualized surfaces must publish a `virtualization` block. If a
surface cannot preserve index truth under virtualization, it must
downgrade to summary/list pagination, static snapshot, or unsupported
state for the affected action instead of presenting mounted row order as
truth.

## Inspector Snapshot

The inspector snapshot is the support and quality-engineering capture
format for tree state. It does not replace platform AT automation; it
captures enough state to reproduce and triage accessibility defects
without requiring screen recordings alone.

Every `a11y_inspector_snapshot_record` includes:

- contract and schema versions;
- capture reason, timestamp, platform profile, bridge, locale, zoom,
  contrast, motion, and assistive-technology posture;
- active window and surface scope;
- tree summary counts, root node ref, role-group counts, and support
  class counts;
- active node projections with role, name, state tokens, position, and
  degradation reasons;
- focus chain and focus-return targets;
- selection model, anchor, active node, selected counts, hidden counts,
  and selection truth class;
- virtualized row/window facts for each active collection;
- announcement contract version, live-region nodes, last event refs,
  and coalescing/suppression facts;
- degraded surfaces, support-class language, user-visible notice node,
  and recovery actions;
- export-redaction decisions and allowed fields.

Support bundles and repro bundles should cite snapshot IDs and schema
refs. They should not embed raw tree labels that violate the privacy
rules above.

## Downgrade and Support-Class Rules

Accessibility downgrade is explicit product state. It is not omission.

Support-class language:

| Class | Meaning for accessibility tree claims |
|---|---|
| `certified` | Release-blocking accessibility evidence exists for the named surface and platform scope. Full role/name/state/relationship/virtualization rules pass. |
| `supported` | First-party tested and documented with explicit scope caveats. Any known narrowings remain visible in the tree and inspector. |
| `community` | Extension or community path exists. Host boundary chrome remains supported; extension-owned body must declare its own support state. |
| `experimental` | Preview or active-iteration surface. Must display experimental/degraded language and stay out of stable launch-critical claims. |
| `unsupported` | No claim-bearing accessibility path. Stable workflows must block, replace, or route to a supported fallback. |

Downgrade rules:

- Missing role, name, state, relationship, position, or live-region
  semantics on a stable claim-bearing surface forces the affected row to
  `experimental` or `unsupported` until fixed or waived.
- Bridge unavailable means keyboard routes remain available where
  possible, but assistive-technology parity is visibly degraded and the
  inspector records `platform_bridge_unavailable`.
- Virtualization without stable object identity or row/index truth
  degrades the collection to summary, static pages, or inspect-only
  rows.
- Embedded or extension surfaces may not hide host-owned trust,
  approval, recovery, or status nodes even when the embedded body is
  unsupported.
- A surface in `ProtectCore` or other resource-degraded state may
  reduce decorative detail, but it must preserve focus, selected
  counts, blocked reasons, recovery actions, and support notices.
- Unsupported controls remain discoverable when discoverability matters
  for safety or recovery; they expose a disabled or unsupported reason
  rather than disappearing.

Every downgraded node carries:

- `support_status.support_class`;
- `support_status.support_state`;
- one or more `degradation_reason_classes`;
- `user_visible_notice_required: true` unless the node is wholly outside
  claimed scope;
- a recovery, fallback, or scope-narrowing action where one exists.

## Fixture Expectations

Tree-node fixtures should exercise:

- shell zone, editor line, gutter, diagnostic, and inline-widget
  relationships;
- virtualized tree/list/table row identity, position truth, hidden
  selected counts, and active descendant behavior;
- log, terminal, notebook, output-summary, status, notification, and
  live-region mappings;
- degraded bridge, support-class narrowing, and user-visible support
  notices;
- inspector snapshots that join tree nodes, focus chain, selection,
  row-index truth, and announcement contract version.

The fixtures are contract seeds. Full AT adapters, vendor-specific
automation, and final rendering implementation land separately, but
future implementations must be able to emit records that conform to
these schemas.
