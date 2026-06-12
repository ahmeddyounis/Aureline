# M5 impact-query packet

This document describes the canonical packet that carries the **M5 impact-query answers** — the
honest result a change-impact or architecture-explorer query returns, keeping *no impact found*
distinct from *unknown*, *out of scope*, *policy limited*, *provider unavailable*, and *stale
graph*. Where the [workset-scope packet](m5-workset-scope.md) answers *what slice am I looking
at?*, the [topology-identity packet](m5-topology-identity.md) answers *which exact graph object is
this?*, and the [graph-governance matrix](m5-graph-governance.md) freezes *which depth claim a lane
may publish*, this packet answers the question refactor planning, review explanation, topology
cards, and support all ask of an impact query: **is this empty answer safe, or does it merely mean
the graph could not see far enough?**

It is the user-facing companion to the governed artifact at
`artifacts/graph/m5/m5-impact-query.json` and the typed model in the `aureline-graph` crate
(`m5_impact_query`).

## Why empty answers are not all the same

An empty impact answer is the dangerous case: presented as a bare "no impact", it reads as *safe
to proceed* even when the graph simply could not see the affected objects. This packet refuses to
collapse those states. Every result carries an [`ImpactResultClass`](#result-classes), and an empty
answer must say which one it is.

### Result classes

| Class | Meaning | Empty answer is… |
| --- | --- | --- |
| `in_scope_impact` | Affected objects were found within the active scope | not empty |
| `no_impact` | The query resolved fully across the active scope and nothing depends on the subject | **safe** |
| `unknown` | Impact could not be determined — the graph has an unresolved gap | not safe |
| `out_of_scope` | Candidate impact resolves outside the active workset slice | not safe |
| `policy_limited` | Affected objects are suppressed by an access or visibility policy | not safe |
| `provider_unavailable` | A required provider connection is unavailable | not safe |
| `stale_graph` | The backing graph index is stale or expired | not safe |

Only `no_impact` means an empty answer is genuinely safe, and a `no_impact` result **may not hide
out-of-scope or policy-limited objects** — if any affected object was withheld, the result is
classed by *why* it was withheld instead.

## What this packet covers

Each entry in `queries` is an `ImpactQueryResult` carrying:

- the `subject_ids` it reasons about (the changed or selected objects), in the shared topology
  identity space, and the `subject_kind`;
- the `included_objects` it shows — each an `AffectedObject` with a canonical `node_id`, an
  `evidence_class` drawn from the stable relation-fidelity vocabulary (`exact`, `approximate`,
  `imported`, `partial`, `stale`, `blocked`), and — for any non-`exact` class — an explicit
  `evidence_reason`;
- the `included_count` shown **alongside** the `out_of_scope_count` and `hidden_count` that were
  withheld, so a user can see how many affected objects were included versus hidden or out of scope
  before approving or trusting a graph-backed action;
- an `evidence_summary` breaking the included objects down per evidence class;
- a `freshness` and `confidence` token;
- the `result_class` and, for every non-`in_scope_impact` result, an explicit `empty_reason`;
- the `remediation_actions` the answer offers; and
- an `export_permalink` that embeds the canonical query id.

### Explicit widen or refresh, never silent broadening

When an answer is narrowed by scope, policy, freshness, or a missing provider, the packet requires
the matching `RemediationAction` rather than silently broadening:

| Result class / condition | Required action |
| --- | --- |
| `out_of_scope` or any `out_of_scope_count > 0` | `widen_scope` |
| `stale_graph` | `refresh_index` |
| `provider_unavailable` | `connect_provider` |
| `policy_limited` or any `hidden_count > 0` | `request_policy_access` |
| `unknown` | `resolve_relations` |
| `in_scope_impact` / `no_impact`, fully in scope | `none` |

A definitive, fully-in-scope answer carries exactly `["none"]`, so it never implies a recovery path
it does not need.

### Carried beyond one panel

Each of `impact_panel`, `refactor_planning`, `review_explanation`, `topology_card`, and
`support_export` carries exactly one `ImpactConsumerBinding`, stamped with the active snapshot and
scope. The same answer therefore survives into AI refactor planning, review explanation, codebase
topology cards, and the support export rather than living in one panel render. The
`support_export` binding **must carry every declared query**, so support and evidence packets can
reconstruct the impact result and its scope or freshness class without scraping panel text.

## Guardrails proven

- A non-in-scope result with no `empty_reason` fails validation (`MissingEmptyReason`).
- A `no_impact` result that carries an out-of-scope or hidden count, or any included object, fails
  validation (`NoImpactHidesObjects`).
- An `in_scope_impact` result with no included objects fails validation
  (`InScopeImpactWithoutObjects`); an `out_of_scope` result with no count fails
  (`OutOfScopeWithoutCount`); a `policy_limited` result with no hidden count fails
  (`PolicyLimitedWithoutHidden`).
- A narrowed result that offers no real action, or omits the action its reason demands, fails
  validation (`MissingRemediationAction`, `MissingRequiredAction`); a definitive in-scope result
  that offers a recovery path fails (`UnexpectedRemediationAction`).
- A non-exact affected object with no `evidence_reason` fails validation
  (`UnlabeledAffectedEvidence`).
- A query or affected-object permalink that is empty or does not embed its id fails validation
  (`UnsafeQueryPermalink`, `UnsafeAffectedPermalink`).
- An `evidence_summary` that disagrees with the included objects fails validation
  (`EvidenceSummaryMismatch`); an `included_count` that disagrees with the body fails
  (`IncludedCountMismatch`).
- A binding not stamped with the active snapshot or scope fails validation
  (`SnapshotBindingMismatch`, `ScopeIdMismatch`); a query not carried by the support-export binding
  fails (`QueryMissingFromSupportExport`).

## Upstream provenance

The packet binds to the canonical graph-depth governance matrix
(`artifacts/graph/m5/m5-graph-governance.json`), the workset-scope packet
(`artifacts/graph/m5/m5-workset-scope.json`), and the topology-identity packet
(`artifacts/graph/m5/m5-topology-identity.json`) whose node identity space its subjects and
affected objects reuse.

<a id="help-surface"></a>
<a id="impact-badge"></a>

## Help surface and impact badge

Help and docs surfaces narrow from this one packet: the **impact badge** reflects the result class
of an answer, and the help surface explains each class and its remediation action rather than
re-describing impact state by hand.
