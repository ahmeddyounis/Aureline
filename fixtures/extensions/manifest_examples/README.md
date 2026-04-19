# Extension manifest example fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary reserved in
[`/docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../../../docs/adr/0012-extension-manifest-permission-publisher-policy.md)
and validated by the seed schema at
[`/schemas/extensions/effective_permission.schema.json`](../../../schemas/extensions/effective_permission.schema.json).

The ADR is a seed at `Status: Proposed`. These fixtures exercise
the reserved field sets and enumerated vocabularies so the later
install-review, permission-inspector, publisher-continuity, and
mirror-continuity lanes can be built against one contract rather
than invent extension-shaped fields ad hoc.

**Scope rules**

- Each fixture validates against
  `schemas/extensions/effective_permission.schema.json` as one of
  `extension_manifest_row`,
  `effective_permission_summary_record`,
  `publisher_continuity_row`, or
  `policy_pack_constraint_row`.
- A fixture MUST exercise at least one frozen
  `host_contract_family`, `artifact_transport_family`,
  `permission_scope_kind`, `publisher_lineage_state`,
  `publisher_trust_tier`, `policy_pack_constraint_kind`,
  `audit_event_id`, or `denial_reason` and MUST name the ADR
  section that motivates it.
- Raw artifact bytes, raw signing-key material, raw policy-bundle
  bytes, and raw publisher-private data MUST NOT appear; refs
  stand in for every field that would otherwise carry raw
  material.
- Ids, refs, aliases, and monotonic timestamps are opaque; they
  are chosen to read well rather than to reflect any real
  deployment.

**Index**

| Fixture                                                                      | Record kinds exercised                                                                                              | Key classes exercised                                                                                                                           | ADR section                                                                 |
|------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------|
| [`declared_vs_effective_example.yaml`](./declared_vs_effective_example.yaml) | `extension_manifest_row`, `effective_permission_summary_record`, `publisher_continuity_row`, `policy_pack_constraint_row` | `wasm_component_model` / `wasm_signed_artifact` / `filesystem_read` + `filesystem_write` + `network_egress` + `ai_provider_access` + `capability_inherit` / `succeeded` + `verified_publisher` / `permission_floor` + `egress_host_narrowing` / `effective_permission_narrowed_by_policy_pack` | Reserved manifest fields; Reserved effective-permission summary fields; Reserved publisher-continuity fields; Reserved policy-pack constraint fields |
