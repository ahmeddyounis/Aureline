# M5 graph-governance matrix

This document describes the canonical packet that freezes the **M5 graph-governance
matrix** — the single qualification report that graduates the M5 workset-scope,
graph-topology, impact-query, ownership-source, architecture-explainer, graph-freshness,
and navigation-recall depth lanes. It aggregates the stable-line graph-truth packets into
one governance gate that automatically narrows or withholds the published claim of any lane
that only knows the current workset, hot set, or policy-limited slice, whose graph has gone
stale, whose relations are approximate, or whose explanation lost its citations. It is the
user-facing companion to the governed artifact at
`artifacts/graph/m5/m5-graph-governance.json` and the typed model in the `aureline-graph`
crate (`m5_graph_governance`).

This packet answers the code-understanding depth question for the graph lane as a whole:
**does this lane help a user understand a large workspace without hiding slice boundaries,
stale graph state, approximate relations, or generated-explanation limits — or is it
automatically downgraded to a narrower label before publication?**

## What this packet covers

The packet carries one governance row for every claimed M5 depth lane, and each row is
pinned to the canonical graph-truth packet it draws its evidence from:

1. **`workset_scope`** — scope provenance (`scope_provenance_truth_packet.json`).
2. **`graph_topology`** — semantic graph object model and query contract
   (`semantic-graph-object-model-and-query-contract.json`).
3. **`impact_query`** — knowledge evidence (`knowledge_evidence_packet.json`).
4. **`ownership_source`** — knowledge evidence (`knowledge_evidence_packet.json`).
5. **`architecture_explainer`** — audit/topology/explainer companion truth
   (`audit_topology_explainer_companion_truth_packet.json`).
6. **`graph_freshness`** — freshness propagation (`freshness_propagation_packet.json`).
7. **`navigation_recall`** — navigation target truth (`navigation_target_truth_packet.json`).

The workset-scope, impact-query, and navigation-recall lanes are **scope-sensitive**: they
reason over a slice of the workspace and must narrow safely rather than inherit a broader
whole-workspace claim.

Each row answers, for its lane:

- **Who owns it?** An `owner` accountable for the lane's evidence and conformance.
- **How much of the workspace is known?** A `scope_mode` of `full_workspace`, `workset`,
  `hot_set`, or `unscoped`. A **workset** slice caps the lane at scope-qualified, a
  **hot_set** slice at provisional, and an **unscoped** lane is withheld.
- **How fresh is the graph?** A `graph_freshness` of `fresh`, `lagging`, `stale`, or
  `expired`. A **stale** index caps at provisional; an **expired** one withholds.
- **How exact are the relations?** A `relation_fidelity` of `exact`, `resolved`,
  `approximate`, or `unresolved`. An **approximate** relation caps at provisional; an
  **unresolved** one withholds.
- **How is the explanation backed?** An `evidence_backing` of `curated`, `cited`,
  `generated`, or `uncited`. A **generated** prose caps at provisional; an **uncited** one
  withholds.
- **What did the impact query find?** An `impact_result_class` of `no_impact`,
  `in_scope_impact`, `out_of_scope`, or `policy_limited`, with a `hidden_result_count` and an
  `out_of_scope_count`, so a no-impact answer is never confused with an out-of-scope or
  policy-limited one.
- **Are the identities stable?** A `node_id_namespace` and an `edge_id_namespace` under the
  packet-level `topology_identity_scheme`, so topology node and edge IDs stay stable across
  views.
- **What is still supported?** The `supported_scopes` slice labels the lane still backs, the
  `caveats` attached to the published claim, and the `stale_or_missing_fields` that drove any
  narrowing.
- **What recovery applies?** A `downgrade_path` of `widen_scope`, `reindex`,
  `resolve_relations`, `cite_or_curate`, `withhold_claim`, or `none`. A narrowed or withheld
  lane always offers a real path.
- **What backs it?** A `packet_ref` (the canonical source packet), a `conformance_ref`, an
  `evidence_ref`, a `governance_receipt_ref` for the machine-readable receipt, and a
  `release_evidence_ref`, `help_surface_ref`, `docs_badge_ref`, and `support_export_ref` so
  release evidence, help/service-health, docs badges, and support exports ingest the same
  packet.

## The governance gate

The `published_claim` a lane may publish is the **weakest ceiling** implied by its observed
states, computed as the minimum of the lane's declared claim and the ceilings of its
scope-mode, graph-freshness, relation-fidelity, and evidence-backing states. Ordered
low-to-high, the claims are `withheld` < `provisional` < `scope_qualified` <
`authoritative`.

Each input caps the published claim:

- **Scope mode** caps at `authoritative` for `full_workspace`, `scope_qualified` for
  `workset`, `provisional` for `hot_set`, and `withheld` for `unscoped`.
- **Graph freshness** caps at `authoritative` for `fresh`, `scope_qualified` for `lagging`,
  `provisional` for `stale`, and `withheld` for `expired`.
- **Relation fidelity** caps at `authoritative` for `exact`, `scope_qualified` for
  `resolved`, `provisional` for `approximate`, and `withheld` for `unresolved`.
- **Evidence backing** caps at `authoritative` for `curated`, `scope_qualified` for `cited`,
  `provisional` for `generated`, and `withheld` for `uncited`.

The `governance_decision` records the gate's action, derived one-to-one from the published
claim:

- **`publish`** — the lane is authoritative.
- **`qualify_scope`** — the lane is narrowed to the active workset, hot set, or resolved
  slice.
- **`mark_provisional`** — the lane is held at a provisional label.
- **`withhold`** — the lane's claim is withheld; no publishable claim.

A lane is **downgraded** whenever its published claim is lower than the level it declared: a
slice-only, stale, approximate, or uncited lane that wanted a stronger claim has its
published claim lowered automatically rather than left quietly authoritative.

The `downgrade_reasons` are the headline triggers recomputed from the observed states:
`scope_narrowed`, `stale_graph`, `approximate_relations`, and `uncited_explanation`. The
stored `published_claim`, `governance_decision`, and `downgrade_reasons` must all equal the
recomputed gate decision, so a lane can neither overstate its claim nor hide a downgrade by
hand.

## Keeping impact answers distinct

An impact query never collapses three different answers into one empty result. The
`impact_result_class` distinguishes `no_impact` (the query resolved fully and found nothing),
`in_scope_impact` (impact within the current scope), `out_of_scope` (candidate impacts
resolve outside the current slice — with a non-zero `out_of_scope_count`), and
`policy_limited` (results suppressed by an access or visibility policy — with a non-zero
`hidden_result_count`). An authoritative lane is forbidden from hiding any result, so a
whole-workspace claim never has a non-zero hidden or out-of-scope count.

## The guardrails

A lane **never implies whole-workspace certainty when it only knows the current workset, hot
set, or policy-limited slice**. Several mechanisms enforce this:

- A `workset`/`hot_set`/`unscoped` scope, a `stale`/`expired` graph, an
  `approximate`/`unresolved` relation, or `generated`/`uncited` explanation caps the
  published claim below `authoritative` and raises a downgrade reason, so the `workset_scope`,
  `impact_query`, `architecture_explainer`, `graph_freshness`, and `navigation_recall` lanes
  are visibly narrower than the clean `graph_topology` and `ownership_source` lanes.
- A slice-only or stale lane is **downgraded automatically**: the `workset_scope` lane, which
  knows only the active workset, is qualified to that slice rather than remaining
  authoritative.
- A narrowed or withheld lane must offer a real `downgrade_path` (not `none`), list at least
  one `caveat`, and name the `stale_or_missing_fields` that drove the narrowing, so a degraded
  lane never drops its recovery semantics or hides why it was narrowed.
- Every row carries a `release_evidence_ref`, `help_surface_ref`, `docs_badge_ref`, and
  `support_export_ref` so release evidence, help/service-health, docs, and support exports
  **ingest the same governance packet** rather than parallel spreadsheets — a narrowed lane
  cannot stay authoritative in one surface while it is downgraded in another.

The governance model is not a blanket downgrade: the `graph_topology` and `ownership_source`
lanes show that a full-workspace, fresh, exact, curated graph publishes a clean authoritative
claim. The `navigation_recall` lane shows the opposite extreme — an unscoped, expired,
unresolved, uncited lane is withheld entirely with a `withhold_claim` recovery rather than
implying whole-workspace recall.

## Per-lane rows

| Lane | Scope | Freshness | Relations | Backing | Published | Decision |
| --- | --- | --- | --- | --- | --- | --- |
| `workset_scope` | `workset` | `fresh` | `exact` | `curated` | `scope_qualified` | `qualify_scope` |
| `graph_topology` | `full_workspace` | `fresh` | `exact` | `curated` | `authoritative` | `publish` |
| `impact_query` | `hot_set` | `lagging` | `approximate` | `cited` | `provisional` | `mark_provisional` |
| `ownership_source` | `full_workspace` | `fresh` | `exact` | `curated` | `authoritative` | `publish` |
| `architecture_explainer` | `full_workspace` | `fresh` | `resolved` | `generated` | `provisional` | `mark_provisional` |
| `graph_freshness` | `full_workspace` | `stale` | `exact` | `curated` | `provisional` | `mark_provisional` |
| `navigation_recall` | `unscoped` | `expired` | `unresolved` | `uncited` | `withheld` | `withhold` |

## Consuming this packet

Downstream surfaces render the packet's export projection — the **graph-depth index** —
instead of restating each lane's claim by hand:

- <a id="release-evidence"></a>**Release evidence** ingests the per-lane
  `release_evidence_ref` and the graph-depth index so a release only marks a lane ready when
  it publishes an `authoritative` claim.
- <a id="help-surface"></a>**Help and service-health** surfaces ingest the `help_surface_ref`
  so a downgraded lane reads as narrowed there too, never authoritative by inertia.
- <a id="docs-badges"></a>**Docs badges** ingest the `docs_badge_ref` so docs and help copy
  derive scope, freshness, confidence, and generated-versus-curated state from this artifact
  rather than feature-local prose.
- <a id="support-export"></a>**Support exports** ingest the `support_export_ref`,
  `governance_receipt_ref`, and `packet_ref` so field triage can reconstruct which lane was
  published, qualified, provisioned, or withheld — and why — without re-running the
  conformance suites.

The packet is metadata-only: every field is a typed state, a count, or an opaque ref, and it
carries no credential bodies, raw provider payloads, or graph node contents.
