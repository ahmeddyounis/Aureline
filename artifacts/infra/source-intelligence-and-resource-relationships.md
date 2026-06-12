# Source-Intelligence and Resource-Relationship Matrix Packet

Generated evidence for the current M5 infrastructure source-intelligence lane is checked in as source-controlled fixtures rather than produced by live Terraform, Kubernetes, container, CI, or policy connectors.

## Evidence

- Schema: `schemas/infra/source-intelligence-and-resource-relationships.schema.json`
- Validator: `crates/aureline-infra::source_intelligence_and_resource_relationships`
- Passing parity fixture: `fixtures/infra/source-intelligence-and-resource-relationships/qualified_matrix_packet.json`
- Explicit downgrade fixture: `fixtures/infra/source-intelligence-and-resource-relationships/file_only_downgraded_matrix_packet.json`
- Missing-layer drill fixture: `fixtures/infra/source-intelligence-and-resource-relationships/missing_truth_layer_and_profile_packet.json`

## Claimed Posture

The checked-in packet qualifies the canonical vocabulary only. It does not claim generic cloud-console replacement or full in-product mutation parity.

Stable claims require every covered infrastructure family to publish the same five truth layers, to reuse the shared relation-edge classes, and to bind each layer to target-context requirements, live-access prerequisites, export fidelity, console-handoff posture, and explicit file-only, inspect-only, and handoff-only fallback behavior.
