# Infrastructure Source-Intelligence and Resource Relationships

This document defines the canonical M5 matrix for infrastructure-aware source intelligence. It freezes one shared vocabulary for Terraform/HCL, Kubernetes/Helm, devcontainer, CI/environment, and policy-manifest families so later infra, review, search, docs, AI, incident, and support surfaces reuse the same truth-layer and relation-edge model.

The canonical machine-readable schema is [`/schemas/infra/source-intelligence-and-resource-relationships.schema.json`](../../schemas/infra/source-intelligence-and-resource-relationships.schema.json). The Rust validation model is in [`/crates/aureline-infra`](../../crates/aureline-infra/src/source_intelligence_and_resource_relationships/mod.rs). Fixtures live in [`/fixtures/infra/source-intelligence-and-resource-relationships`](../../fixtures/infra/source-intelligence-and-resource-relationships).

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

## Support Export Posture

Support exports may include packet ids, family ids, truth-layer names, relation-edge names, target-context requirement classes, required context fields, live-access prerequisites, console-handoff posture, export-fidelity class, downgrade posture, and redaction-safe summaries. They must not include raw provider payloads, raw credential material, raw kubeconfig or cloud profile bodies, private endpoint URLs, browser cookies, or hidden mutation instructions.

## Predecessor Notes

The earlier [`/docs/devops/resource_relationship_matrix.md`](../devops/resource_relationship_matrix.md) document remains useful background, but this infra packet is the canonical M5 source of truth for later implementation rows.
