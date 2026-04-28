# Table-grid, column-preset, sort-filter, and export contract

This document is the product-wide contract for dense table and grid
surfaces. It applies to review queues, result grids, schema explorers,
support tables, admin grids, evidence tables, and any future surface
that renders structured rows with columns, sort, filters, selection,
copy, or export.

The contract is normative. It composes with the broader collection
contract instead of redefining it:

- [`/docs/ux/collection_view_contract.md`](./collection_view_contract.md)
  owns filter ASTs, saved views, batch-review packets, count truth, and
  the visible / loaded / matching vocabulary.
- [`/docs/ux/selection_and_scope_contract.md`](./selection_and_scope_contract.md)
  owns focus, current item, checked state, row selection, range anchors,
  hidden selected rows, not-loaded selected rows, and visible-to-matching
  escalation.
- [`/docs/ux/output_log_viewer_contract.md`](./output_log_viewer_contract.md)
  owns generic output viewer posture for structured result rows,
  truncation, live streams, active-content blocking, and copy/export
  representation.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  owns cross-surface freshness and captured-vs-live truth.

The companion artifacts are:

- [`/schemas/ux/table_view_state.schema.json`](../../schemas/ux/table_view_state.schema.json)
  - boundary schema for one table view-state record.
- [`/fixtures/ux/table_grid_cases/`](../../fixtures/ux/table_grid_cases/)
  - worked fixtures covering large virtualized grids, redacted cells,
  filter-preserving exports, and mixed fresh/stale rows.

This document does not define a table renderer implementation. It
freezes the state shape and interaction rules later renderers,
extensions, CLI exports, support tooling, and accessibility tests must
read.

## Scope

A table or grid surface conforms when it emits one
`table_view_state_record` for any state that is persisted, reopened,
copied, exported, handed off, shared with support, or used to drive a
batch action.

The record answers eight questions:

1. What row population is visible, loaded, matching, selected, hidden,
   blocked, or not loaded?
2. Which column preset, hidden columns, frozen columns, and column
   widths are active?
3. Which sort basis and filter/query chips produced the current row
   order?
4. Which cells carry stale, approximate, redacted, policy-hidden, or
   provider-lag cues?
5. Which action is row-level, which action is cell-level, and what does
   Enter or Space do?
6. What survives copy, export, handoff, reopen, and support capture?
7. Which viewport/virtualization rules preserve header context and
   range-selection anchors?
8. Which consequential actions require an explicit review step before
   touching more than the visible subset?

## Table Families

Closed table-family vocabulary:

- `review_queue_table`
- `result_grid_table`
- `schema_explorer_table`
- `support_table`
- `admin_data_table`
- `evidence_table`
- `variable_table`

Each table family must still map to one collection `surface_class` when
it participates in selection or batch review. For example,
`result_grid_table` usually maps to `search_collection` or
`log_or_event_collection`; `admin_data_table` maps to
`admin_or_settings_grid`.

## Row Population And Scope Counts

Every table must keep these row populations separate:

| Term | Meaning | Required behavior |
|---|---|---|
| `visible_rows` | Rows rendered in the current viewport or page. | Never label as all rows. |
| `loaded_rows` | Rows materialized by the client or provider cursor. | State may survive scroll, pagination, or virtualized unmount. |
| `matching_rows` | Rows matching the active filter/query basis. | Count may be exact, approximate, stale, cached, provider-limited, partial, or unknown. |
| `selected_rows` | Stable row identities admitted by the user. | Selection follows identity, never row position. |
| `hidden_rows` | Rows hidden by filter, policy, column preset, grouping, or viewport. | Hidden selection count must remain visible when non-zero. |
| `blocked_rows` | Rows ineligible for the next action. | Cell cues cannot hide row-level blockers. |
| `not_loaded_rows` | Selected or matching identities whose bodies are not materialized client-side. | Export/review must say whether body fetch is required, denied, or skipped. |

The unqualified label `Select all` is forbidden. Controls must name
their real scope:

- `Select visible rows`
- `Select loaded rows`
- `Select all matching rows`
- `Select current filtered result`

When a table can broaden from visible or loaded rows to all matching
rows, the broadening must be a second explicit step. The broadening
review names the visible count, loaded count, matching count, hidden
selected count, not-loaded count, blocked count if known, provider-limit
status, and query/freshness basis.

## Column Model

Every visible or persisted column carries:

- stable `column_id_ref`;
- redaction-aware display label;
- semantic role;
- data type class;
- width and resize policy;
- visibility state;
- frozen/pinned state;
- sort eligibility and active sort state;
- filter eligibility and active filter chip refs;
- copy/export inclusion policy;
- fallback state when a provider, policy, preset, or viewport cannot
  render the column.

### Column Visibility

Closed visibility vocabulary:

- `visible`
- `hidden_by_user`
- `hidden_by_preset`
- `hidden_by_policy`
- `collapsed_before_truncate`
- `unavailable_from_provider`

Hidden columns remain part of table state. A hidden column must say why
it is hidden, whether export includes it, and whether the user can
restore it. `hidden_by_policy` is not equivalent to `redacted`: a
policy-hidden column may have no cell at all, while a redacted cell
preserves the fact that a value exists but cannot be shown.

### Frozen Columns

Closed frozen-position vocabulary:

- `not_frozen`
- `frozen_start`
- `frozen_end`

Frozen columns must remain visually and semantically attached to the
row identity they describe. A frozen status cell cannot scroll away from
the row blocker it summarizes. If the viewport is too narrow, low-
priority columns collapse before important columns truncate; frozen
identity and row-status columns are last to collapse.

### Resizing And Collapse

Column resizing must be keyboard reachable and screen-reader
announced. Width changes persist only as table view state, not as raw
application layout. A table must expose:

- min, max, and current width class;
- whether resize is user, preset, provider, or policy controlled;
- collapse priority;
- whether collapsed content is still available in a row detail panel,
  cell detail panel, copy/export, or not at all.

Important text must not be hidden by truncation while empty or low-value
columns stay visible. `collapsed_before_truncate` is the required state
when width pressure hides a low-priority column to preserve primary
identity, blockers, freshness, redaction, or action safety.

## Sort And Filter Basis

Sort state is a basis, not decoration. Each active sort term carries:

- `sort_key_ref`;
- direction;
- priority order;
- source (`provider`, `client`, `saved_view`, or `policy`);
- null placement;
- stability class;
- freshness and provider-limit posture when the provider performed the
  sort.

Filter and query chips carry:

- chip id;
- redaction-aware label;
- source (`user`, `saved_view`, `policy`, `client_scope`, or
  `provider`);
- state (`active`, `locked`, `unavailable`, `stale`, or `partial`);
- whether removal is allowed;
- count impact when known.

Policy chips and hidden narrowing must be visible beside user chips.
Export and handoff packets preserve the typed filter refs and reviewable
labels; raw query bodies, raw provider URLs, raw row text, and secret-
bearing literals do not cross the table view-state boundary.

## Column Presets

A column preset is a portable view policy for columns. It may capture:

- ordered column ids;
- hidden column reasons;
- frozen start/end columns;
- widths and collapse priorities;
- active sort refs;
- active filter refs;
- density class;
- copy/export column policy.

Closed preset-owner vocabulary:

- `user_local`
- `workspace_shared`
- `team_shared`
- `policy_locked`
- `provider_default`
- `support_capture`

Restoring a preset must not silently drop unavailable columns or policy
changes. The restored table must disclose any unavailable, newly hidden,
newly policy-hidden, or fallback-mapped column and offer a reset path
unless the preset is policy locked.

## Cell Truth Cues

Cells may not imply precision or authority they do not have. The table
state must preserve these cell cues:

| Cue | Meaning | Required behavior |
|---|---|---|
| `fresh_value` | Current value from the stated authority. | No warning cue required. |
| `stale_value` | Value may be outdated under the current freshness basis. | Show stale cue in cell and summary when it affects selection/export. |
| `approximate_value` | Value is estimated, sampled, rounded, or provider-limited. | Use approximate count/measurement language in cell, copy, and export. |
| `policy_hidden` | Policy denies showing this cell or column. | Preserve policy boundary and do not export raw value. |
| `redacted` | Value exists but is masked for privacy/security. | Export redacted representation unless a reviewed local-only reveal allows more. |
| `provider_lag` | Provider has not caught up to the local or requested basis. | Pair with provider and freshness refs; do not show as authoritative. |
| `unavailable` | Value cannot currently be resolved. | Provide reason and recovery route when available. |

Cell-level cues never demote row-level blockers. If one cell is
`fresh_value` but the row has a policy blocker, the row blocker remains
visible in row chrome, selection summaries, review sheets, and export
manifests.

## Row Actions Versus Cell Actions

Tables must keep row selection, cell focus, row activation, cell
activation, and checked state separate.

- Arrow keys move cell focus by default in grid mode.
- Row focus can follow cell focus, but the focused cell is not
  automatically selected.
- Space toggles row selection only when the focused row has selection
  semantics; otherwise it performs the focused control action and must
  announce that it is not row selection.
- Enter activates the focused cell action when a cell action is focused;
  otherwise it activates the current row's primary action.
- Row-level batch actions read the selection state from the collection
  contract.
- Cell-level actions may inspect, copy, open detail, reveal lineage, or
  explain a cue for one cell. They do not mutate row selection unless
  the user invokes a selection command.

Cell action menus must not hide row blockers, redaction boundaries,
authority boundaries, or export scope.

## Viewport And Virtualization

Large tables virtualize rows early and may virtualize columns when
needed. Virtualization must preserve:

- stable row identity;
- range-selection anchor;
- selected, hidden selected, and not-loaded identity sets;
- sticky header context;
- frozen columns;
- current cell coordinates;
- scroll restoration by identity plus offset, not by row index alone;
- screen-reader row/column counts and approximate/unknown status.

Sticky headers must show enough context to interpret cells:

- table title or object label;
- active saved view or preset label;
- sorted columns and direction;
- active filter/query chip summary;
- visible/loaded/matching count truth;
- selected and hidden selected count when non-zero;
- export/copy scope when an export action is visible.

When rows stream in or a provider reorders data while the user is
reviewing, the table must buffer or freeze the live update according to
the live-set contract. A destructive, remote, or export/share action
must review a pinned snapshot or deny with a stale-basis explanation.

## Copy And Export

Copy and export must name both row scope and column scope.

Closed row-scope vocabulary:

- `current_cell`
- `current_row`
- `visible_rows`
- `loaded_rows`
- `selected_rows`
- `all_matching_rows`
- `filter_preserving_snapshot`

Closed column-scope vocabulary:

- `visible_columns`
- `visible_columns_plus_identity`
- `preset_columns`
- `all_user_visible_columns`
- `redacted_safe_columns`
- `explicit_reviewed_columns`

Default copy uses visible rows/cells only. Export may use a wider scope
only when the export review names:

- row scope and count truth;
- column scope and hidden-column treatment;
- filter/query refs preserved;
- sort basis preserved;
- redaction and policy-hidden treatment;
- stale, approximate, provider-lag, and unavailable cues preserved;
- blocked, skipped, hidden selected, and not-loaded counts;
- whether the export can be replayed, reopened, or handed off.

Filter-preserving export does not mean full-result export. A
filter-preserving export records the filter/query/sort/preset basis and
the chosen row scope. If the chosen row scope is visible or loaded rows,
the export must say so even when it carries the filter AST.

Remote, destructive, provider-owned, or share/export actions that widen
beyond the visible subset require a batch-review sheet. The sheet may
continue only after it shows included, excluded, blocked, hidden,
not-loaded, redacted, and policy-hidden treatment separately.

## Persistence, Handoff, And Reopen

The following survives handoff and reopen when policy allows:

- active column preset ref and drift state;
- column order, hidden state, frozen state, widths, and collapse
  priorities;
- active sort basis;
- active filter/query chip refs and reviewable labels;
- selection state by stable identity or provider selection ref;
- range anchor by stable row identity;
- selected hidden and not-loaded identity sets;
- copy/export review basis and redaction policy;
- viewport identity anchor and sticky-header context.

The following does not survive as authoritative truth:

- row positions without stable identities;
- cell values without freshness and authority context;
- client-only approximations represented as exact counts;
- raw secret-bearing filter literals;
- raw redacted values;
- transient hover/focus state unless captured by a review or support
  packet.

On reopen, a table must disclose drift before using restored state for a
consequential action. If a selection, filter, provider basis, column
preset, or policy epoch no longer resolves, the table must either
review the drift, fall back with disclosure, or deny the action.

## Accessibility Requirements

Tables must be keyboard complete and screen-reader legible:

- The accessibility tree exposes table role, row count, column count,
  approximate/unknown count status, sticky header labels, row headers,
  column headers, and selected state.
- Cell focus and row selection are separately announced.
- Range selection announces the stable anchor and updated count.
- Hidden selected and not-loaded counts are announced when non-zero.
- Redacted, policy-hidden, stale, approximate, provider-lag, blocked,
  and unavailable cues have text equivalents.
- Column resize is reachable from keyboard and announces current width
  class.
- Frozen columns and sticky headers do not duplicate focusable elements
  into conflicting accessibility nodes.
- Copy/export commands announce row scope, column scope, and redaction
  treatment before execution when the scope is wider than the current
  cell or visible rows.

## Fixture Acceptance

A table-grid fixture conforms when it demonstrates at least one of the
following without relying on row position:

- large virtualized grid with sticky headers, frozen columns, stable
  range anchor, and explicit `Select visible rows` or `Select all
  matching rows` behavior;
- redacted or policy-hidden cells whose cell cues do not hide row-level
  blockers;
- filter-preserving export that names row scope, column scope, hidden
  columns, redaction, and preserved filter/sort/preset refs;
- mixed fresh/stale or approximate rows where each cell cue survives
  copy/export and reopened review state.

Any fixture is non-conforming if:

- `Select all` appears without a visible, loaded, or all-matching scope;
- range selection is anchored by row index instead of stable identity;
- hidden selected rows disappear after filtering;
- a redacted or policy-hidden cell exports raw value by default;
- a stale or approximate value exports as exact without cue metadata;
- a cell-level status hides a row-level blocker;
- a sticky header disappears in a large grid without alternate context.
