# M5 Review-Request / Check / Merge-Queue Component Matrix (Design)

This is the design-side component inventory for the seven reusable M5 review
components frozen by
`aureline_review::current_stable_m5_review_component_matrix_export`. Every claimed
M5 review surface consumes this one shared component family instead of private row
text or provider-specific badges.

| Component | Maturity | Provider / local distinction | Stale-provider vocabulary | Browser-handoff boundary | Local-continue fallback | Source contract |
| --- | --- | --- | --- | --- | --- | --- |
| `review_request_row` | Stable | Provider-authored title/author/state labeled provider-backed; base/head + diff summary local-computed | `provider_fresh`, `provider_refreshing`, `provider_stale`, `local_only_continuation` | Opening on the provider host is an explicit handoff with labeled return path | Last-known provider fields labeled; local diff review continues offline | `schemas/review/review_workspace.schema.json` |
| `checks_summary_card` | Stable | Provider check verdicts labeled provider-backed; local re-runs labeled local | `provider_fresh`, `provider_refreshing`, `provider_stale`, `provider_conflict` | Full provider logs are an explicit handoff; triage never forces raw-provider nav | Last-known verdicts labeled stale; local re-run continues triage | `schemas/ci/pipeline_run_row.schema.json` |
| `pending_review_tray` | Stable | Provider-assigned requests labeled provider-backed; local publish-later reviews labeled local | `provider_fresh`, `provider_refreshing`, `provider_stale`, `local_only_continuation` | Opening a review on the provider is an explicit handoff | Last-known items labeled stale; local reviews continue offline | `schemas/review/review_surface_record.schema.json` |
| `merge_readiness_panel` | Stable | Provider branch policy / required-check gates labeled provider-backed; local readiness labeled local | `provider_fresh`, `provider_stale`, `provider_unreachable`, `local_only_continuation` | Resolving a provider block is an explicit handoff | Last-known gates labeled stale; local readiness continues without asserting approval | `schemas/review/landing_candidate.schema.json` |
| `merge_queue_entry` | Stable | Provider queue position/owner labeled provider-backed; local pre-merge estimate labeled local | `provider_fresh`, `provider_refreshing`, `provider_stale`, `provider_unreachable` | Managing the entry on the provider is an explicit handoff | Last-known position/owner labeled stale; local review continues | `schemas/review/merge_queue_entry.schema.json` |
| `stack_dependency_chip` | Beta | Provider stack relations labeled provider-backed; local topology labeled local | `provider_fresh`, `provider_stale`, `local_only_continuation` | Opening a stack parent on the provider is an explicit handoff | Last-known relation labeled stale; local topology from change lineage | `schemas/review/change_lineage.schema.json` |
| `approval_invalidation_banner` | Preview | Provider-recomputed approval labeled provider-backed; local prediction labeled local | `provider_fresh`, `provider_stale`, `provider_conflict`, `local_only_continuation` | Re-requesting approval on the provider is an explicit handoff | Last-known invalidation reason labeled stale; local review continues | `schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json` |

## Hard invariants

The `trust_review` block encodes the guardrails as invariants that must all hold:
provider-managed state is never flattened into a local estimate, stale-provider
downgrades are named, approval invalidation is never a generic warning pill,
browser handoff stays explicit, local continuation is preserved on degraded
freshness, and stack blocking, queue ownership, and check class stay explicit.
Ordinary check triage never forces raw-provider navigation. Downgrade narrows the
claim rather than hiding the component, and stale or underqualified rows block
promotion.
