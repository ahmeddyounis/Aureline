# M5 editor-inline component matrix contract

This document is the human-readable companion to the frozen **M5 editor-tab, gutter,
diagnostic-decoration, code-action-chip, diff-view, review-thread, AI-message-card, and
evidence-timeline component matrix**.

The authoritative source of truth is the Rust validator and seed builder in
`crates/aureline-editor/src/m5_editor_inline_component_matrix/`. The checked-in support export, matrix
CSV, design report, and narrowed fixtures are minted from that seed builder by the
`dump_m5_editor_inline_component_matrix` example; the schemas under `schemas/ui/` document the shape
and the JSON Schemas are meta-valid Draft 2020-12.

## What this freezes

Every claimed M5 surface that still ships its own editor tab, gutter marker, diagnostic decoration,
code-action chip, diff view, review thread, AI message card, or evidence timeline is named once here
and bound to one shared vocabulary, so item state, marker layering, problem severity and freshness,
fix posture, comment-anchor durability, AI source context / confidence / actions, and evidence
readability truth stop drifting across claimed M5 editor, diff/merge, review, notebook, AI,
diagnostics, and support surfaces.

### Governed component families

| Component family | Canonical schema |
| --- | --- |
| `editor_tab` | `schemas/ui/m5-editor-tab.schema.json` |
| `gutter` | `schemas/ui/m5-gutter-marker.schema.json` |
| `diagnostic_decoration` | `schemas/ui/m5-diagnostic-decoration.schema.json` |
| `code_action_chip` | `schemas/ui/m5-code-action-chip.schema.json` |
| `diff_view` | `schemas/ui/m5-diff-view.schema.json` |
| `review_thread` | `schemas/ui/m5-review-thread.schema.json` |
| `ai_message_card` | `schemas/ui/m5-ai-message-card.schema.json` |
| `evidence_timeline` | `schemas/ui/m5-evidence-timeline.schema.json` |

## The one controlled disposition vocabulary

Every consumer binds to one inline-disposition vocabulary and no surface invents a parallel word for
any of these:

`modified`, `preview`, `pinned`, `read_only`, `shared`, `generated`, `remote`, `exact_fix`,
`inferred_fix`, `outdated`, `resolved`, `re_anchored`, `blocked_by_policy`, `streaming`,
`review_required`, `applied`, `reverted`, `failed`, `export_safe_evidence`.

## Family-specific controlled vocabularies

Each family declares only the vocabularies applicable to it:

- **Editor-tab state** — `active_current`, `background_open`, `preview_unpinned`, `modified_unsaved`,
  `read_only_locked`, `context_unresolved` (editor tab).
- **Gutter-marker kind** — `breakpoint`, `change_added`, `change_modified`, `change_removed`,
  `fold_region`, `marker_unresolved` (gutter).
- **Diagnostic severity** — `error`, `warning`, `info`, `hint`, `stale_diagnostic`, `severity_unknown`
  (gutter, diagnostic decoration).
- **Fix posture** — `exact_fix`, `inferred_fix`, `heuristic_suggestion`, `multiple_candidates`,
  `not_applicable`, `posture_unknown` (code-action chip).
- **Diff change kind** — `added`, `removed`, `modified`, `moved`, `conflicted`, `unchanged_context`
  (diff view).
- **Anchor durability** — `anchored_exact`, `re_anchored`, `drifted_approximate`, `outdated_anchor`,
  `orphaned_anchor`, `anchor_unresolved` (diagnostic decoration, review thread).
- **AI confidence** — `grounded_high`, `grounded_medium`, `low_confidence`, `unverified`,
  `streaming_partial`, `confidence_unknown` (AI message card).
- **Evidence disclosure** — `expanded_full`, `collapsed_summary`, `partially_loaded`,
  `redacted_export_safe`, `empty_no_evidence`, `disclosure_unknown` (AI message card, evidence
  timeline).

## Hard invariants

Every component row asserts (all `false`):

1. `encodes_tab_marker_or_diagnostic_state_by_color_alone`
2. `lets_comment_anchor_or_evidence_pointer_silently_drift`
3. `blurs_outdated_and_resolved_review_state`
4. `presents_inferred_fix_as_exact`
5. `hides_evidence_timeline_in_opaque_log`

## Non-visual / CLI / export requirements

Every component declares a non-visual accessibility route set (keyboard-focusable,
screen-reader-announced, high-zoom-reflow, reduced-motion-safe, CLI-exportable,
support-packet-present) so none of these components becomes a renderer-only affordance, and every
component must be present in the support / export packet.

## Acceptance-criteria mapping

- **Shared matrix** — design, schema, QA, security, and release owners share this one matrix for the
  B133 inline component family.
- **One canonical contract** — every claimed M5 consumer points at one canonical per-component schema
  (or the combined matrix schema) instead of rewording inline state locally.
- **Agreed baseline** — future implementation rows inherit this field/state baseline with no open
  ambiguity about marker, anchor, confidence, or evidence-lineage labeling.
