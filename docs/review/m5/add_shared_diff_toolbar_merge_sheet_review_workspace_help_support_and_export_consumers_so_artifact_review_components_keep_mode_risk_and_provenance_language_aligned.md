# Shared Artifact-Review Component Consumers: Mode, Risk, and Provenance Parity

This is the closing consumer-adoption lane for the nine reusable M5 structured-
artifact review components frozen in
`freeze_the_m5_structured_artifact_review_component_matrix` and implemented by the
artifact-identity / diff-mode, structure-row / compare-summary, merge-decision /
generated-notice, and rendered-compare / media-rail / redaction-trust-badge lanes.
It binds each shared component to the six consumer surfaces that render it and
proves — by fixtures, not screenshots — that the same artifact object presents the
same artifact-class, canonical-source, diff-mode, compare-risk, and generated-from
language wherever it appears.

- Boundary schema: [`schemas/ui/m5-structured-artifact-review-component-consumer.schema.json`](../../../schemas/ui/m5-structured-artifact-review-component-consumer.schema.json)
- Support export: [`artifacts/release/m5-structured-artifact-review-consumers-proof/support_export.json`](../../../artifacts/release/m5-structured-artifact-review-consumers-proof/support_export.json)
- Protected fixtures: [`fixtures/ui/m5-structured-artifact-review-component-consumers/`](../../../fixtures/ui/m5-structured-artifact-review-component-consumers/)

## Consumers

| Consumer | Surface |
| --- | --- |
| `diff_toolbar` | Diff / compare toolbar |
| `merge_sheet` | Merge / conflict resolution sheet |
| `review_workspace` | Review workspace |
| `help_surface` | Help / About surface |
| `support_packet` | Support packet |
| `exported_view` | Exported review evidence / artifact view |

## Parity facets

For a given artifact object, every consumer surface must present identical values
for all four parity facets:

- `canonical_source_label` — the artifact-class / canonical-source label.
- `mode_action` — the primary diff-mode / action offered.
- `risk_status_language` — the compare risk / status language.
- `provenance_relation` — the generated-from / source-of-truth relation.

A surface may narrow *how much* it renders, but it may never reword any of these
values per surface. Narrowing never touches the parity facets; it is disclosed
additively through an explicit narrow banner.

## Render modes and disclosure

Render mode is derived from the object's render/schema fidelity, reused directly
from the frozen matrix (`resolve_artifact_component_render_disclosure`):

| Render fidelity | Render mode | Narrow reason | Raw-fallback note | Redaction note |
| --- | --- | --- | --- | --- |
| `structured_faithful` | `full_parity` | — | no | no |
| `structured_partial` | `structured_fidelity_narrowed` | `structured_fidelity_degraded` | yes | no |
| `render_untrusted` | `structured_fidelity_narrowed` | `structured_fidelity_degraded` | yes | no |
| `schema_unrecognized` | `raw_fallback_disclosed` | `structured_mode_unavailable_raw_fallback` | yes | no |
| `raw_fallback` | `raw_fallback_disclosed` | `structured_mode_unavailable_raw_fallback` | yes | no |
| `redacted_or_withheld` | `redaction_narrowed` | `content_redacted_or_withheld` | no | yes |

A narrowed binding must carry a narrow banner naming the reason, the preserved
facets, and the next action. A full-parity binding must not carry a narrow banner.
The raw / export-safe fallback stays explicit whenever render or schema fidelity
narrows below faithful structure, and redacted content narrows through its own
explicit redaction-posture note.

## Honesty axes and guardrails

Two AC honesty axes anchor validation:

1. **Parity** — bindings that share an `artifact_object_id` must carry identical
   parity facet values (`ParityDriftAcrossSurfaces`).
2. **Proven reuse** — every one of the nine shared components must be adopted by at
   least two distinct consumers (`ArtifactComponentReuseUnproven`), every component
   and consumer must appear (`ComponentCoverageMissing` / `ConsumerCoverageMissing`),
   and Help / support / exported-view bindings must point at the canonical component
   contracts (`HelpSupportExportReferenceMissing`).

Each binding also carries five guardrail row-invariants that must be false, mapping
to the spec guardrails:

- `promotes_compare_only_to_writable_state`
- `flattens_structured_mode_without_explanation`
- `hides_generated_from_relation_behind_generic_chrome`
- `drops_raw_or_export_safe_fallback`
- `rewords_artifact_labels_per_surface`

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
GEN_ARTIFACT_REVIEW_CONSUMER_ARTIFACTS=1 cargo test -p aureline-review --lib \
  regenerate_artifact_review_consumer_artifacts
```
