# Collection-truth beta contract

This document is the M3 beta product/runtime contract for dense
collection surfaces — search results, problems, review queues, logs,
admin boards, package inventories, work-item boards, and provider-
backed tables. It freezes the shell-side record kinds that every
collection-bearing surface MUST emit, complementing the alpha
collection-view grammar in
[`docs/ux/collection_view_contract.md`](../collection_view_contract.md)
and the boundary schemas under `schemas/collections/`.

The contract is normative. Where this document disagrees with
[`docs/ux/collection_view_contract.md`](../collection_view_contract.md),
[`docs/ux/selection_and_scope_contract.md`](../selection_and_scope_contract.md),
[`docs/ux/shell_interaction_safety_contract.md`](../shell_interaction_safety_contract.md),
or
[`docs/ux/live_update_review_contract.md`](../live_update_review_contract.md),
those documents win and this document plus the schemas are updated in
the same change.

## Companion artifacts

- [`crates/aureline-shell/src/collection_truth/`](../../../crates/aureline-shell/src/collection_truth/)
  — shell-owned Rust types, seeded packet, validation, and the
  `aureline_shell_collection_truth` headless inspector.
- [`schemas/ux/filter_bar_state.schema.json`](../../../schemas/ux/filter_bar_state.schema.json)
  — boundary schema for the filter-bar state record.
- [`schemas/ux/saved_collection_view.schema.json`](../../../schemas/ux/saved_collection_view.schema.json)
  — boundary schema for the saved collection-view record.
- [`schemas/ux/collection_scope_counter.schema.json`](../../../schemas/ux/collection_scope_counter.schema.json)
  — boundary schema for the scope-counter record.
- [`schemas/ux/batch_review_sheet.schema.json`](../../../schemas/ux/batch_review_sheet.schema.json)
  — boundary schema for the batch-review sheet record.
- [`fixtures/ux/m3/collection_truth/`](../../../fixtures/ux/m3/collection_truth/)
  — deterministic packet plus per-surface worked records.

## Why freeze this now

Without one frozen beta contract every dense collection surface is
free to invent its own per-feature notion of *which counts are
honest*, *what a saved view promises*, *which chips disclose hidden
narrowing*, and *what `Select all` will actually touch*. Each
divergence widens a different axis silently:

1. Filter bars hide policy / workset / provider / client narrowing
   behind a "filtered" badge — users see a list they assume is the
   whole truth.
2. Saved views silently drop unsupported filter operators on restore
   — the same view returns different rows than the user captured.
3. Counters collapse `visible`, `loaded`, and `matching` into one
   number — exporters and operators commit to a wider population
   than the surface represented.
4. `Select all` escalates to all matching against ambiguous scope —
   destructive actions touch rows the user never reviewed.
5. Batch sheets paint blocked rows as skipped or already-compliant
   — operators rerun and rerun until the blocked rows mutate too.

The freeze matters now, ahead of any production collection UI
landing, so the M3 launch-critical wedges (search results, problems,
review queues, logs, admin boards) inherit one product-wide answer to
all five risks instead of inventing per-surface synonyms.

## Frozen vocabulary

Every vocabulary in this beta contract is closed. Adding a new value
is additive-minor and bumps the relevant
`*_record.schema_version`; repurposing an existing value is breaking.

### Surface family (six values)

```
search_or_result_grid
review_inbox
log_or_event_collection
package_or_inventory_grid
work_item_board
admin_or_settings_grid
```

Every record (filter bar, saved view, scope counter, batch review)
carries exactly one surface family.

### Narrowing source class (seven values)

```
user
saved_view
policy
workset
client_limit
provider_limit
partial_data
```

`policy`, `workset`, `client_limit`, `provider_limit`, and
`partial_data` are **hidden narrowing** and MUST remain visible,
locked, and explainable. Surfaces that collapse them into the
`user`/`saved_view` family are non-conforming.

### Count-summary class (seven values)

```
exact_local
exact_with_workset_narrowing
exact_with_policy_pinning
approximate_provider_limited
partial_indexing
provider_retention_windowed
unknown
```

### Counter class (six values)

```
visible
loaded
matching
total
partial
provider_owned
```

Each row carries one counter class and one counter status from
`{exact, approximate, provider_limited, partial, unknown}`. A
counter without status is non-conforming.

### Saved-view scope class (five values, spec-aligned)

```
user
workspace
shared
policy_pinned
provider_owned
```

### Saved-view drift state (seven values)

```
bound_current_state_matches_captured
provider_state_drifted_disclosed
column_set_drifted_disclosed
policy_narrowing_changed_disclosed
view_archived_offered_restore
view_unresolvable_offered_recreate
view_unavailable_provider_offline_disclosed
```

### Saved-view fallback behavior (five values)

```
preserve_and_label_degraded
load_portable_subset_with_labels
refuse_until_rebound
provider_rebind_required
offer_recreate_from_current
```

Drift states pair to fallback behaviors per
[`docs/ux/collection_view_contract.md`](../collection_view_contract.md);
silently wiping user state or pretending a drifted view is bound is
non-conforming.

### Batch action consequence class (seven values)

```
routine_non_mutating
local_reversible
destructive_local
remote_mutation
destructive_remote
export_or_share
provider_owned_mutation
```

Every class except `routine_non_mutating` and `local_reversible`
requires a review sheet before continuing.

### Select-all escalation class (three values)

```
visible_or_loaded
all_matching_safe
all_matching_refused
```

`Select all` starts at `visible_or_loaded` on every surface and only
escalates to `all_matching_safe` when the matching count is exact,
approximate, or provider-limited but disclosed. When the matching
scope is unknown or unsafe to express, escalation MUST resolve to
`all_matching_refused` and continue MUST be disabled.

### Recovery-guidance class (six values)

```
reversible_via_undo_stack
compensating_revert_within_window
export_rollback_by_redelivery
regenerate_from_source
evidence_only_no_rerun
no_recovery_available
```

### Blocked-reason class (seven values)

```
policy_narrowed
ownership_missing
protected_path
provider_unsupported
freshness_required
grant_missing
concurrent_edit
```

Blocked items are refused before continue. They MUST remain
distinguishable from `excluded`, `hidden`, and skipped items;
collapsing them denies the sheet.

## Truthfulness posture (normative)

Every rule below is normative.

1. **One filter-bar state record per surface.** A surface MUST emit
   one `shell_collection_filter_bar_state_record` whenever the bar
   is rendered, persisted, exported, or replayed. Hidden-narrowing
   chips MUST remain locked and explained.
2. **One saved-view record per saved view.** A surface MUST emit
   one `shell_saved_collection_view_beta_record` whenever a column
   set, filter, sort, or group-by is saved, restored, or shared.
   Transient row selection, provider cursors, and secret-bearing
   values MUST stay out of the record.
3. **One scope-counter record per counter strip.** A surface MUST
   emit one `shell_collection_scope_counter_beta_record` whenever a
   counter strip is rendered. Counters always carry visible, loaded,
   and matching axes at minimum; collapsing two axes into one
   number is non-conforming.
4. **One batch-review sheet per consequential action.** A surface
   MUST emit one `shell_collection_batch_review_sheet_beta_record`
   before any destructive, remote, export-bearing, or provider-
   owned action. The sheet MUST split included, excluded, blocked,
   and hidden counts and MUST NOT enable continue when the scope is
   ambiguous.
5. **`Select all` starts visible-or-loaded.** Escalation to
   `all_matching_safe` requires honest matching counts; otherwise
   escalation MUST resolve to `all_matching_refused` and continue
   MUST be disabled.
6. **Hidden narrowing follows the row everywhere.** Filter chips,
   accessibility narration, saved-view serialization, support
   export, and batch-review summaries MUST carry the same source
   class. Stripping the class at any boundary is non-conforming.
7. **Drift discloses, fallback fires.** A saved view that no longer
   matches the current state MUST resolve to a drift state other
   than `bound_current_state_matches_captured` and MUST pair to a
   fallback behavior. Restore that silently wipes user state denies.

## Acceptance criteria cross-walk

This contract delivers the spec's four acceptance bullets:

1. **`Select all` starts visible / loaded, escalates explicitly.**
   The `select_all_escalation_class` field plus the `ambiguity_findings`
   gating on `continue_enabled` force escalation through review.
2. **Filter bars and saved-view switchers expose narrowing.**
   `NarrowingSourceClass` distinguishes user, saved view, policy,
   workset, client limit, provider limit, and partial data; the
   `hidden_narrowing_summary` on the filter-bar record carries the
   accessibility-narration text.
3. **Portable saved views migrate explicitly.** `SavedViewScopeClass`,
   `SavedViewDriftState`, and `SavedViewFallbackBehavior` pin the
   restore behavior. Capturing transient selection, provider cursors,
   or secret-bearing values is forbidden at the schema boundary.
4. **Batch actions cannot run on ambiguous scope.** `BatchReviewSheetRecord`
   computes ambiguity findings (refused select-all, zero included
   on a consequential action) and disables `continue_enabled` until
   the surface resolves them; blocked items stay separate from
   excluded, hidden, and skipped items.

## Out of scope at this milestone

- The live collection UI (filter builder, saved-view picker, batch-
  review sheet engine, column-pinning gestures, virtualised list /
  table rendering pipelines).
- A saved-view share UI or org-share sync transport.
- A CLI render layer for filter / saved-view / batch output.
- Broadening into a BI / reporting product, full database tooling,
  or arbitrary user-scripted filter languages.

These lines move only by opening a new decision row, not by editing
this contract.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — dense collection, filter, saved view,
  batch review, and count-truth requirements.
- `.t2/docs/Aureline_Technical_Design_Document.md` — §13.13
  collection-truth lane and ARCH-COLLECT-013.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §§16.64–16.65 — filter
  bars, saved views, result-scope counters, batch-review sheets.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` — launch-
  critical map for filterable tables, logs, review inboxes, admin
  queues.
- `.t2/docs/Aureline_Milestones_Document.md` — launch-bearing
  collection-truth and claim-honesty controls.
- `docs/ux/collection_view_contract.md` — alpha grammar this beta
  contract composes with.
