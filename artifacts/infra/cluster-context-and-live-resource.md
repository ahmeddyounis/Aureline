# Cluster-Context and Live-Resource Qualification Packet

Generated evidence for the cluster-context and live-resource lane is checked in as source-controlled fixtures rather than produced by live Terraform, Kubernetes, or incident connectors.

## Evidence

- Schema: `schemas/infra/cluster-context-and-live-resource.schema.json`
- Validator: `crates/aureline-infra::cluster_context_and_live_resource`
- Passing parity fixture: `fixtures/infra/cluster-context-and-live-resource/qualified_cluster_context_packet.json`
- Stale-downgrade fixture: `fixtures/infra/cluster-context-and-live-resource/stale_live_downgraded_packet.json`
- Wrong-target / blended-view fixture: `fixtures/infra/cluster-context-and-live-resource/wrong_target_blended_view_packet.json`

## Claimed Posture

The checked-in packet qualifies the context-strip, truth-mode, action-gate, and console-handoff evidence model only. It does not claim broad live Terraform, Kubernetes, or incident-workspace control-plane parity.

Stable claims require every Terraform, Kubernetes, and incident-adjacent surface to render the same context strip and to keep desired, rendered, plan, live, and provider-overlay state as separate truth modes with explicit freshness and source labels. Mutating or boundary-raising actions must preview the exact target and source-of-truth posture before execution or console handoff, and provider consoles remain explicit, non-authoritative handoff destinations.
