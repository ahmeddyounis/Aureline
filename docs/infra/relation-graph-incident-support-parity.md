# Infrastructure Relation-Graph Incident/Support Parity

This document defines the checked-in packet that keeps infrastructure relation-graph state reopenable across incident packets, support exports, and proof corpora. It composes the shared [source-intelligence object packet](./source-intelligence-and-resource-relationships.md) with explicit reopen bindings, stale-live overlay posture, connector-skew posture, local/remote/managed mismatch labels, and control-plane handoff lineage so later incident and support flows do not rely on memory or browser history.

The machine-readable schema is:

- [`/schemas/infra/relation-graph-incident-support-parity.schema.json`](../../schemas/infra/relation-graph-incident-support-parity.schema.json)

The Rust validation model is in [`/crates/aureline-infra`](../../crates/aureline-infra/src/relation_graph_incident_support_parity/mod.rs). Fixtures live in [`/fixtures/infra/relation-graph-incident-support-parity`](../../fixtures/infra/relation-graph-incident-support-parity).

## Qualification Rule

An infrastructure relation-graph parity packet is promotable only when all of the following are true:

- the packet embeds a valid shared source-intelligence object packet instead of reconstructing a private graph;
- every graph selection preserves exact object ids, relation ids, truth layers, freshness labels, authority posture, stale-live overlay state, connector-skew state, locality mismatch state, and handoff lineage refs;
- incident packet, support export, and proof corpus bindings all reopen the same environment slice and relation-set signature with `exact_reopen_only = true`;
- permission-limited, stale-overlay, wrong-target, connector-skew, and local/remote/managed mismatch drills are explicit for every claimed infrastructure family and stay included in the proof packet;
- handoff lineage keeps the control-plane boundary visible rather than collapsing provider-owned follow-up into a generic shell or implied product authority.

Packets that fail any error-severity check are not promotable. The affected surface must narrow or withhold the claim.

## Exported Graph State

- A relation-graph selection binds one infrastructure family, one environment context, one primary object, the exact visible object roots, the exact visible relation ids, the visible truth layers, visible freshness labels, authority posture, overlay state, connector-skew state, locality mismatch state, and a stable `relation_set_signature`.
- Consumer bindings freeze the reopen context and reopen command id that incident, support, and proof consumers use to reopen the same graph slice later.
- Handoff lineage rows carry the stable target ref, handoff reason, return surface, return-anchor ref, and an explicit control-plane-boundary disclosure.

## Required Drill Coverage

Every claimed infrastructure family must carry all five drill classes:

- `wrong_target`
- `stale_live_overlay`
- `missing_permission`
- `connector_skew`
- `locality_mismatch`

The proof packet must retain every drill row with its explicit resolution state instead of collapsing failures into a generic infrastructure error.

## Fixture Meaning

- `qualified_parity_packet.json` proves incident/support/proof parity across Terraform, Kubernetes, devcontainer, CI, and policy relation graphs.
- `missing_connector_skew_drill_packet.json` intentionally fails validation by dropping one required drill class from the Terraform family.
- `permission_limited_binding_dropped_packet.json` intentionally fails validation by stripping permission-limited preservation from the policy support path.

## Support Export Posture

Support exports may include packet ids, object ids, relation ids, truth-layer names, freshness labels, overlay posture, connector-skew labels, locality mismatch labels, handoff refs, return-anchor refs, reopen refs, and redaction-safe summaries. They must not include raw provider payloads, raw credential material, raw kubeconfig or cloud profile bodies, browser cookies, or hidden mutate instructions.
