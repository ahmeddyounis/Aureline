# Range-selection, multi-select, and batch-action scope-preview contract

This document freezes the UX-side contract every dense collection
surface obeys when the user selects more than one item and the next
action has consequence. It applies to search results, problems,
diagnostics queues, dependency tables, review inboxes, pipeline logs,
package inventories, work-item lists, admin grids, and any future
surface where a single user gesture can act on more than one row.

The contract is normative. Where this document disagrees with the
source UI / UX, interaction-safety, collection-view, filter-AST, or
governance contracts it cites, the source contract wins and this
document, its two schemas, and its fixtures update in the same change.
Where this document disagrees with a downstream surface's private
widget behavior, this document wins and the surface is non-conforming.

This contract composes with, and does not replace:

- [`/docs/ux/selection_and_scope_contract.md`](./selection_and_scope_contract.md)
  for the broader collection-side selection model. The records here
  are the UX-namespace boundary that surface code emits to UI,
  accessibility, support export, and review tooling. The collection
  contract owns the durable serialized boundary in
  [`/schemas/collections/selection_state.schema.json`](../../schemas/collections/selection_state.schema.json)
  and [`/schemas/collections/batch_review_packet.schema.json`](../../schemas/collections/batch_review_packet.schema.json);
  every UX record cites the matching collection record by ref so a
  single source of truth governs identity, count truth, and apply-time
  drift labelling.
- [`/docs/ux/collection_view_contract.md`](./collection_view_contract.md)
  for filter ASTs, saved views, and the durable batch-review packet.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  and [`/schemas/ux/interaction_safety.schema.json`](../../schemas/ux/interaction_safety.schema.json)
  for preview / apply / revert posture, recovery class, batch-scope
  packet, focus-return state, and copy / export representation. Every
  consequential apply emitted from a batch-action review record pairs
  to exactly one `interaction_safety_packet_record`.
- [`/docs/ux/editor_selection_contract.md`](./editor_selection_contract.md)
  for source-editor caret, multi-cursor, column-selection, and IME
  composition. Source-editor text ranges are intentionally outside
  this contract; multi-cursor must not borrow dense-collection
  multi-select vocabulary.
- [`/docs/ux/table_grid_contract.md`](./table_grid_contract.md),
  [`/docs/ux/list_and_card_row_contract.md`](./list_and_card_row_contract.md),
  and [`/docs/ux/tree_view_contract.md`](./tree_view_contract.md)
  for the host row / cell / tree presentations. Those contracts own
  the row chrome; this contract owns the multi-select gesture, scope,
  and review semantics that ride on top.

Companion artifacts:

- [`/schemas/ux/selection_state.schema.json`](../../schemas/ux/selection_state.schema.json)
  — boundary schema for one `selection_state_record`. Carries the
  live focus / current-item / activation / checked-state / selection
  distinction, keyboard and pointer gesture contract, identity-rule
  posture, scope label and counters, hidden-selected and not-loaded
  disclosure, and visible-to-matching escalation state.
- [`/schemas/ux/batch_action_review.schema.json`](../../schemas/ux/batch_action_review.schema.json)
  — boundary schema for one `batch_action_review_record`. Carries the
  per-action review requirement, included / excluded / blocked /
  hidden-selected / not-loaded / server-side count summaries, recovery
  class, focus-return target, and the paired interaction-safety and
  durable-packet refs.
- [`/fixtures/ux/selection_batch_action_cases/`](../../fixtures/ux/selection_batch_action_cases/)
  — worked YAML fixtures for the acceptance scenarios: keyboard range
  selection across virtualized rows, hidden selected rows after a
  filter change, and a remote batch action requiring explicit review
  of the affected scope before continuation.

## Scope

A surface emits one or more of these records when it offers any
multi-row gesture (Shift-range, Ctrl/Cmd-toggle, Select visible,
Select loaded, Select all matching, custom identity set), or when it
opens a batch action whose execution affects more than one item.

A surface that renders a multi-select control without publishing the
selection state to assistive technology, support export, and review
tooling is non-conforming. A surface that opens a remote, destructive,
provider-owned, export, or share batch action without a paired
`batch_action_review_record` is non-conforming.

Single-item routine actions (open, focus, copy current cell) do not
require a `batch_action_review_record`. They still ride on the
existing `selection_state_record` so focus / current-item /
activation distinctions remain visible to keyboard and AT users.

## State Model

Dense collection rows must keep five states distinct. Collapsing any
two into one denies via `focus_selection_activation_collapsed`.

| State | Meaning | Required behavior |
|---|---|---|
| `focus` | Where keyboard input goes now | Exactly one window-local focus owner is exposed to keyboard and assistive technology. Focus may move without changing selection. |
| `current_item` | The row driving the details, preview, or status pane | Current-row emphasis may follow focus; current-row is not selection. Activation acts on the current item. |
| `selection` | Stable item identities admitted to the current batch set | Selection survives sort, filter, pagination, streaming, and virtualization by identity; never by row position. |
| `checked_state` | A checkbox or tri-state control rendered in a row | A selection check cell may mirror selection. An independent boolean checkbox MUST disclose that the checked state is not batch selection. |
| `activation` | The command Enter, double-click, or the primary action invokes | Activation opens or acts on the current item; it MUST NOT silently widen or clear selection. |

A surface MUST publish:

- `focus_owner_id_ref` — the keyboard / AT focus owner.
- `focused_item_id_ref` — the focused row, or null when focus is on
  the selection bar, header, or another non-row owner.
- `current_item_id_ref` — the row backing details / preview / status,
  or null when no row is current.
- `activation_target_id_ref` — what Enter or primary-action invokes.
- `activation_behavior_class` — one of
  `activates_current_item_without_selection_change`,
  `opens_current_item_without_selection_change`, or
  `disabled_while_batch_review_open`.
- `checked_state_class` — one of
  `selection_check_cell_mirrors_selection`,
  `independent_boolean_not_selection`, or
  `tree_descendant_mixed_state`.
- `focus_selection_activation_distinct: true` — the surface explicitly
  separates these states.

## Keyboard contract

Required keyboard behavior:

- **Space** toggles selection for the focused row when the row has a
  selection role
  (`toggles_focused_selection_without_activation`). Space MUST NOT
  activate the row. If Space toggles an independent checked state
  instead (`toggles_focused_checked_state_without_activation`), the
  row MUST disclose that the checked state is not batch selection. A
  read-only collection emits `unavailable_readonly_announced`.
- **Shift+Up/Down/PageUp/PageDown/Home/End** extends from the visible
  or announced range anchor (`extends_from_visible_anchor`). Scrolling
  and virtualization MUST NOT move the anchor. If the dataset
  identity changed since the anchor was set, the action denies with
  `denied_dataset_identity_changed`. If no visible anchor exists, the
  action emits `unavailable_no_visible_anchor`.
- **Ctrl/Cmd+click**, **Ctrl/Cmd+Space**, or the equivalent AT toggle
  toggles the focused identity without clearing the rest of the set
  (`toggles_focused_identity`). Single-select collections emit
  `unavailable_single_select`.
- **Enter** activates the current item without changing selection
  (`activates_current_item_without_selection_change` /
  `opens_current_item_without_selection_change`). When a batch review
  sheet is open and pending, Enter denies with
  `denied_when_batch_scope_pending_review` rather than committing the
  current item silently.
- **Esc** dismisses the selection bar (when focused) or the batch
  review sheet without clearing the underlying selection. A second
  Esc on an empty selection bar focus returns focus to the row that
  opened the bar.
- **Clear selection** is a first-class keyboard and AT action exposed
  on at least one of `selection_bar_clear_button`,
  `command_palette_action`, `keyboard_shortcut`,
  `screen_reader_action`, `filter_chip_clear_hidden_selected`, or
  `escape_when_selection_bar_focused`. Clearing also resets the range
  anchor.

## Pointer contract

Required pointer behavior:

- **Click** on a row sets focus and current-item, but does not toggle
  selection on its own. Surfaces MAY rebind click to also toggle the
  selection check cell when the row's `checked_state_class` is
  `selection_check_cell_mirrors_selection`; the bound behavior MUST
  appear in `pointer_contract.click_behavior_class`.
- **Shift+click** extends the visible-range anchor to the clicked row.
  The anchor stays bound to the row that established it; subsequent
  Shift+clicks rebind only the head, not the anchor.
- **Ctrl/Cmd+click** toggles the clicked identity. Surfaces MUST NOT
  silently demote a `provider_owned_identity_pinned` selection to a
  position-based one when toggled.
- **Range drag** (rubber-band) extends visible-range only. The drop
  emits one final `selection_state_record`; intermediate drag frames
  do not need to emit. Drag across a virtualization boundary preserves
  the anchor.
- **Right-click / context menu** opens a context menu against the
  clicked row without changing selection unless the user explicitly
  picks a select-or-clear command from the menu.

## Scope labels and counters

Every collection with multi-select MUST expose a selection bar (or AT
equivalent) when one or more items are selected. The bar carries:

- `selected_count` — exact count of selected identities, with status
  one of `exact`, `approximate`, `provider_limited`, `stale`,
  `cached`, `partial`, `unknown`.
- `scope_label_class` — one of `current_item_only`, `visible_range`,
  `loaded_set`, `all_matching_query`, or `explicit_custom_set`.
- `scope_counter_label` — a redaction-aware reviewable string that
  states the scope and any qualifiers in plain language.
- `hidden_selected_count` — count of selected identities currently
  outside the visible filter / window. Required even when zero, so
  reviewers can verify the disclosure exists.
- `not_loaded_selected_count` — count of selected identities whose
  body is not currently materialised on the client. Required even
  when zero.
- `blocked_count` — count of pre-commit-ineligible selected items, if
  the surface already knows. May be `unknown`.
- `skipped_count` — post-commit no-op count. Empty before apply;
  populated only on apply outcome.
- `hidden_selected_disclosure_class` — one of
  `not_applicable_zero_hidden`, `visible_chip_with_reveal_and_clear`,
  or `review_sheet_breakout_only`. A non-zero hidden count without an
  explicit disclosure denies via `hidden_selected_count_missing`.
- `clear_selection_action_id_ref` — opaque ref to the clear action.
- `reset_path_label` — short reviewable string describing what
  clearing resets (anchor, hidden chip, focus return target).

The unqualified label `Select all` is forbidden. A control whose real
scope is only the rendered viewport MUST use visible-row language
(`Select visible rows`). A control whose real scope is only the
client cursor window MUST use loaded-row language (`Select loaded
rows`). Only a control that truly targets the query or provider
result set MAY use all-matching language (`Select all 247 matching
items`). The schema enforces this with a closed
`select_action_scope_class` and a label invariant.

Result counters MUST NOT collapse visible, loaded, matching,
selected, hidden selected, and not-loaded into one number. Every
count carries an explicit term and an explicit status. Promoting an
approximate / provider-limited / stale / cached / partial / unknown
count to `exact` denies via `count_status_missing`.

## Visible-to-matching escalation

Selection broadening from a visible or loaded scope to an all-matching
scope is a deliberate, two-step transition:

1. The user selects the current item, a visible range, a loaded set,
   or a custom identity set. The bar reads "N visible-range items
   selected" / "N loaded items selected" / "N items selected".
2. If an all-matching scope is available, the bar offers a second
   explicit action that names the matching count and freshness:
   "Select all 5,000 matching items". The action MUST quote the
   matching count and its status (e.g. `provider_limited`) when
   non-exact.
3. Before broadening, the surface emits a review whose summary names
   visible, loaded, matching, hidden_selected, not_loaded, and
   blocked counts and the provider-side scope status. The review is
   cancellable.
4. Cancelling returns to the prior visible / loaded / custom set. The
   prior anchor is preserved unless the user clears selection.
5. If the query, filter, sort, provider basis, dataset identity, or
   policy epoch changes after broadening, the next consequential
   action MUST route through review again or deny via
   `selection_scope_widened_without_review`.

Closed `scope_escalation_state` values:

- `not_available` — no all-matching scope is reachable for this
  collection (e.g. local-truth list with no matching count term).
- `offered_visible_to_matching` — the second-step action exists but
  the user has not chosen it.
- `review_pending` — the user opened the escalation review.
- `broadened_all_matching` — the user accepted the second step. Any
  basis change reverts this to `review_pending` or denies.
- `cancelled_to_prior_scope` — the user cancelled out of review.

## Identity rules

Selection follows stable item identity, never row position.
Position-based selection is permanently forbidden; the schema sets
`position_based_selection_forbidden: true` on every record.

Closed `churn_rule_class` membership for the live record MUST include
at least three of:

- `sort_filter_preserve_by_identity` — sorting and filtering rebind
  by identity; selection survives.
- `virtualized_unmount_preserves_membership` — virtualization may
  unmount rows but MUST NOT drop selected identities or move the
  range anchor.
- `pagination_uses_identity_not_position` — pagination and cursor
  windows rebase by identity. A selected item leaving the loaded page
  becomes `not_loaded`, never silently unselected.
- `streaming_new_items_not_auto_selected` — streaming arrivals are
  not auto-selected unless the user accepted an all-matching query
  scope that explicitly names future arrivals.
- `provider_requery_requires_review` — when the provider requeries on
  drift, the next batch action routes through review.

Identity classes:

- `stable_item_identity` — the host owns identity end-to-end.
- `provider_owned_identity_pinned` — the provider owns identity; the
  surface pins a `provider_side_selection_ref` when the visible
  selection is a projection of a provider-side set.
- `local_alias_identity_disclosed` — local aliases (e.g. ephemeral
  in-list ids) are admitted only when disclosed and never replayed
  across sessions as provider identity.

## Batch-action review

Consequential actions require preview or review. The closed
`batch_action_class` vocabulary maps to a closed
`review_requirement_class`:

| Action class | Required review |
|---|---|
| `routine_non_mutating` (e.g. open, copy current cell) | `no_review_required`. Current-item wording is enough. |
| `local_reversible` (e.g. mark read, tag, group, set local label) | `inline_preview_required` is acceptable when included and skipped counts are exact. |
| `remote_mutation`, `destructive_mutation`, `export_or_share`, or `provider_owned_mutation` | `review_sheet_required`. The sheet pairs to an `interaction_safety_packet_record` and a `batch_review_packet_record` before commit. |

The review record MUST publish, on a single screen / packet:

- `included_count` — items the action will attempt.
- `excluded_count` — items the user explicitly removed at commit.
- `blocked_summary` — per-reason blocked items
  (`blocked_by_policy`, `blocked_by_ownership`,
  `blocked_by_protected_path`, `blocked_by_provider_unsupported`,
  `blocked_by_freshness_required`, `blocked_by_grant_missing`,
  `blocked_by_concurrent_edit`).
- `hidden_selected_count` — selected identities outside the visible
  filter / window.
- `not_loaded_count` — selected identities whose body is not
  materialised on the client.
- `server_side_count` — provider-authoritative count for the matching
  scope on remote / provider-owned actions, with explicit status
  (e.g. `provider_limited` and a `tolerance_label`).
- `recovery_guidance_class` — one of
  `inline_undo_revert_available`, `compensating_action_only`,
  `regenerate_from_source`, `evidence_only_no_rerun`,
  `restore_from_checkpoint`, or `no_recovery_available`.
- `focus_target_id_ref` — where focus returns when the sheet closes.
- `cancel_action_id_ref` — first-class cancel route.

The schema enforces that:

- `remote_mutation`, `destructive_mutation`, `export_or_share`, and
  `provider_owned_mutation` require `review_sheet_required`.
- `all_matching_query` selection scope requires a `matching` count
  term in the review summary.
- `routine_non_mutating` MUST emit `no_review_required` and MUST NOT
  carry a review-sheet ref.
- `no_recovery_available` recovery class is admitted only when the
  paired `interaction_safety_packet_record` accepts an
  `irreversible_high_blast` posture; the record carries the paired
  ref so support tooling can audit that pairing.

`Blocked` items are pre-commit ineligible (refused before commit and
remain reviewable). `Unavailable` items are absent from evaluation
(provider offline, column redacted, filter unsupported by provider).
`Skipped` items are post-commit no-ops. These three counts MUST NOT
be collapsed into one "unavailable" bucket.

## Forbidden collapses

These collapses deny via the typed `selection_state_denial_reason`
or `batch_action_review_denial_reason` vocabulary:

- Collapsing focus, selection, current-item, checked state, or
  activation into a single "selected" state — denies via
  `focus_selection_activation_collapsed`.
- Treating an independent boolean checkbox as batch selection —
  denies via `checked_state_overclaimed_as_selection`.
- Rendering an unqualified `Select all` label whose true scope is
  only the visible viewport or the loaded window — denies via
  `ambiguous_select_all_label_forbidden` and
  `visible_or_loaded_scope_overclaimed`.
- Broadening from visible to all-matching without naming the matching
  count or freshness — denies via
  `selection_scope_widened_without_review`.
- Hiding a non-zero hidden-selected or not-loaded count — denies via
  `hidden_selected_count_missing` or `not_loaded_count_missing`.
- Rebinding selection by row position after a sort, filter,
  pagination, virtualization, or streaming churn — denies via
  `position_based_selection_forbidden` or
  `range_anchor_moved_by_virtualization`.
- Opening a `remote_mutation` / `destructive_mutation` /
  `export_or_share` / `provider_owned_mutation` apply path without
  routing through the review sheet — denies via
  `review_sheet_required_for_action_class`.
- Emitting a review record that collapses included, excluded,
  blocked, hidden, not-loaded, or server-side counts — denies via
  `batch_count_term_collapsed`.
- Promoting a provider-limited or stale count to `exact` on the
  review sheet — denies via `count_status_missing`.
- Surfacing a remote / provider-owned action's affected scope without
  a server-side count — denies via
  `server_side_count_missing_for_remote_action`.
- Carrying raw row text, raw query bodies, raw URLs, raw absolute
  paths, raw secret-bearing literals, or raw prompt text on either
  boundary record — denies via `raw_body_forbidden_on_boundary`.

## Acceptance

A seeded `selection_state_record` and `batch_action_review_record`
pair conforms only when a keyboard-only or AT user can determine:

- the current focus owner, current item, and activation target;
- whether Space will select, check, or do nothing;
- selected count, scope label, and any qualifier (visible / loaded /
  all matching / custom);
- whether scope has broadened beyond visible or loaded rows, with
  the matching count and status named;
- hidden-selected and not-loaded counts (even when zero);
- how to reveal hidden selected items;
- how to clear selection and reset the range anchor;
- whether the next batch action requires review and what its scope
  is;
- where focus returns after review or cancel.

Identity acceptance: any fixture that loses selection on sort,
filter, pagination, streaming, or virtualization is non-conforming.
Any fixture that uses position indexes as durable selection identity
is non-conforming.

## Worked fixtures

[`/fixtures/ux/selection_batch_action_cases/`](../../fixtures/ux/selection_batch_action_cases/)
holds three worked records:

- `keyboard_range_selection_virtualized.yaml` — a virtualized
  work-item collection where the user extends a Shift range with
  Shift+Down across rows that are unmounted by virtualization. The
  range anchor stays bound to the originating identity; sort and
  virtualization preserve the membership; only `local_reversible`
  inline preview is required for the next action.
- `hidden_selected_after_filter_change.yaml` — a review collection
  where an explicit custom selection survives a tighter filter. Five
  selected identities become `hidden_selected`; two become
  `not_loaded`. The selection bar exposes both counts with reveal /
  clear chips, and an `export_or_share` follow-up routes through a
  review sheet that shows included, excluded, blocked, hidden, and
  not-loaded counts.
- `remote_batch_action_explicit_review.yaml` — a search collection
  where the user escalates from `Select visible rows` to `Select all
  5,000 matching items` against a provider-authoritative index. The
  review sheet quotes the provider-limited matching count, three
  policy-blocked rows, and a server-side count, and the apply pairs
  to an `interaction_safety_packet_record` with
  `irreversible_high_blast` posture and `regenerate_from_source`
  recovery.

The manifest at
[`/fixtures/ux/selection_batch_action_cases/manifest.yaml`](../../fixtures/ux/selection_batch_action_cases/manifest.yaml)
names the schema each fixture validates against and the contract
sections it exercises.
