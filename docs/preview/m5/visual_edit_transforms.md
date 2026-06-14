# M5 visual-edit transforms

This document is the contract for the M5 source-first visual-edit transforms. It
binds the **four visual-edit outcomes** onto a single shared packet so the
round-trip honesty, preview diff, rollback class, and fallback behavior of every
claimed visual edit stop hiding inside provider-specific designer chrome.

Where the
[source-first preview / browser-runtime inspection matrix](freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md)
freezes the *qualification* of each claimed preview/runtime surface, the
[preview-session descriptors](preview_session_descriptors.md) materialize the
*per-session* state, the
[inspect-to-source tree mapping](inspect_to_source_tree_mapping.md) materializes
the *per-node* source-mapping truth, and the
[browser-runtime inspectors](browser_runtime_inspectors.md) materialize the
*per-inspector* runtime truth, this packet materializes the *per-edit* truth
behind every claimed visual-edit flow.

Source remains canonical; the transform packet is derivative — never a second
writable truth model. Every claimed visual edit names whether it writes back to
source, what real source diff it will produce, how it can be rolled back, and —
when the construct is ambiguous or lossy — that it degrades to a code-first
suggestion or inspect-only mode instead of a silent lossy rewrite.

## Source of truth

- Packet type: `VisualEditTransformPacket`
  (`crates/aureline-preview/src/visual_edit_transforms/`).
- Boundary schema:
  `schemas/preview/visual_edit_transforms.schema.json`.
- Checked support export:
  `artifacts/preview/m5/visual_edit_transforms/support_export.json`.
- Markdown summary:
  `artifacts/preview/m5/visual_edit_transforms.md`.
- Protected fixtures:
  `fixtures/preview/m5/visual_edit_transforms/`.
- Conformance dump: `cargo run -p aureline-preview --example dump_m5_visual_edit_transforms [support|summary]`.

## Outcome vocabulary

Every claimed visual edit names its disposition through one vocabulary:

| Outcome | Meaning |
| --- | --- |
| `exact_round_trip_apply` | A supported exact-round-trip edit: emits a transform manifest, a real source diff, and a rollback class, then applies the exact canonical-source diff |
| `approximate_round_trip_apply` | An approximate-round-trip edit: maps approximately to source but still previews the real diff and a rollback class before commit |
| `code_first_fallback` | An ambiguous or lossy construct degrades to a code-first source suggestion; no visual write |
| `inspect_only` | An unsupported construct degrades to inspect-only mode; no write, no suggestion, but the selection context is preserved |

An `exact_round_trip_apply` and `approximate_round_trip_apply` are *apply*
outcomes; a `code_first_fallback` and `inspect_only` are *fallback* outcomes. An
`inspect_only` row is the canonical **preview-only** row.

## Every apply previews the real source diff first

A row whose outcome is an apply MUST carry, before any source byte changes:

- a `transform_manifest` whose `applies_real_source_diff` is true and whose
  `pipeline_ref` names the shared preview/apply/revert pipeline used by other
  wide-scope mutations — never a private write path;
- a real-source `preview_diff` (`real_source_unified_diff` or
  `real_source_multi_file_diff`);
- a revertible `rollback_class` (`checkpoint_revertible`, `snapshot_revertible`,
  or `inverse_transform_revertible`); and
- a `review_posture` (confirmation or review).

The round-trip capability must agree with the outcome: an exact apply claims
`exact_source_round_trip`, an approximate apply claims
`approximate_source_round_trip`.

## No silent lossy rewrite

A construct that is ambiguous or lossy — `dynamic_bound_expression`,
`conditional_or_loop_generated`, or `external_or_generated_artifact` — can never
be the target of an apply. It MUST degrade to a `code_first_fallback` or
`inspect_only` outcome that:

- carries no `transform_manifest`;
- carries a `no_mutation_no_rollback` rollback class and no `review_posture`;
- carries an `unsupported_card` whose `reason` is quoted verbatim, whose
  `card_label` is precise (not a generic "unsupported" / "blocked"), and whose
  `preserves_selection_context` is true so the user keeps their place; and
- shows a `code_first_suggestion_diff` (code-first) or `no_diff_inspect_only`
  (inspect-only).

An `inspect_only` row never auto-upgrades into a write-capable designer flow.

## Protected paths and ownership

The `protected_path_posture` preserves protected-file awareness on every
visual-edit path. A `protected_blocked` target can never carry an apply; it must
degrade like any other unsupported construct (with a `protected_path_blocked`
card reason). A `protected_review_required` or `protected_owner_approval_required`
target may apply only under the recorded review posture.

## Preview-only stays distinguishable from round-trip

Each row names its `framework_pack_family`. The packet proves that release and
support surfaces can tell a preview-only row from an exact-round-trip row on the
very same framework pack family: at least one family carries both an
`exact_round_trip_apply` row and an `inspect_only` row.

## Validation

`VisualEditTransformPacket::validate` enforces, among others:

- every outcome is represented; at least one complete round-trip apply and one
  complete fallback exist; some family carries both a round-trip and a
  preview-only row;
- per row: outcome/round-trip/preview-diff/rollback agreement, apply-only
  manifests, fallback-only cards, protected-path gating, review-posture presence,
  and non-empty evidence refs;
- guardrails and consumer-projection invariants hold; and
- the export carries no raw boundary material.

The checked support export is byte-aligned with the in-crate builder via the
conformance dump, and the protected fixture mirrors it.
