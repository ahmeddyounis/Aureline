# M5 test-generation-suggestion-card primitive

This document is the contract reference for the reusable M5 **test-generation suggestion card** —
one governed test-intelligence component implemented as a single primitive in the
`aureline-runtime` crate
(`implement_test_generation_suggestion_cards_with_uncovered_path_or_bug_trigger_truth_assumption_summaries_helper_fixture_snapshot_separation_sandbox_validation_and_diff_first_apply_parity_across_claimed_m5_ai_test_flows`).

It narrows the seventh of the seven families frozen by the
[test-intelligence component matrix](m5_test_intelligence_component_matrix.md) —
`test_generation_suggestion_card` — into one resolver plus a parity matrix, so a reviewer sees
*why* a test was proposed, *what* it assumes, and *which* classes of change it would apply
**before** any apply-capable action is offered.

## Why this exists

A user should never accept an AI-generated test through a one-click apply that bundles assertion
changes, helper / fixture additions, and snapshot / golden updates into one opaque diff path, or
that hides the assumptions the generator made. And an AI-assisted proposal should carry the same
diff-first preview, rollback, and evidence rules as any ordinary multi-file mutation flow. This
primitive keeps AI-assisted test suggestions review-first and assumption-visible, identically
across every claimed AI test-review consumer.

## Suggestion card

`resolve_test_generation_suggestion_card` takes one card's trigger source, target symbol / file
refs, uncovered-path / bug context, generated-test assumption classes, the review classes it
separates its churn into, its apply scope, its generated file count, and provenance class, and
derives a **suggestion posture** that is one-to-one with the apply scope:

| Apply scope | Suggestion posture | Apply-capable? |
| --- | --- | --- |
| `assertion_only` | `assertion_only_suggestion` | yes |
| `fixture_and_assertion` | `fixture_and_assertion_suggestion` | yes |
| `snapshot_included` | `snapshot_included_suggestion` | yes |
| `full_bundle_apply` | `full_bundle_suggestion` | no (held to review-first) |
| `review_required` | `review_required_suggestion` | no |
| `apply_blocked` | `apply_blocked_suggestion` | no |

Because the map is one-to-one, a full-bundle proposal never reads as an assertion-only apply. An
**apply-capable scope may only be offered when it names every review class it would apply**;
otherwise resolution fails with `ApplyScopeUnderstatesReviewClasses`. This is the
acceptance-criterion guarantee: assertion changes, helper / fixture additions, and snapshot /
golden updates are separated into distinct review classes before any apply-capable action, and a
snapshot / golden update can never be applied through an assertion-only click. A proposal that
mixes all three classes must therefore route through `full_bundle_apply` (or `review_required`),
which is never one-click apply. An **apply-capable generated card must disclose its assumption
summary**; otherwise resolution fails with `GeneratedWithoutAssumptionSummary`, so generated
assumptions are never hidden behind the assertions. An **apply-capable proposal must keep a
diff-first preview and a rollback**; otherwise resolution fails with
`ApplyWithoutDiffPreviewOrRollback`, so an AI-assisted proposal preserves the same preview,
rollback, and evidence rules as an ordinary multi-file mutation flow. The trigger source, targets,
context, assumption summary, review classes, and generated file count are always carried.

Actions: `reveal_suggestion_details` and `export_suggestion` are always offered; `run_in_sandbox`,
`open_diff_preview`, and `rollback_applied` whenever the card declares them; `apply_reviewed_classes`
only when the posture is apply-capable.

## Parity matrix

`M5SuggestionCardComponentsPacket` binds one row per claimed AI test-review consumer — the
suggestion review panel, the editor inline suggestion, the test-tree suggestion, the headless/CLI
suggestion, and the suggestion export — to the shared card anatomy, vocabulary, postures, actions,
export fields, and non-visual accessibility routes, so the same suggestion grammar holds across the
panel, the editor, the test tree, CI/headless, and support consumers with identical vocabulary.
Each row carries four hard invariants (all `false`):

- `bundles_assumption_fixture_or_snapshot_into_opaque_apply`
- `hides_trigger_source_or_target_symbols`
- `hides_assumption_summary_or_generated_file_count`
- `invents_alternate_suggestion_or_apply_state_label`

## Boundary

Raw generated source, pasted paths, credentials, and private endpoints stay outside the export
boundary; every card identity, target ref, and context ref is carried only as an opaque,
export-safe representation.

## Artifacts

- Canonical packet schema: `schemas/ui/m5-test-generation-suggestion-card.schema.json`
- Test-generation / diff-first-apply contract: `schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json`
- Support export: `artifacts/release/m5-test-generation-suggestion-card-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-test-generation-suggestion-card-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-test-generation-suggestion-card-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-test-generation-suggestion-card-primitive/`

All are minted from the seed builders by the
`aureline_runtime_test_generation_suggestion_card_primitive` headless emitter; the checked-in
support export is asserted equal to the seed builder in tests.
