# Structured-Artifact Review Component Accessibility, Headless, and Export Parity

This is the accessibility / headless / export capstone over the nine reusable M5
structured-artifact review components frozen in
`freeze_the_m5_structured_artifact_review_component_matrix`, implemented by the
artifact-identity / diff-mode, structure-row / compare-summary, merge-decision /
generated-notice, and rendered-compare / media-rail / redaction-trust-badge lanes, and
adopted by the shared consumers in
`add_shared_diff_toolbar_merge_sheet_review_workspace_help_support_and_export_consumers_so_artifact_review_components_keep_mode_risk_and_provenance_language_aligned`.

Where the consumer lane proves mode / risk / provenance parity across desktop
surfaces, this lane proves the harder claim: that artifact class, diff mode, structure,
rendered-compare fallback, and merge-decision state is exposed just as honestly in
assistive, headless, and exported forms as it is on the desktop — and that a
claim-bearing component automatically narrows the moment its parser/schema certainty,
render trust, write-back safety, or metadata availability stops being trustworthy.

- Boundary schema: [`schemas/ui/m5-structured-artifact-review-component-accessibility-parity.schema.json`](../../../schemas/ui/m5-structured-artifact-review-component-accessibility-parity.schema.json)
- Support export: [`artifacts/release/m5-structured-artifact-review-accessibility-proof/support_export.json`](../../../artifacts/release/m5-structured-artifact-review-accessibility-proof/support_export.json)
- Protected fixtures: [`fixtures/ui/m5-structured-artifact-review-component-accessibility-parity/`](../../../fixtures/ui/m5-structured-artifact-review-component-accessibility-parity/)

## Parity across forms (AC1)

Every claimed component exposes five parity fields and renders on all three surfaces:

- `keyboard_label` — how the component is focused and operated by keyboard.
- `screen_reader_label` — the non-visual label, including its claim.
- `cli_enum_token` — the stable enum a headless CLI prints.
- `export_enum_token` — the stable enum the support export carries.
- `explanation_field` — a human-readable explanation of the current claim.

`rendering_surfaces` must cover `desktop_full`, `cli_headless`, and `support_export`.
No component may be pointer-only (`is_pointer_only`), export-opaque
(`is_export_opaque`), or semantically stronger on the desktop than in CLI or export
(`desktop_stronger_than_cli`) — all three guardrails must be false.

## Automatic claim narrowing (AC2)

Each component carries a claim about how much structured or rendered fidelity it
asserts, drawn from `ArtifactReviewClaimTier` (strongest first):

| Claim tier | Meaning | Rank |
| --- | --- | --- |
| `full_structured_fidelity` | Full semantic and rendered fidelity with write-back safety | 5 |
| `structured_compare_only` | Full structure, but compare-only: write-back is unavailable | 4 |
| `partial_structure` | Structured mode covers only part of the artifact; parser/schema uncertain | 3 |
| `raw_fallback_disclosed` | An explicit raw/export-safe fallback; render trust unavailable | 2 |
| `metadata_withheld` | Metadata or content is withheld or redacted | 1 |

`resolve_artifact_review_claim_narrowing` maps each condition to the ceiling it
permits, the trigger it must disclose, and its next action:

| Condition | Permitted ceiling | Trigger | Next action | Raw-fallback note | Compare-only note | Redaction note |
| --- | --- | --- | --- | --- | --- | --- |
| `structured_truth_trusted` | `full_structured_fidelity` | — | — | no | no | no |
| `parser_schema_uncertain` | `partial_structure` | `parser_schema_uncertain` | `reparse_against_schema` | yes | no | no |
| `render_trust_unavailable` | `raw_fallback_disclosed` | `render_trust_unavailable` | `review_raw_safe_fallback` | yes | no | no |
| `write_back_safety_unavailable` | `structured_compare_only` | `write_back_safety_unavailable` | `keep_compare_only` | yes | yes | no |
| `metadata_unavailable` | `metadata_withheld` | `metadata_availability_unavailable` | `restore_metadata_access` | yes | no | yes |

A component's `effective_claim` may never exceed the ceiling its condition permits
(`ClaimCeilingExceeded`) — this is the AC2 device that prevents a review, Help, or
export surface from overstating structured or rendered fidelity. A weakening condition
must carry an explicit `narrowing` disclosure pinned to that ceiling
(`ClaimNarrowingMissing` / `NarrowedToMismatch` / `NarrowTriggerMismatch` /
`NarrowNextActionMismatch`), keep the raw/export-safe fallback explicit
(`RawFallbackNoteMissing`), never silently promote a compare-only artifact to a
writable state (`CompareOnlyNoteMissing`), and keep redacted or withheld metadata
labeled (`RedactionNoteMissing`). A `structured_truth_trusted` row must not carry a
narrowing (`ClaimNarrowingUnexpected`).

## Coverage

The canonical row set covers all nine components (`ComponentCoverageMissing`), all five
conditions (`ConditionCoverageMissing`), and all five claim tiers reached as an
effective claim (`ClaimTierCoverageMissing`). Every row points at its component's
canonical schema and the frozen component matrix
(`CanonicalContractReferenceMissing`).

## Canonical component contracts

`component_canonical_schema_ref` maps each component to the schema of the implement
lane that produced it:

| Component(s) | Canonical schema |
| --- | --- |
| `artifact_identity_bar`, `diff_mode_switcher` | `schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json` |
| `structure_row`, `compare_summary_card` | `schemas/ui/m5-structure-compare-summary-controls.schema.json` |
| `merge_decision_row`, `generated_artifact_notice` | `schemas/ui/m5-merge-decision-generated-notice-controls.schema.json` |
| `rendered_compare_viewer`, `media_metadata_rail`, `redaction_or_trust_badge_set` | `schemas/ui/m5-rendered-compare-media-trust-controls.schema.json` |

## Regenerating artifacts

The support export, Markdown summary, and fixtures are checked in. Regenerate them
after a contract change:

```sh
GEN_ARTIFACT_REVIEW_ACCESSIBILITY_ARTIFACTS=1 cargo test -p aureline-review --lib \
  regenerate_artifact_review_accessibility_artifacts
```
