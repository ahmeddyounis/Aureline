# Focus-return and batch-scope verification seed

This packet freezes one shared verification story for dense
collections: focus return, count truth, range-anchor behavior,
selection-bar disclosures, and keyboard or screen-reader access to
batch-review flows. It bridges the interaction-safety contract, the
live-review contract, the accessibility dense-review lane, and the
window/topology verification seed so later tables, lists, trees, review
queues, and provider-backed result surfaces reuse one reviewer-facing
packet instead of inventing per-surface selection truth.

If this document, the
[`selection_and_virtualization_manifest.yaml`](../../fixtures/ux/selection_and_virtualization_manifest.yaml)
corpus, the focus-return worked examples, and the range-selection AT
cases disagree, the manifest and the source contracts win for tooling
and this packet must update in the same change.

Companion artifacts:

- [`/fixtures/ux/selection_and_virtualization_manifest.yaml`](../../fixtures/ux/selection_and_virtualization_manifest.yaml)
  — machine-readable case roster covering virtualized tables and lists,
  filtered collections, hidden-selected disclosure, resize/reflow,
  detached panels, multi-window transfers, and range-select/clear/extend
  flows.
- [`/artifacts/ux/focus_return_examples/`](../../artifacts/ux/focus_return_examples/)
  — reviewer-facing focus-return examples showing exact return, nearest
  safe ancestor return, current batch/detail-owner return, and
  placeholder-announced return.
- [`/artifacts/accessibility/range_selection_at_cases/`](../../artifacts/accessibility/range_selection_at_cases/)
  — accessibility cases for current-row selection, range extension,
  hidden-selected inspection, and keyboard-only batch-review entry.
- [`/docs/ux/shell_interaction_safety_contract.md`](../ux/shell_interaction_safety_contract.md)
  — canonical `batch_scope_record` and `focus_return_record` fields the
  packet reuses.
- [`/docs/ux/live_update_review_contract.md`](../ux/live_update_review_contract.md)
  — canonical `loaded_count`, `visible_count`, `total_count`, anchor,
  and batch-membership honesty rules the packet reuses.
- [`/docs/accessibility/review_charter.md`](../accessibility/review_charter.md),
  [`/artifacts/accessibility/shell_conformance_checklist.yaml`](../../artifacts/accessibility/shell_conformance_checklist.yaml),
  [`/artifacts/accessibility/accessibility_tree_coverage_rows.yaml`](../../artifacts/accessibility/accessibility_tree_coverage_rows.yaml),
  and
  [`/fixtures/accessibility/task_corpus_manifest.yaml`](../../fixtures/accessibility/task_corpus_manifest.yaml)
  — launch-critical dense-collection accessibility lane, checklist ids,
  tree rows, and stable task ids.
- [`/fixtures/ux/live_review_examples/result_grid_frozen_buffered_reorder.json`](../../fixtures/ux/live_review_examples/result_grid_frozen_buffered_reorder.json)
  — current repo anchor for loaded/visible/total count truth and drifting
  batch-membership honesty on a dense result grid.
- [`/fixtures/ux/interaction_safety_cases/destructive_bulk_rename_apply.yaml`](../../fixtures/ux/interaction_safety_cases/destructive_bulk_rename_apply.yaml)
  — current repo anchor for query-backed batch scope, blocked or hidden
  member summaries, responsive fallback, and a committed
  `focus_return_record`.
- [`/docs/qa/multi_window_verification.md`](../qa/multi_window_verification.md)
  and
  [`/fixtures/platform/window_display_cases/`](../../fixtures/platform/window_display_cases/)
  — window-local focus history, off-screen owner recovery, mixed-DPI,
  and multi-window continuity rules the packet composes over rather than
  redefining.

Normative sources projected here:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — `ARCH-UX-005`, `ARCH-SLICE-007`, `A11Y-CORE-002`,
  `A11Y-SR-003`, and the desktop window/session fidelity rows.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — Section 8.22 plus the collection, batch-review, and accessibility
  parity rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — Section 9.1, Section 9.12, Appendix EL, Appendix EO, and the dense
  collection or batch-scope evidence crosswalk rows.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`
  — the selection-bar, batch-review, and filter/scope vocabulary matrix
  plus the dense-surface accessibility rules.
- `.t2/docs/Aureline_Milestones_Document.md`
  — interaction-integrity, focus/batch-scope truth, and release-blocking
  accessibility posture for virtualization, resize, and multi-window
  conditions.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.focus_and_batch_scope.collection_seed
evidence_id: evidence.ux.focus_and_batch_scope.collection_seed
title: Focus-return and batch-scope verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - ARCH-UX-005
    - ARCH-SLICE-007
    - A11Y-CORE-002
    - A11Y-SR-003
    - FR-SEARCH-001
    - TOOL-EVT-001
  claim_row_refs: []
  covered_lanes:
    - release_evidence
    - accessibility_input_review
    - support_export
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: commit:working_tree
  trigger_revision: focus_and_batch_scope_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen dense-collection interaction vocabulary,
    accessibility task lane, live-review count truth, and window-local
    focus-return contract. No product-wide green claim is made yet.
artifact_links:
  supporting_evidence_ids:
    - evidence.ux.selection_and_virtualization_manifest
    - evidence.ux.focus_return_examples
    - evidence.accessibility.range_selection_at_cases
    - evidence.accessibility.dense_collection_seed
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/ux/selection_and_virtualization_manifest.yaml
    - fixtures/ux/live_review_examples/result_grid_frozen_buffered_reorder.json
    - fixtures/ux/interaction_safety_cases/destructive_bulk_rename_apply.yaml
    - fixtures/accessibility/ime_and_text_cases/virtualized_selection_scope.yaml
    - fixtures/accessibility/ime_and_text_cases/range_selection_anchor_stability.yaml
    - fixtures/accessibility/ime_and_text_cases/multi_window_mixed_dpi_composition.yaml
    - fixtures/platform/window_display_cases/
  archetype_refs: []
  source_anchor_refs:
    - docs/ux/shell_interaction_safety_contract.md
    - docs/ux/live_update_review_contract.md
    - docs/accessibility/review_charter.md
    - artifacts/accessibility/shell_conformance_checklist.yaml
    - artifacts/accessibility/accessibility_tree_coverage_rows.yaml
    - fixtures/accessibility/task_corpus_manifest.yaml
    - docs/qa/multi_window_verification.md
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one closed count vocabulary for `selected`, `visible`, `loaded`,
  `matching`, `hidden selected`, `blocked`, `skipped`, and
  `not loaded`;
- one cross-contract mapping from dense-collection review to
  `batch_scope_record`, `focus_return_record`, and live-review count
  truth;
- one explicit pass/degraded/fail rubric for a current dense-collection
  surface family in this repository; and
- one set of focus-return and range-selection examples that future
  accessibility packets, support exports, and collection UX reviews can
  cite without per-surface synonyms.

It does not claim that every product collection already emits all of
these fields. It claims only that the reviewable evidence shape now
exists and is grounded in the current repo's interaction-safety,
live-review, accessibility, and windowing artifacts.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:collection.focus_selection_activation` | `ARCH-UX-005`, `A11Y-CORE-002` | `seed_only` | `internal` | `evidence.ux.selection_and_virtualization_manifest` | Freezes focus-versus-selection-versus-activation wording for dense collections. |
| `packet_row:collection.count_truth` | `ARCH-SLICE-007`, `FR-SEARCH-001`, `TOOL-EVT-001` | `seed_only` | `internal` | `evidence.ux.selection_and_virtualization_manifest` | Keeps selected, visible, loaded, matching, hidden-selected, blocked, skipped, and not-loaded counts orthogonal. |
| `packet_row:collection.focus_return` | `ARCH-UX-005`, `A11Y-CORE-002` | `seed_only` | `internal` | `evidence.ux.focus_return_examples` | Makes exact return, safe-ancestor return, batch-owner return, and placeholder-announced return reviewable across dense surfaces. |
| `packet_row:collection.range_selection_accessibility` | `A11Y-CORE-002`, `A11Y-SR-003` | `seed_only` | `internal` | `evidence.accessibility.range_selection_at_cases` | Current-row select, range extension, hidden-selected inspection, and keyboard-only batch-review entry now have stable case ids. |
| `packet_row:collection.resize_detach_multiwindow` | `ARCH-UX-005`, `ARCH-SLICE-007`, `A11Y-CORE-002` | `seed_only` | `internal` | `evidence.ux.selection_and_virtualization_manifest`, `evidence.ux.focus_return_examples` | Focus-locality and scope honesty stay explicit under resize/reflow, detached panels, and multi-window transfers. |
| `packet_row:collection.surface_evaluation_rubric` | `ARCH-UX-005`, `A11Y-CORE-002`, `A11Y-SR-003` | `seed_only` | `internal` | `evidence.ux.selection_and_virtualization_manifest`, `evidence.accessibility.range_selection_at_cases` | At least one dense surface family in the current repo can now be scored as passed, degraded, or failed. |

## What this seed freezes

- One reviewer-facing distinction between focus, current item,
  selection, anchor, and activation so dense collections stop blurring
  "where input goes" with "what will be acted on."
- One count taxonomy that keeps `hidden selected`, `blocked`, `skipped`,
  and `not loaded` separate instead of burying them inside one
  ambiguous number.
- One required selection-bar field set: selected count, scope label,
  hidden-selected count, clear-selection action, blocked/skipped count
  when known, and the explicit `Select all matching` escalation when
  broader scope exists.
- One required batch-review sheet field set: included, excluded,
  blocked, skipped, and query-basis disclosure plus recovery or undo
  posture.
- One focus-return rubric that stays window-local and survives
  virtualization, filtering, compact-shell fallback, detached panels,
  mixed-DPI reflow, and multi-window transfers.

## Focus, selection, and activation remain separate

Dense collections reuse the UI/UX Spec state model rather than minting a
new one:

| State | Reviewer question | Required cue |
|---|---|---|
| `focus` | Where does keyboard input go now? | one visible focus owner per window plus an AT announcement |
| `current_item` | Which row/object drives detail or preview? | row or pane emphasis distinct from multi-selection styling |
| `selection` | Which objects will a batch action target? | selected count plus row state that survives virtualization by identity |
| `anchor` | Where will range extension start from? | visible or announced anchor marker |
| `activation` | What will Enter or the primary action do next? | pressed/progress cue separate from durable selection |

Rule: a row becoming selected may not overclaim focus, and opening a
detail or batch-review surface may not silently clear a broader
selection unless the narrowing is explicit first.

## Packet field set

Use these packet-level fields whenever Aureline explains dense
collection scope, focus return, or range-selection posture:

| Packet field | Meaning | Primary source |
|---|---|---|
| `surface_group` | Stable dense-collection family id under review | accessibility task/checklist rows |
| `selection_scope_class` | Current-item, visible, loaded, all-matching, or explicit custom-set scope | UI/UX Spec Section 9.12 |
| `selected_count` | Explicitly selected objects | selection bar or batch review |
| `visible_count` | Currently rendered or reviewed rows | live-review count truth |
| `loaded_count` | Currently fetched rows in client/session | live-review count truth |
| `matching_count` | All results matching the active query/filter | selection bar or query basis |
| `hidden_selected_count` | Selected objects hidden by filter, sort, collapse, or viewport | selection bar or batch review |
| `blocked_count` | Objects known pre-commit to be ineligible because of policy, ownership, or protected path | `batch_scope_record.member_summary` |
| `skipped_count` | Objects intentionally excluded from commit or aftermath because they were already compliant, unchanged, or otherwise non-actionable | batch result summary or aftermath view |
| `not_loaded_selected_count` | Selected objects known by identity but not currently materialized into the client | packet projection over query-backed selection |
| `focus_return_state` | How focus returned when a transient surface closed | `focus_return_record` |
| `range_anchor_state` | Whether the anchor remained stable, reset on clear, or went stale on dataset identity change | AT case or range-selection review |
| `batch_review_entry_route` | How batch review was opened | keyboard, command, pointer, or assistive path |
| `detachment_context` | Same window, compact-shell fallback, detached panel, or other window | windowing or responsive-fallback evidence |
| `basis_snapshot_freshness` | Freshness of the query or selection basis | interaction-safety or live-review source |
| `verdict` | `passed`, `degraded`, or `failed` reviewer outcome for the evaluated surface row | this packet's rubric |

Rules:

1. `selected_count` never substitutes for `visible_count`,
   `loaded_count`, or `matching_count`.
2. `hidden_selected_count`, `blocked_count`, `skipped_count`, and
   `not_loaded_selected_count` are orthogonal. If more than one is
   non-zero, they MUST remain separately addressable.
3. `blocked_count` is a pre-commit ineligibility story.
   `skipped_count` is an intentional non-application story. They may not
   collapse to one "unavailable" bucket.
4. `focus_return_state` and `detachment_context` are coupled: when
   exact return is no longer possible, the packet must name why and what
   took focus instead.
5. `range_anchor_state` may reset only when selection is cleared or the
   dataset identity changes materially. Sort, filter, and virtualization
   churn alone do not move the anchor.

## Count truth and selection-bar fields

These terms are the controlled vocabulary for dense collection review:

| Term | Meaning | When it must remain visible |
|---|---|---|
| `selected` | current explicit user selection | every selection bar and batch review |
| `visible` | currently rendered subset | virtualized or paged surfaces |
| `loaded` | currently fetched into the client | provider-backed or incremental surfaces |
| `matching` | all results matching the active query/filter | any query-backed `Select all matching` path |
| `hidden selected` | selected identities not currently visible | after filter/sort/collapse/viewport change |
| `blocked` | pre-commit ineligible members | review sheet and preflight summary |
| `skipped` | intentionally not applied or not rerun members | batch aftermath or provider summary |
| `not loaded` | selected identities known by scope but not yet materialized locally | query-backed broad selections and sparse windows |

Required selection-bar fields for serious dense collections:

- selected count
- scope label (`current item`, `visible`, `loaded`, `all matching`, or
  `custom set`)
- hidden-selected count when non-zero
- blocked/skipped count when already known
- clear-selection action
- explicit `Select all matching` second step when broader scope exists
- query-basis freshness when the selection rides a query snapshot

## Scope escalation and range rules

- `Select all` begins with the visible page or loaded subset and only
  escalates to `Select all matching` through a second deliberate step.
- Virtualized tables and lists must preserve selection by stable object
  identity; offscreen rows do not need to remain mounted once identity
  membership is established.
- Filtering or sorting after selection preserves selected identities
  while making hidden-selected count inspectable.
- Range extend uses the same anchor until the user clears selection or
  the dataset identity changes materially. Clear-and-reselect
  establishes a new anchor; refresh or scroll alone does not.
- Trees respect visible traversal order. Collapsed descendants are not
  silently included in a range unless the user chose a subtree action.

## Dense collection evaluation rubric

This packet is immediately usable against the repository's existing
`dense_collection_batch_review` surface family.

| Surface under review | Passed | Degraded | Failed |
|---|---|---|---|
| `dense_collection_batch_review` projected through the current result-grid, interaction-safety, accessibility, and windowing fixtures | selected, visible, loaded, matching, hidden-selected, blocked, skipped, and not-loaded truth are explicit where applicable; keyboard and screen-reader routes can select the current row, extend a range, inspect hidden-selected count, open batch review, and return focus lawfully | the invoker disappeared or moved across panels/windows so focus returns to the nearest safe ancestor, current batch/detail owner, or a placeholder-announced owner; totals may be approximate or not-loaded, but the narrowing remains explicit | hidden or filtered members are silently included; blocked and skipped collapse into one count; batch review requires pointer-only controls; focus is lost silently or warps to another window/background owner |

The current repo anchors for that evaluation are:

| Existing artifact | What it already proves | What this packet adds |
|---|---|---|
| [`result_grid_frozen_buffered_reorder.json`](../../fixtures/ux/live_review_examples/result_grid_frozen_buffered_reorder.json) | `loaded_count`, `visible_count`, `total_count`, and drifting batch-membership honesty on a dense result grid | adds hidden-selected, blocked, skipped, not-loaded, and focus-return review requirements |
| [`destructive_bulk_rename_apply.yaml`](../../fixtures/ux/interaction_safety_cases/destructive_bulk_rename_apply.yaml) | query-backed batch scope, blocked/hidden/query-derived summary, responsive fallback, and a committed `focus_return_record` | adds dense-collection-specific selection bar, `Select all matching`, and range-anchor review language |
| [`shell_conformance.dense_collection_batch_review`](../../artifacts/accessibility/shell_conformance_checklist.yaml) and [`corpus.accessibility.review.dense_collection_batch_review`](../../fixtures/accessibility/task_corpus_manifest.yaml) | launch-critical keyboard, narration, and tree-coverage lane for dense review | adds concrete AT case ids and pass/degraded/fail scoring for current-row selection, range extension, hidden-selected inspection, and keyboard-only review entry |
| [`multi_window_verification.md`](../qa/multi_window_verification.md) and [`window_display_cases/`](../../fixtures/platform/window_display_cases/) | window-local focus history, owner-dialog recovery, mixed-DPI reflow, and off-screen recovery | adds detached-panel and multi-window transfer focus-return truth for dense collection review |

## Focus-return outcomes

The packet reuses the closed `focus_return_state` vocabulary from the
interaction-safety contract. For dense collections, the states evaluate
like this:

| Focus-return state | Collection meaning | Typical verdict |
|---|---|---|
| `returned_exact` | focus closed back onto the same logical row or invoking control | `passed` |
| `returned_nearest_safe_ancestor` | original row no longer exists or is hidden; focus moved to the closest still-valid collection owner and the announcement says why | `degraded` |
| `returned_current_batch_or_detail_owner` | a detached sheet or detail pane now owns the batch and focus returned there intentionally | `degraded` unless the detail owner was the committed origin |
| `returned_placeholder_announced` | original window/panel/row is unavailable, but a visible placeholder now owns recovery and explains why | `degraded` |
| `focus_loss_denied` | shell fallback removed the original return target; persistent re-entry affordance is required immediately | `failed` unless the user can continue safely from the re-entry affordance without losing scope truth |

Silent focus loss is never acceptable.

## Accessibility and batch-review expectations

Dense review remains non-conforming unless keyboard and screen-reader
paths can:

- select the current row without activating it;
- extend a range from a stable anchor across virtualized churn;
- clear selection and establish a new anchor explicitly;
- inspect hidden-selected count and query-backed scope;
- open batch review without pointer-only checkboxes; and
- close review back to the originating row, current batch/detail owner,
  or a visible placeholder with an explanation.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Artifact ref |
|---|---|---|---|
| `evidence.ux.selection_and_virtualization_manifest` | `verification_corpus` | canonical case roster for virtualization, filtering, hidden-selected, resize/reflow, detached-panel, multi-window, and range flows | [`selection_and_virtualization_manifest.yaml`](../../fixtures/ux/selection_and_virtualization_manifest.yaml) |
| `evidence.ux.focus_return_examples` | `verification_corpus` | reviewer-facing lawful focus-return outcomes for dense collections | [`focus_return_examples/`](../../artifacts/ux/focus_return_examples/) |
| `evidence.accessibility.range_selection_at_cases` | `verification_corpus` | AT-specific review cases for current-row select, range extend, hidden-selected inspection, and keyboard-only review entry | [`range_selection_at_cases/`](../../artifacts/accessibility/range_selection_at_cases/) |
| `evidence.accessibility.dense_collection_seed` | `acceptance_pack_seed` | current dense-review accessibility checklist, tree rows, and task ids in the repo | [`shell_conformance_checklist.yaml`](../../artifacts/accessibility/shell_conformance_checklist.yaml), [`accessibility_tree_coverage_rows.yaml`](../../artifacts/accessibility/accessibility_tree_coverage_rows.yaml), [`task_corpus_manifest.yaml`](../../fixtures/accessibility/task_corpus_manifest.yaml) |

## Corpus coverage

The manifest is the canonical row set. The table below is the
reviewer-facing summary.

| Case id | Main coverage | Primary refs |
|---|---|---|
| `selection_scope.virtualized_table.select_visible_then_matching` | virtualized table, visible-to-matching escalation, exact focus return | [`virtualized_selection_scope.yaml`](../../fixtures/accessibility/ime_and_text_cases/virtualized_selection_scope.yaml), [`batch_review_sheet_return_exact.yaml`](../../artifacts/ux/focus_return_examples/batch_review_sheet_return_exact.yaml) |
| `selection_scope.virtualized_list.hidden_selected_after_filter` | filtered collection, hidden-selected disclosure, not-loaded truth | [`hidden_selected_count_inspection.yaml`](../../artifacts/accessibility/range_selection_at_cases/hidden_selected_count_inspection.yaml), [`filtered_row_return_nearest_safe_ancestor.yaml`](../../artifacts/ux/focus_return_examples/filtered_row_return_nearest_safe_ancestor.yaml) |
| `selection_scope.filtered_collection.blocked_vs_skipped_breakout` | separate blocked and skipped outcomes | [`destructive_bulk_rename_apply.yaml`](../../fixtures/ux/interaction_safety_cases/destructive_bulk_rename_apply.yaml) |
| `selection_scope.resize_reflow.selection_bar_survives_compact_shell` | resize/reflow, compact shell, preserved summary fields | [`destructive_bulk_rename_apply.yaml`](../../fixtures/ux/interaction_safety_cases/destructive_bulk_rename_apply.yaml), [`mixed_dpi_cross_monitor_reflow.json`](../../fixtures/platform/window_display_cases/mixed_dpi_cross_monitor_reflow.json) |
| `selection_scope.detached_panel.review_focus_owner` | detached panel ownership and lawful focus return | [`detached_panel_return_current_batch_owner.yaml`](../../artifacts/ux/focus_return_examples/detached_panel_return_current_batch_owner.yaml) |
| `selection_scope.multi_window_transfer.focus_locality` | multi-window transfers, owner recentering, placeholder-announced recovery | [`multi_window_transfer_placeholder_announced.yaml`](../../artifacts/ux/focus_return_examples/multi_window_transfer_placeholder_announced.yaml), [`offscreen_dialog_owner_recenter.json`](../../fixtures/platform/window_display_cases/offscreen_dialog_owner_recenter.json) |
| `selection_scope.range_select_clear_extend.identity_anchor` | range select, clear, extend, anchor reset rules | [`range_selection_anchor_stability.yaml`](../../fixtures/accessibility/ime_and_text_cases/range_selection_anchor_stability.yaml), [`range_extension_across_virtualized_rows.yaml`](../../artifacts/accessibility/range_selection_at_cases/range_extension_across_virtualized_rows.yaml) |
