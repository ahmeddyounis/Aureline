# Shared Review-Component Consumers: Label, Action, and Handoff Parity

This is the closing consumer-adoption lane for the seven reusable M5 review
components frozen in
`freeze_the_m5_review_request_check_and_merge_queue_component_matrix` and
implemented by the review-request-row, checks-summary-card, merge-readiness /
merge-queue / stack-dependency, and pending-review-tray / approval-invalidation-
banner lanes. It binds each shared component to the six consumer surfaces that
render it and proves — by fixtures, not screenshots — that the same review object
presents the same provider, queue, readiness, and staleness language wherever it
appears.

- Boundary schema: [`schemas/ui/m5-review-component-consumer.schema.json`](../../../schemas/ui/m5-review-component-consumer.schema.json)
- Support export: [`artifacts/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity/support_export.json`](../../../artifacts/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity/support_export.json)
- Protected fixtures: [`fixtures/ui/m5-review-component-consumers/`](../../../fixtures/ui/m5-review-component-consumers/)

## Consumers

| Consumer | Surface |
| --- | --- |
| `desktop_list` | Desktop review list |
| `detail_pane` | Review detail pane |
| `companion_triage` | Browser companion triage queue |
| `help_surface` | Help / About surface |
| `support_export` | Support packet |
| `exported_evidence` | Exported review evidence |

## Parity facets

For a given review object, every consumer surface must present identical values for
all four parity facets:

- `label` — the primary label for the component.
- `primary_action` — the primary action offered.
- `queue_readiness_status_language` — the queue / readiness / status language.
- `handoff_reason` — the handoff reason shown when the component hands off.

A surface may narrow *how much* it renders, but it may never reword any of these
values per surface. Narrowing never touches the parity facets; it is disclosed
additively through an explicit narrow banner.

## Render modes and disclosure

Render mode is derived from the object's provider freshness
(`resolve_review_component_render_disclosure`):

| Provider freshness | Render mode | Narrow reason | Local-continue note | Browser-handoff boundary |
| --- | --- | --- | --- | --- |
| `provider_fresh` | `full_parity` | — | no | no |
| `provider_refreshing` | `freshness_narrowed` | `provider_freshness_degraded` | no | no |
| `provider_stale` | `freshness_narrowed` | `provider_freshness_degraded` | yes | no |
| `provider_conflict` | `freshness_narrowed` | `provider_freshness_degraded` | yes | no |
| `provider_unreachable` | `handoff_required` | `browser_handoff_required` | yes | yes |
| `local_only_continuation` | `local_continue_fallback` | `local_continue_engaged` | yes | no |

A narrowed binding must carry a narrow banner naming the reason, the preserved
facets, and the next action. A full-parity binding must not carry a narrow banner.

## Honesty axes and guardrails

Two AC honesty axes anchor validation:

1. **Parity** — bindings that share a `review_object_id` must carry identical parity
   facet values (`ParityDriftAcrossSurfaces`).
2. **Proven reuse** — every one of the seven shared components must be adopted by at
   least two distinct consumers (`ReviewComponentReuseUnproven`), every component and
   consumer must appear (`ComponentCoverageMissing` / `ConsumerCoverageMissing`), and
   Help / support / exported-evidence bindings must point at the canonical component
   contracts (`HelpSupportExportReferenceMissing`).

Each binding also carries five guardrail row-invariants that must be false, mapping
to the spec guardrails:

- `forces_raw_provider_navigation_for_triage`
- `flattens_provider_state_into_local_estimate`
- `hides_approval_invalidation_behind_generic_pill`
- `rewords_labels_per_surface`
- `drops_handoff_reason_or_local_continue`

## Canonical component contracts

`component_canonical_schema_ref` maps each component to the schema of the lane that
implemented it:

| Component(s) | Canonical schema |
| --- | --- |
| `review_request_row` | `schemas/ui/m5-review-request-row.schema.json` |
| `checks_summary_card` | `schemas/ui/m5-checks-summary-card.schema.json` |
| `merge_readiness_panel`, `merge_queue_entry`, `stack_dependency_chip` | `schemas/ui/m5-merge-readiness-panel.schema.json` |
| `pending_review_tray`, `approval_invalidation_banner` | `schemas/ui/m5-pending-review-tray.schema.json` |

## Regenerating artifacts

The support export, Markdown summary, and fixtures are checked in. Regenerate them
after a contract change:

```sh
GEN_REVIEW_COMPONENT_CONSUMER_ARTIFACTS=1 cargo test -p aureline-review --lib \
  regenerate_review_component_consumer_artifacts
```
