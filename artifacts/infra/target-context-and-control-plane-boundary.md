# Target-Context and Control-Plane Boundary Qualification Packet

Generated evidence for the current infra boundary lane is checked in as source-controlled fixtures rather than produced by live connectors.

## Evidence

- Schema: `schemas/infra/environment-context-and-action-safety.schema.json`
- Validator: `crates/aureline-infra::target_context_and_control_plane_boundary`
- Passing parity fixture: `fixtures/infra/target-context-and-control-plane-boundary/qualified_context_parity_packet.json`
- Stale-overlay downgrade fixture: `fixtures/infra/target-context-and-control-plane-boundary/stale_live_overlay_downgraded_packet.json`
- Wrong-target block fixture: `fixtures/infra/target-context-and-control-plane-boundary/wrong_target_action_blocked_packet.json`

## Claimed Posture

The checked-in packet qualifies the evidence model and downgrade behavior only. It does not claim broad live Kubernetes, cloud, or incident-workspace implementation depth.

Stable claims require all target-context chip groups to agree across terminal, logs, resource graph, incident workspace, AI action sheet, CLI JSON, browser handoff, and support export. Stale, permission-limited, unavailable, or wrong-target rows remain inspect-only, file-only, handoff-only, or blocked.
