# Infrastructure Plan and Validation Viewers

This document defines the checked-in packet required before Aureline can claim first-class plan, diff, dry-run, admission, or policy-check viewers on infrastructure-aware M5 surfaces.

The canonical machine-readable schema is [`/schemas/infra/plan-and-validation-viewers.schema.json`](../../schemas/infra/plan-and-validation-viewers.schema.json). The Rust validation model is in [`/crates/aureline-infra`](../../crates/aureline-infra/src/plan_and_validation_viewers/mod.rs). Fixtures live in [`/fixtures/infra/plan-and-validation-viewers`](../../fixtures/infra/plan-and-validation-viewers).

This packet extends the [target-context and control-plane boundary](./target-context-and-control-plane-boundary.md), [cluster-context and live-resource](./cluster-context-and-live-resource.md), and [source-intelligence and resource relationships](./source-intelligence-and-resource-relationships.md) contracts. Those packets govern stable target identity, object truth layers, and relationship edges; this packet governs the viewer outputs built from that shared vocabulary.

## Qualification Rule

A claimed viewer row is promotable only when all of the following are true:

- every plan, diff, dry-run, admission, and policy-check output is labeled as **planned / validated** truth rather than authored, observed, or provider-overlay state;
- each viewer preserves exact target context, capture timestamp, tool identity, and tool version;
- static file inspection, rendered output, or validation results do not silently inherit live mutate authority;
- any viewer that can lead to a mutate action requires explicit review-before-apply posture through a stable follow-up gate;
- review, incident, and support-export consumers preserve viewer ids, follow-up gate ids, target context, timestamps, tool identity, and any explicit handoff or later-repair breadcrumb;
- exports remain metadata-safe and do not embed raw provider payloads, credentials, kubeconfig bodies, or hidden mutation instructions.

Packets that fail any error-severity check are not promotable. The affected surface must narrow to inspect-only or handoff-only posture instead of implying in-product live authority.

## Viewer Coverage

- **Plan** viewers preserve change scope, target selectors, and the exact planner version that produced the result.
- **Diff** viewers preserve target context and change intent without collapsing into live state.
- **Dry-run** viewers stay simulation-only and do not authorize the corresponding apply.
- **Admission** viewers preserve denied or blocked results and keep any vendor-console repair path explicit as a handoff.
- **Policy-check** viewers preserve source revision, target environment, and validation provenance so rollout decisions stay reviewable.

## Fixture Meaning

- `qualified_viewer_packet.json` proves the full five-viewer packet with explicit target context, tool identity/version, review-before-apply gates, and review/incident/support joins.
- `hidden_live_authority_packet.json` intentionally fails validation by letting one viewer silently inherit live authority.
- `missing_tool_identity_and_review_gate_packet.json` intentionally fails validation by stripping tool identity and the review-before-apply requirement from a mutate-adjacent viewer.

## Support Export Posture

Support exports may include packet ids, viewer ids, viewer kinds, infrastructure families, target-context fields, tool identity/version, timestamps, gate ids, approval or ticket refs, handoff refs, and redaction-safe summaries. They must not include raw plan blobs, raw admission payloads, raw provider responses, raw credentials, or private endpoint secrets.
