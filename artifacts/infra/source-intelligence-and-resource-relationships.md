# Source-Intelligence and Resource-Relationship Matrix Packet

Generated evidence for the current M5 infrastructure source-intelligence lane is checked in as source-controlled fixtures rather than produced by live Terraform, Kubernetes, container, CI, or policy connectors.

## Evidence

- Matrix schema: `schemas/infra/source-intelligence-and-resource-relationships.schema.json`
- Object schema: `schemas/infra/source-intelligence-object-packet.schema.json`
- Parity schema: `schemas/infra/relation-graph-incident-support-parity.schema.json`
- Validator: `crates/aureline-infra::source_intelligence_and_resource_relationships`
- Passing parity fixture: `fixtures/infra/source-intelligence-and-resource-relationships/qualified_matrix_packet.json`
- Explicit downgrade fixture: `fixtures/infra/source-intelligence-and-resource-relationships/file_only_downgraded_matrix_packet.json`
- Missing-layer drill fixture: `fixtures/infra/source-intelligence-and-resource-relationships/missing_truth_layer_and_profile_packet.json`
- Passing object fixture: `fixtures/infra/source-intelligence-and-resource-relationships/qualified_object_packet.json`
- Missing-lineage object drill fixture: `fixtures/infra/source-intelligence-and-resource-relationships/missing_rendered_lineage_object_packet.json`
- Relation-graph parity fixture: `fixtures/infra/relation-graph-incident-support-parity/qualified_parity_packet.json`
- Relation-graph parity support export: `artifacts/infra/relation-graph-incident-support-parity-support-export.json`

## Claimed Posture

The checked-in packet qualifies the canonical vocabulary only. It does not claim generic cloud-console replacement or full in-product mutation parity.

Stable claims require every covered infrastructure family to publish the same five truth layers, to reuse the shared relation-edge classes, and to bind each layer to target-context requirements, live-access prerequisites, export fidelity, console-handoff posture, and explicit file-only, inspect-only, and handoff-only fallback behavior.

The concrete object packet extends that vocabulary into stable object ids, lineage-carrying rendered/planned records, live and overlay records with explicit authority posture, and shared graph/review/docs/incident projections that resolve by object and relation id rather than bespoke parsers or hidden side stores.

The parity packet extends the same graph into reopenable incident/support/proof evidence with explicit wrong-target, stale-overlay, permission-limited, connector-skew, and local/remote/managed mismatch drill coverage.
