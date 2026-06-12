# Relation-Graph Incident/Support Parity Packet

Generated evidence for infrastructure relation-graph parity is checked in as source-controlled fixtures and a support-export-safe packet rather than produced by live Terraform, Kubernetes, devcontainer, CI, or policy connectors.

## Evidence

- Schema: `schemas/infra/relation-graph-incident-support-parity.schema.json`
- Validator: `crates/aureline-infra::relation_graph_incident_support_parity`
- Passing parity fixture: `fixtures/infra/relation-graph-incident-support-parity/qualified_parity_packet.json`
- Missing-drill fixture: `fixtures/infra/relation-graph-incident-support-parity/missing_connector_skew_drill_packet.json`
- Permission-preservation drill fixture: `fixtures/infra/relation-graph-incident-support-parity/permission_limited_binding_dropped_packet.json`
- Support-export artifact: `artifacts/infra/relation-graph-incident-support-parity-support-export.json`

## Claimed Posture

The checked-in packet proves that incident packets, support exports, and proof corpora can reopen the same infrastructure relation graph without collapsing target identity, truth-layer mix, stale-live overlay posture, connector-skew posture, local/remote/managed mismatch labels, or control-plane handoff lineage.

The packet does not claim generic cloud-console replacement or hidden live mutation parity. Provider-owned follow-up remains an explicit control-plane handoff with a stable return anchor.
