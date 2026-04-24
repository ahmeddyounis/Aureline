# Host-negotiation example fixtures

These fixtures anchor the vocabulary reserved in
[`/docs/adr/0019-wasm-wit-extension-host-and-capability-worlds.md`](../../../docs/adr/0019-wasm-wit-extension-host-and-capability-worlds.md)
and validated by the seed schema at
[`/schemas/extensions/host_negotiation.schema.json`](../../../schemas/extensions/host_negotiation.schema.json).

The ADR is a seed at `Status: Proposed`. These fixtures exercise
the reserved field sets and enumerated vocabularies so the later
extension-runtime, SDK, install-review, permission-inspector,
registry-mirror, and compatibility-bridge lanes can be built
against one contract rather than invent extension-host-shaped
fields ad hoc.

**Scope rules**

- Each fixture validates against
  `schemas/extensions/host_negotiation.schema.json` as one of
  `capability_world_row`, `host_negotiation_packet`,
  `world_admission_decision_record`, or
  `compatibility_bridge_profile_row`.
- A fixture MUST exercise at least one frozen `world_identity`,
  `host_contract_family`, `permission_scope_kind`, budget class,
  `narrowing_reason_kind`, `unsupported_world_reason_kind`,
  `host_negotiation_audit_event_id`, or
  `host_negotiation_denial_reason` and MUST name the ADR section
  that motivates it.
- Raw Wasm component bytes, raw core module bytes, raw external-
  host launch bodies, raw helper-binary invocation bodies, raw
  bridge-shim payloads, raw signing-key material, and raw policy-
  bundle bytes MUST NOT appear; refs stand in for every field that
  would otherwise carry raw material.
- Ids, refs, aliases, and monotonic timestamps are opaque; they
  are chosen to read well rather than to reflect any real
  deployment.

**Index**

| Fixture                                                                      | Record kinds exercised                                                                                 | Key classes exercised                                                                                                                                                                        | ADR section                                                 |
|------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------|
| [`declared_vs_negotiated_example.yaml`](./declared_vs_negotiated_example.yaml) | `host_negotiation_packet`, `world_admission_decision_record`, `compatibility_bridge_profile_row` | `wasm_component_model` / `editor-read` + `workspace-read` + `diff-apply-preview` + `terminal-observe` + `network-egress` / `workspace_trust_restricted` + `admin_policy_egress_host_narrowing` / `host_negotiation_opened` + `host_negotiation_worlds_narrowed` + `host_negotiation_completed` | Host-negotiation packet; Narrowing reasons; Unsupported-world behavior |
