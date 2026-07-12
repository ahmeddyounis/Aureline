# M5 decision / feedback component matrix contract

This document is the human-readable companion to the frozen **M5 badge-chip-pill, popover, dialog-sheet,
banner-inline-notice, toast, empty-state, loading-state, and consequence-block component matrix**.

The authoritative source of truth is the Rust validator and seed builder in
`crates/aureline-ui/src/m5_decision_feedback_component_matrix/`. The checked-in support export, matrix
CSV, design report, and narrowed fixtures are minted from that seed builder by the
`dump_m5_decision_feedback_component_matrix` example; the schemas under `schemas/ui/` document the shape
and the JSON Schemas are meta-valid Draft 2020-12.

## What this freezes

Every claimed M5 surface that still ships its own badge / chip / pill, popover, dialog or sheet, banner
or inline notice, toast, empty state, loading state, or consequence block is named once here and bound
to one shared vocabulary, so state meaning, badge plain-language, popover focus-return, dialog rationale
/ scope / explicit actions, banner and inline-notice scoping, toast durability, empty-state purpose /
emptiness / next action, loading-state partial-data preservation, and consequence-block blast-radius and
rollback truth stop drifting across claimed M5 shell, entry, trust, review, repair, and notification
surfaces.

### Governed primitive families

| Primitive family | Canonical schema |
| --- | --- |
| `badge_chip_pill` | `schemas/ui/m5-badge-chip-pill.schema.json` |
| `popover` | `schemas/ui/m5-popover.schema.json` |
| `dialog_sheet` | `schemas/ui/m5-dialog-sheet.schema.json` |
| `banner_inline_notice` | `schemas/ui/m5-banner-inline-notice.schema.json` |
| `toast` | `schemas/ui/m5-toast.schema.json` |
| `empty_state` | `schemas/ui/m5-empty-state.schema.json` |
| `loading_state` | `schemas/ui/m5-loading-state.schema.json` |
| `consequence_block` | `schemas/ui/m5-consequence-block.schema.json` |

## The one controlled state vocabulary

Every consumer binds to one state taxonomy and no feature family invents a parallel word for any of
these — they mean the same thing everywhere these primitives ship, and none of them may be conveyed by
color alone:

`info`, `success`, `warning`, `blocked`, `pending`, `degraded`, `acknowledged`, `dismissed`.

`warning`, `blocked`, and `degraded` are the attention states that must always expand into plain
language and must never be hidden behind generic chrome.

## Family-specific controlled vocabularies

Each family declares only the vocabulary applicable to it:

- **Badge expression** — `text_label`, `icon_with_text`, `count_with_label`, `status_word`,
  `removable_chip`, `color_only_disallowed` (badge / chip / pill).
- **Popover dismissal** — `dismiss_on_outside_click`, `dismiss_on_escape`, `explicit_close_button`,
  `focus_returns_to_trigger`, `non_modal_secondary`, `carries_only_instruction_disallowed` (popover).
- **Dialog action model** — `named_specific_actions`, `primary_and_cancel`, `destructive_confirm_named`,
  `rationale_and_scope_stated`, `dismissible_safe`, `generic_yes_no_disallowed` (dialog / sheet).
- **Notice scope** — `page_scoped`, `section_scoped`, `field_inline`, `global_system`,
  `actionable_with_next_step`, `unscoped_color_only_disallowed` (banner / inline notice).
- **Toast durability** — `transient_acknowledgment`, `mirrored_to_activity_center`, `dismissible_by_user`,
  `auto_dismiss_timed`, `action_retained_elsewhere`, `toast_only_truth_disallowed` (toast).
- **Empty-state purpose** — `explains_purpose`, `explains_current_emptiness`, `offers_next_action`,
  `first_run_guidance`, `filtered_no_results`, `blank_no_explanation_disallowed` (empty state).
- **Loading fidelity** — `skeleton_preserves_layout`, `partial_data_retained`, `inline_progress_scoped`,
  `determinate_progress`, `indeterminate_spinner_scoped`, `full_screen_spinner_disallowed` (loading
  state).
- **Consequence disclosure** — `named_blast_radius`, `rollback_available`, `rollback_unavailable_stated`,
  `help_path_present`, `explicit_named_actions`, `generic_yes_no_disallowed` (consequence block).

## Hard invariants

Every primitive row asserts (all `false`), one per B135 guardrail:

1. `relies_on_color_alone_for_meaning`
2. `lets_popover_carry_only_critical_instruction`
3. `uses_generic_yes_no_in_high_risk_dialog`
4. `represents_durable_work_as_toast_only`
5. `blanks_useful_pane_during_loading`
6. `uses_full_screen_spinner_when_partial_capable`

## Non-visual / CLI / export requirements

Every primitive declares a non-visual accessibility route set (keyboard-focusable,
screen-reader-announced, high-zoom-reflow, reduced-motion-safe, CLI-exportable, support-packet-present)
so none of these primitives becomes a renderer-only affordance, and every primitive must be present in
the support / export packet. Every primitive also declares the rationale or recovery path it links back
to rather than inventing surface-local folklore.

## Acceptance-criteria mapping

- **Shared matrix** — design, help, QA, security, and release owners share this one matrix for the B135
  decision / feedback primitive family; it is referenced by docs, help, and release evidence and names
  its first consumers (shell, entry, trust, review, repair, notification) instead of remaining a
  design-only placeholder.
- **No bypass** — no claimed M5 lane introducing a new badge / dialog / banner / toast / empty-loading
  pattern can bypass this shared contract without an explicit waiver or a narrower lifecycle label (Beta
  / Preview / Held), and later rows cannot invent parallel feedback vocabulary.
- **One canonical proof set** — release / help / support packets point at one canonical proof set
  (`artifacts/release/m5-decision-feedback-proof/`) for reusable decision and feedback primitives.
