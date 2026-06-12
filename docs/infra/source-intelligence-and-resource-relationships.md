# Infrastructure Source-Intelligence and Resource Relationships

This document defines the canonical M5 infrastructure source-intelligence contract. It freezes one shared vocabulary for Terraform/HCL, Kubernetes/Helm, devcontainer, CI/environment, and policy-manifest families and pairs that vocabulary with a concrete object packet so later infra, review, search, docs, AI, incident, and support surfaces reuse the same truth-layer, relation-edge, and stable-object model.

The machine-readable schemas are:

- [`/schemas/infra/source-intelligence-and-resource-relationships.schema.json`](../../schemas/infra/source-intelligence-and-resource-relationships.schema.json) for the qualification matrix.
- [`/schemas/infra/source-intelligence-object-packet.schema.json`](../../schemas/infra/source-intelligence-object-packet.schema.json) for the concrete object, relation, and consumer-projection packet.
- [`/schemas/infra/relation-graph-incident-support-parity.schema.json`](../../schemas/infra/relation-graph-incident-support-parity.schema.json) for incident/support/proof reopen parity over the shared graph packet.

The Rust validation model is in [`/crates/aureline-infra`](../../crates/aureline-infra/src/source_intelligence_and_resource_relationships/mod.rs). Fixtures live in [`/fixtures/infra/source-intelligence-and-resource-relationships`](../../fixtures/infra/source-intelligence-and-resource-relationships).

This packet extends, rather than replaces, the [target-context and control-plane boundary](./target-context-and-control-plane-boundary.md) and [cluster-context and live-resource](./cluster-context-and-live-resource.md) packets. Those packets govern exact target identity and live boundary safety; this matrix freezes which truth layers and relationship edges later infra surfaces are allowed to claim.

## Qualification Rule

An M5 infrastructure family row is promotable only when all of the following are true:

- the row covers all five truth layers: **authored/desired**, **rendered/expanded**, **planned/validated**, **observed/live**, and **provider-overlay**;
- each truth layer binds one explicit target-context requirement, live-access prerequisite, console-handoff posture, export-fidelity class, and file-intelligence downgrade posture;
- relation edges are expressed with the shared vocabulary: `source_of_render`, `plan_for`, `live_counterpart_of`, `applied_by`, `owned_by`, `impacts`, `runbook_reference`, `review_anchor`, and `provider_overlay_of`;
- file-only, inspect-only, and handoff-only downgrade profiles are spelled out for every family instead of being implied by product copy;
- provider overlays remain explicit enrichments or handoff destinations rather than silent replacements for repo-owned or live truth.

Packets that fail any error-severity check are not promotable. The affected family must narrow to file-only, inspect-only, handoff-only, or a stricter claim.

## Family Coverage

- **Terraform/HCL** binds plan, live counterpart, apply provenance, impact, runbook, review, and provider-overlay edges around repo-owned source and provider-backed live resources.
- **Kubernetes/Helm** binds source-to-render, plan, live counterpart, ownership/controller, impact, runbook, review, and overlay edges around rendered manifests and live cluster objects.
- **Devcontainer** binds source-to-render, live counterpart, ownership, impact, review, and overlay edges around resolved workspace/container state.
- **CI/environment** binds plan, apply provenance, impact, runbook, review, and overlay edges around workflow definitions, rollout previews, hosted runs, and environment slices.
- **Policy-manifest** binds source-to-render, plan, ownership/scope, impact, runbook, review, and overlay edges around compiled policy, validation results, and enforcement observations.

## Fixture Meaning

- `qualified_matrix_packet.json` proves the full five-family matrix with all five truth layers, the shared edge vocabulary, and explicit file-only, inspect-only, and handoff-only profiles.
- `file_only_downgraded_matrix_packet.json` proves that the same matrix remains valid when the focus is explicit downgrade posture instead of stable-qualified depth language.
- `missing_truth_layer_and_profile_packet.json` intentionally fails validation by omitting one truth layer and one downgrade profile from the Terraform row.
- `qualified_object_packet.json` proves that every claimed family emits stable authored, rendered, planned, observed, and provider-overlay objects plus shared graph/review/docs/incident projections.
- `missing_rendered_lineage_object_packet.json` intentionally fails validation by stripping rendered/planned lineage fields that must preserve authored source paths and tool identity/version.

## Canonical Object Packet

The object packet instantiates the matrix as actual infrastructure facts instead of leaving the contract at vocabulary-only level.

- Every object carries a stable object id, infrastructure family, truth layer, target-context ref, freshness label, authority posture, provenance refs, and a redaction-safe support summary.
- Derived objects preserve authored lineage through `authored_object_refs`, `source_input_refs`, and `known_path_back_to_source_refs`; rendered and planned objects additionally preserve tool identity and version where known.
- Stable identities capture selectors and owners directly, so Terraform addresses, Kubernetes selectors, devcontainer/workspace handles, CI environment selectors, and policy scopes are not trapped inside raw text viewers.
- Relation edges stay in the shared vocabulary and bind concrete objects rather than private caches.
- Consumer projections for code/graph, review, docs, and incident surfaces resolve object and relation refs from the shared packet and explicitly forbid hidden side caches.
- Every claimed surface reuses the same packet to serve `show live counterpart`, `show applied-by`, `show owned-by`, `show impacts`, and `explain this environment slice` flows instead of silently falling back to raw search or browser tabs.
- If a surface narrows one environment slice or drops an edge needed by those flows, the validator marks the packet underqualified rather than letting the UI imply stable coverage.

## Incident, Support, and Proof Parity

The companion [relation-graph incident/support parity packet](./relation-graph-incident-support-parity.md) freezes how incident packets, support exports, and proof corpora reopen the exact graph state a user saw. It carries:

- exact relation-set signatures for reopened environment slices;
- stale-live overlay posture instead of a flattened “graph is stale” banner;
- connector-skew and local/remote/managed mismatch labels;
- control-plane handoff lineage and return-anchor refs;
- per-family wrong-target, stale-live, missing-permission, skew, and locality drills.

## Support Export Posture

Support exports may include packet ids, family ids, truth-layer names, relation-edge names, target-context requirement classes, required context fields, live-access prerequisites, console-handoff posture, export-fidelity class, downgrade posture, and redaction-safe summaries. They must not include raw provider payloads, raw credential material, raw kubeconfig or cloud profile bodies, private endpoint URLs, browser cookies, or hidden mutation instructions.

## Predecessor Notes

The earlier [`/docs/devops/resource_relationship_matrix.md`](../devops/resource_relationship_matrix.md) document remains useful background, but this infra packet is the canonical M5 source of truth for later implementation rows.
