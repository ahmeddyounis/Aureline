# Fixtures: M5 impact-query packet

This directory contains fixture metadata for the `m5_impact_query_packet`.

The canonical packet is checked in at:

`artifacts/graph/m5/m5-impact-query.json`

and validated by the typed model in the `aureline-graph` crate (`m5_impact_query`) and the JSON
Schema at `schemas/graph/m5-impact-query.schema.json`.

## Coverage

- **Distinct empty states.** The corpus declares one query of every result class —
  `in_scope_impact`, `no_impact`, `unknown`, `out_of_scope`, `policy_limited`,
  `provider_unavailable`, and `stale_graph` — so an empty answer never collapses into one
  misleading "no impact" message. Only the `no_impact` query is genuinely empty-and-safe, and it
  hides no out-of-scope or policy-limited objects.
- **Included versus withheld counts.** The `in_scope_impact` query shows three affected objects
  (one `exact`, one `partial`, one `stale`, each non-exact carrying an `evidence_reason`) while
  also counting one out-of-scope object, so the included count is visible alongside the withheld
  count. The `out_of_scope` and `policy_limited` queries carry their out-of-scope and hidden counts
  even though they show no objects.
- **Explicit widen or refresh.** Each narrowed query offers the remediation action its reason
  demands — `widen_scope`, `refresh_index`, `connect_provider`, `request_policy_access`, or
  `resolve_relations` — and the definitive `no_impact` query offers exactly `none`.
- **Survives beyond one panel.** Each of `impact_panel`, `refactor_planning`,
  `review_explanation`, `topology_card`, and `support_export` carries exactly one binding, stamped
  with the active snapshot and scope. The `support_export` binding carries every declared query, so
  support and evidence packets can reconstruct each answer.
- **Upstream provenance.** The packet binds to the canonical graph-depth governance matrix
  (`artifacts/graph/m5/m5-graph-governance.json`), the workset-scope packet
  (`artifacts/graph/m5/m5-workset-scope.json`), and the topology-identity packet
  (`artifacts/graph/m5/m5-topology-identity.json`) whose node identity space it reuses.

## Guardrails proven

- A non-in-scope result with no `empty_reason` fails validation (`MissingEmptyReason`).
- A `no_impact` result that carries an out-of-scope or hidden count fails validation
  (`NoImpactHidesObjects`).
- A narrowed result that offers no action, or omits the action its reason demands, fails validation
  (`MissingRemediationAction`, `MissingRequiredAction`).
- A non-exact affected object with no `evidence_reason` fails validation
  (`UnlabeledAffectedEvidence`).
- A query or affected-object permalink that is empty or does not embed its id fails validation
  (`UnsafeQueryPermalink`, `UnsafeAffectedPermalink`).
- A binding not stamped with the active snapshot or scope, or a query not carried by the
  support-export binding, fails validation (`SnapshotBindingMismatch`, `ScopeIdMismatch`,
  `QueryMissingFromSupportExport`).
