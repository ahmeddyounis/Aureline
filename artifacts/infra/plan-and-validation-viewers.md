# Plan and Validation Viewer Packet

Generated evidence for the current M5 infrastructure viewer lane is checked in as source-controlled fixtures rather than produced by live Terraform, Kubernetes, policy, or CI connectors.

## Evidence

- Schema: `schemas/infra/plan-and-validation-viewers.schema.json`
- Validator: `crates/aureline-infra::plan_and_validation_viewers`
- Passing fixture: `fixtures/infra/plan-and-validation-viewers/qualified_viewer_packet.json`
- Hidden-authority drill fixture: `fixtures/infra/plan-and-validation-viewers/hidden_live_authority_packet.json`
- Missing-identity/review drill fixture: `fixtures/infra/plan-and-validation-viewers/missing_tool_identity_and_review_gate_packet.json`

## Claimed Posture

The checked-in packet qualifies viewer truth only. It does not claim generic cloud-console replacement or implicit in-product apply authority.

Stable claims require every covered viewer class to stay labeled as planned/validated truth, to carry exact target context plus tool identity/version and timestamp, to block hidden live-authority inheritance from static inspection, and to preserve review, incident, and support-export attribution for later handoff or repair work.
