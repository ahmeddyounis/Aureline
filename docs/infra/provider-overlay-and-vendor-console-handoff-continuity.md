# Provider-Overlay and Vendor-Console Handoff Continuity

This document defines the checked-in packet that keeps provider-overlay truth and vendor-console handoffs explicit across infrastructure-adjacent surfaces. It composes the shared [source-intelligence object packet](./source-intelligence-and-resource-relationships.md) with the [target-context and control-plane boundary](./target-context-and-control-plane-boundary.md) packet so code, incident, preview, route, and infrastructure surfaces all disclose the same overlay truth, handoff reason, authority boundary, and return path.

The canonical machine-readable schema is [`/schemas/infra/provider-overlay-and-vendor-console-handoff-continuity.schema.json`](../../schemas/infra/provider-overlay-and-vendor-console-handoff-continuity.schema.json). The Rust validation model is in [`/crates/aureline-infra`](../../crates/aureline-infra/src/provider_overlay_and_vendor_console_handoff_continuity/mod.rs). Fixtures live in [`/fixtures/infra/provider-overlay-and-vendor-console-handoff-continuity`](../../fixtures/infra/provider-overlay-and-vendor-console-handoff-continuity).

## Qualification Rule

A promotable overlay or console-handoff surface needs a current packet proving all of the following:

- every provider-overlay row names the canonical object and canonical truth layer it enriches instead of flattening overlay truth into the same state badge;
- every overlay row binds to an explicit handoff reason and a known provider-handoff object rather than assuming a generic browser escape;
- every handoff preserves destination class, target identity, authority-boundary disclosure, structured return anchor, and at least two return-safe breadcrumbs;
- code breadcrumbs, incident workspace, preview route, route explorer, and infrastructure panel bindings all consume the same packet and keep overlay badges, canonical truth reference, target identity, control-plane boundary, handoff reason, return anchor, and breadcrumbs visible;
- returning from a provider page rehydrates the same target context and surface anchor instead of dropping the user into a generic reopened shell.

Packets that fail any error-severity check are not promotable; affected surfaces must narrow to inspect-only or handoff-only posture with the gap made explicit.

## Fixture Meaning

- `qualified_overlay_handoff_packet.json` proves explicit overlay disclosure and return-safe handoff continuity across code, incident, preview, route, and infrastructure surfaces for a Kubernetes checkout target.
- `blurred_overlay_truth_packet.json` intentionally fails validation by hiding canonical truth behind a provider overlay row.
- `generic_shell_return_packet.json` intentionally fails validation by allowing the handoff return anchor to fall back to a generic shell instead of rehydrating the same target surface.

## Support Export Posture

Support exports may include packet ids, overlay row ids, canonical object refs, relation refs, handoff reasons, destination classes, target identity snapshots, boundary labels, return-anchor ids, breadcrumb labels, and redaction-safe summaries. They must not include raw provider payloads, raw credentials, browser cookies, private endpoint URLs, or console-only mutation instructions.
