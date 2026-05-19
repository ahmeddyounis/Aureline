# Collection-truth corpus report

Generated from the seeded corpus in `crates/aureline-shell/src/collection_truth_corpus/mod.rs`.
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- report-md > \
  artifacts/qe/m3/collection_truth_report.md
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- matrix-json > \
  artifacts/qe/m3/collection_truth_matrix.json
```

- Packet id: `shell:collection_truth_corpus:packet:default`
- Shared contract ref: `shell:collection_truth_corpus:v1`
- Base packet shared contract ref: `shell:collection_truth_beta:v1`
- Generated at: `2026-05-18T00:00:00Z`

## Surface family matrix

| Surface | Edge cases exercised | Saved-view migrations | Drills | Ambiguous total | Hidden selected |
| ------- | -------------------- | --------------------- | ------ | --------------- | --------------- |
| `search_or_result_grid` | `virtualized_window_partial_matching`, `provider_capped_approximate_matching`, `blocked_rows_present`, `hidden_selected_rows_present` | `older_schema_version_upgraded_degraded` | `keyboard_anchor_range_selection` | yes | yes |
| `review_inbox` | `virtualized_window_partial_matching`, `blocked_rows_present`, `hidden_selected_rows_present` | (none) | `keyboard_batch_review_open` | yes | yes |
| `log_or_event_collection` | `provider_capped_approximate_matching`, `provider_retention_unknown_total`, `stale_provider_cursor_detected` | `stale_provider_cursor_refused_and_offered` | `saved_view_switcher_under_virtualization` | yes | no |
| `package_or_inventory_grid` | `blocked_rows_present` | `older_schema_version_upgraded_exact`, `stale_provider_owned_scope_offer_recreate` | (none) | no | no |
| `work_item_board` | `visible_equals_loaded_exact_total` | `unsupported_column_preset_dropped_labeled` | (none) | no | no |
| `admin_or_settings_grid` | `visible_equals_loaded_exact_total`, `blocked_rows_present`, `hidden_selected_rows_present` | `policy_narrowed_collection_rebound` | `screen_reader_hidden_selected_inspection` | no | yes |

## Edge-case class counts

- `blocked_rows_present` -- 4
- `hidden_selected_rows_present` -- 4
- `provider_capped_approximate_matching` -- 3
- `provider_retention_unknown_total` -- 2
- `stale_provider_cursor_detected` -- 1
- `virtualized_window_partial_matching` -- 3
- `visible_equals_loaded_exact_total` -- 3

## Saved-view migration class counts

- `older_schema_version_upgraded_degraded` -- 1
- `older_schema_version_upgraded_exact` -- 1
- `policy_narrowed_collection_rebound` -- 1
- `stale_provider_cursor_refused_and_offered` -- 1
- `stale_provider_owned_scope_offer_recreate` -- 1
- `unsupported_column_preset_dropped_labeled` -- 1

## Drill class counts

- `keyboard_anchor_range_selection` -- 1
- `keyboard_batch_review_open` -- 1
- `saved_view_switcher_under_virtualization` -- 1
- `screen_reader_hidden_selected_inspection` -- 1

## Corpus cases

### `corpus:case:search:export-selected` -- Search export — visible scope only (base)

- Surface: `search_or_result_grid`
- Edge cases: `virtualized_window_partial_matching`, `provider_capped_approximate_matching`, `blocked_rows_present`, `hidden_selected_rows_present`
- Count summary: `approximate_provider_limited`
- Hidden narrowing: `visibility: Internal projects only; Provider truncated at 5,000 matches`
- Anchor row: `anchor:search_or_result_grid:row:0`
- Selected ids: [`row:search_or_result_grid:included:0`, `row:search_or_result_grid:included:1`, `row:search_or_result_grid:included:2`, `row:search_or_result_grid:included:plus-more`]
- Hidden selected ids: [`row:search_or_result_grid:hidden-selected:0`, `row:search_or_result_grid:hidden-selected:1`]
- Blocked ids: [`row:search_or_result_grid:blocked:0`, `row:search_or_result_grid:blocked:1`]
- Batch action: `search.export_selected_matches` (export_or_share); continue enabled: true
- Select-all escalation: `visible_or_loaded`

### `corpus:case:review:approve-selected` -- Review inbox — selected approvals (base)

- Surface: `review_inbox`
- Edge cases: `virtualized_window_partial_matching`, `hidden_selected_rows_present`
- Count summary: `partial_indexing`
- Hidden narrowing: `workset: release branch; AI evidence still indexing`
- Anchor row: `anchor:review_inbox:row:0`
- Selected ids: [`row:review_inbox:included:0`, `row:review_inbox:included:1`, `row:review_inbox:included:2`, `row:review_inbox:included:plus-more`]
- Hidden selected ids: [`row:review_inbox:hidden-selected:0`, `row:review_inbox:hidden-selected:1`]
- Batch action: `review.approve_selected` (remote_mutation); continue enabled: true
- Select-all escalation: `visible_or_loaded`

### `corpus:case:logs:export-window` -- Log export — visible window (base)

- Surface: `log_or_event_collection`
- Edge cases: `provider_capped_approximate_matching`, `provider_retention_unknown_total`
- Count summary: `provider_retention_windowed`
- Hidden narrowing: `Provider retention 24 h; Client window 10,000 lines`
- Anchor row: `anchor:log_or_event_collection:row:0`
- Selected ids: [`row:log_or_event_collection:included:0`, `row:log_or_event_collection:included:1`, `row:log_or_event_collection:included:2`, `row:log_or_event_collection:included:plus-more`]
- Batch action: `logs.export_window` (export_or_share); continue enabled: true
- Select-all escalation: `visible_or_loaded`

### `corpus:case:packages:uninstall-selected` -- Inventory uninstall — admin pinned blocked (base)

- Surface: `package_or_inventory_grid`
- Edge cases: `blocked_rows_present`
- Count summary: `exact_local`
- Hidden narrowing: `marketplace: Org allowlist`
- Anchor row: `anchor:package_or_inventory_grid:row:0`
- Selected ids: [`row:package_or_inventory_grid:included:0`, `row:package_or_inventory_grid:included:1`, `row:package_or_inventory_grid:included:2`, `row:package_or_inventory_grid:included:plus-more`]
- Blocked ids: [`row:package_or_inventory_grid:blocked:0`, `row:package_or_inventory_grid:blocked:1`]
- Batch action: `packages.uninstall_selected` (destructive_local); continue enabled: true
- Select-all escalation: `all_matching_safe`

### `corpus:case:work-items:move-selected` -- Work items — all matching move (base)

- Surface: `work_item_board`
- Edge cases: `visible_equals_loaded_exact_total`
- Count summary: `exact_with_workset_narrowing`
- Hidden narrowing: `workset: current sprint`
- Anchor row: `anchor:work_item_board:row:0`
- Selected ids: [`row:work_item_board:included:0`, `row:work_item_board:included:1`, `row:work_item_board:included:2`, `row:work_item_board:included:plus-more`]
- Batch action: `work_items.move_selected` (remote_mutation); continue enabled: true
- Select-all escalation: `all_matching_safe`

### `corpus:case:admin:rotate-keys` -- Admin grid — rotate selected keys (base)

- Surface: `admin_or_settings_grid`
- Edge cases: `visible_equals_loaded_exact_total`
- Count summary: `exact_with_policy_pinning`
- Hidden narrowing: `tenant: Tenant A`
- Anchor row: `anchor:admin_or_settings_grid:row:0`
- Selected ids: [`row:admin_or_settings_grid:included:0`, `row:admin_or_settings_grid:included:1`]
- Batch action: `admin.rotate_keys` (destructive_remote); continue enabled: true
- Select-all escalation: `all_matching_safe`

### `corpus:review:virtualized-window` -- Review inbox -- virtualized window approve

- Surface: `review_inbox`
- Edge cases: `virtualized_window_partial_matching`, `blocked_rows_present`, `hidden_selected_rows_present`
- Count summary: `partial_indexing`
- Hidden narrowing: `workset: release branch; Reviews still loading`
- Anchor row: `anchor:review:virtualized-window:row:0`
- Selected ids: [`row:review:virtualized-window:included:0`, `row:review:virtualized-window:included:1`, `row:review:virtualized-window:included:plus-more`]
- Hidden selected ids: [`row:review:virtualized-window:hidden-selected:0`, `row:review:virtualized-window:hidden-selected:1`]
- Blocked ids: [`row:review:virtualized-window:blocked:0`, `row:review:virtualized-window:blocked:1`]
- Batch action: `review.approve_visible_window` (remote_mutation); continue enabled: true
- Select-all escalation: `visible_or_loaded`

### `corpus:logs:stale-cursor` -- Logs -- stale provider cursor

- Surface: `log_or_event_collection`
- Edge cases: `provider_capped_approximate_matching`, `provider_retention_unknown_total`, `stale_provider_cursor_detected`
- Count summary: `provider_retention_windowed`
- Hidden narrowing: `Provider cursor stale; Client window 5,000 lines`
- Anchor row: `anchor:logs:stale-cursor:row:0`
- Selected ids: []
- Batch action: `logs.rebind_cursor` (routine_non_mutating); continue enabled: true
- Select-all escalation: `visible_or_loaded`

### `corpus:admin:hidden-selected-revoke` -- Admin grid -- hidden selected revoke

- Surface: `admin_or_settings_grid`
- Edge cases: `visible_equals_loaded_exact_total`, `blocked_rows_present`, `hidden_selected_rows_present`
- Count summary: `exact_with_policy_pinning`
- Hidden narrowing: `tenant: Tenant A`
- Anchor row: `anchor:admin:hidden-selected:row:0`
- Selected ids: [`row:admin:hidden-selected:included:0`, `row:admin:hidden-selected:included:1`, `row:admin:hidden-selected:included:plus-more`]
- Hidden selected ids: [`row:admin:hidden-selected:hidden-selected:0`, `row:admin:hidden-selected:hidden-selected:1`]
- Blocked ids: [`row:admin:hidden-selected:blocked:0`, `row:admin:hidden-selected:blocked:1`]
- Batch action: `admin.revoke_invites_selected` (destructive_remote); continue enabled: true
- Select-all escalation: `visible_or_loaded`

## Saved-view migration cases

### `migration:packages:outdated:exact` -- `older_schema_version_upgraded_exact`

- Surface: `package_or_inventory_grid`
- Schema 0 -> 1
- Restored drift: `bound_current_state_matches_captured`; fallback: `preserve_and_label_degraded`
- Migration notes:
  - v0 -> v1 upgrade applied without semantic loss

### `migration:search:service-tier:degraded` -- `older_schema_version_upgraded_degraded`

- Surface: `search_or_result_grid`
- Schema 0 -> 1
- Restored drift: `column_set_drifted_disclosed`; fallback: `load_portable_subset_with_labels`
- Migration notes:
  - v0 -> v1 upgrade dropped removed columns and surfaced disclosure label
- Portability findings:
  - captured column `legacy_owner` not present in v1 schema

### `migration:work-items:my-board:unsupported-preset` -- `unsupported_column_preset_dropped_labeled`

- Surface: `work_item_board`
- Schema 1 -> 1
- Restored drift: `column_set_drifted_disclosed`; fallback: `load_portable_subset_with_labels`
- Migration notes:
  - Unsupported column preset dropped and labeled rather than silently misinterpreted
- Portability findings:
  - captured column `legacy_swimlane` is not supported on the current surface

### `migration:logs:stale-cursor` -- `stale_provider_cursor_refused_and_offered`

- Surface: `log_or_event_collection`
- Schema 0 -> 1
- Restored drift: `view_unavailable_provider_offline_disclosed`; fallback: `provider_rebind_required`
- Migration notes:
  - Stale provider cursor refused; rebind path offered
- Portability findings:
  - Provider cursors are never restored verbatim; rebind required

### `migration:packages:stale-provider-owned-scope` -- `stale_provider_owned_scope_offer_recreate`

- Surface: `package_or_inventory_grid`
- Schema 0 -> 1
- Restored drift: `view_unresolvable_offered_recreate`; fallback: `offer_recreate_from_current`
- Migration notes:
  - Provider-owned scope refused; recreate-from-current path offered
- Portability findings:
  - Provider-owned scope is never reused silently when the provider catalog changes

### `migration:admin:policy-narrowed-rebound` -- `policy_narrowed_collection_rebound`

- Surface: `admin_or_settings_grid`
- Schema 0 -> 1
- Restored drift: `policy_narrowing_changed_disclosed`; fallback: `refuse_until_rebound`
- Migration notes:
  - Policy narrowing changed since capture; rebind required before reuse
- Portability findings:
  - Captured policy-pinned scope is no longer authoritative

## Accessibility drills

### `drill:keyboard-anchor-range-selection` -- `keyboard_anchor_range_selection`

- Surface: `search_or_result_grid`
- Label: Anchor-based range selection across virtualized rows
- Steps:
  - Focus the first row to set the anchor
  - Press Shift+ArrowDown to extend the range to the next row
  - Press Shift+PageDown to extend across the virtualization window boundary
  - Press Shift+End to extend through the last loaded row
- Expected assertions:
  - Anchor row id stays stable across virtualization window scrolls
  - Range selection never includes blocked or hidden rows silently
  - Selected count narration matches `visible_or_loaded` escalation
- Virtualization invariants:
  - Anchor row remains in the loaded buffer after window recycling
  - Selection state survives a viewport scroll cycle
  - Loaded count never drops below the anchor row index
- Accessibility narration: `Selected 5 of 200 loaded rows; 0 hidden, 2 blocked; anchor row 1.`

### `drill:screen-reader-hidden-selected-inspection` -- `screen_reader_hidden_selected_inspection`

- Surface: `admin_or_settings_grid`
- Label: Inspect hidden-selected count via screen reader
- Steps:
  - Activate the scope counter strip via the keyboard
  - Navigate to the hidden-selected count row
  - Activate the Inspect hidden selected affordance
  - Verify the hidden-selected rows are narrated by their stable ids
- Expected assertions:
  - Hidden-selected count is non-zero and visible to the screen reader
  - Inspect affordance announces stable row ids without payload literals
  - Batch review summary hidden_count equals the hidden-selected count
- Virtualization invariants:
  - Hidden-selected rows remain countable when scrolled out of the viewport
  - Scope counter strip never collapses hidden into visible
- Accessibility narration: `3 selected rows are hidden by the current view. Inspect to review them before continuing.`

### `drill:keyboard-batch-review-open` -- `keyboard_batch_review_open`

- Surface: `review_inbox`
- Label: Open the batch-review sheet from a consequential action
- Steps:
  - Focus the consequential action affordance
  - Press Enter to request the action
  - Verify the batch-review sheet captures focus
  - Tab through included, excluded, blocked, and hidden count rows
  - Verify continue is disabled when the scope is ambiguous
- Expected assertions:
  - Sheet appears before destructive, export-bearing, or provider-backed actions
  - Sheet exposes included, excluded, blocked, and hidden rows distinctly
  - Continue control reflects `continue_enabled` from the record
- Virtualization invariants:
  - Sheet survives the virtualization window scrolling underneath
  - Selected count rendered on the sheet matches the count strip
- Accessibility narration: `Batch review: 6 included, 12 excluded, 0 blocked, 2 hidden. Continue available; cancel restores selection.`

### `drill:saved-view-switcher-virtualized` -- `saved_view_switcher_under_virtualization`

- Surface: `log_or_event_collection`
- Label: Switch saved view under virtualization
- Steps:
  - Focus the saved-view switcher
  - Select a drifted saved view
  - Verify the drift disclosure is narrated
  - Switch back and confirm anchor row id is restored
- Expected assertions:
  - Drift disclosure (provider/policy/columns) is announced before switching
  - Fallback behavior (`preserve`, `subset`, `refuse`, `rebind`, `recreate`) is announced
  - Anchor row id survives switching back to the previous saved view
- Virtualization invariants:
  - Loaded buffer does not collapse during switch
  - Scope counter axes refresh without dropping `visible` or `loaded`
- Accessibility narration: `Saved view changed to Errors -- last 24 h. Captured cursor stale; rebind required before reuse.`

## Support export invariants

- Support export id: `shell:collection_truth_corpus:support_export:default`
- Sourced packet: `shell:collection_truth_corpus:packet:default`
- No sensitive payload: true
- Redaction rules:
  - row payload literals are never exported
  - saved-view secret-bearing values are never exported
  - provider cursors are never exported
  - selected row ids are stable redaction-safe slugs only

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- validate
cargo test -p aureline-shell --test collection_truth_corpus_fixtures
```
