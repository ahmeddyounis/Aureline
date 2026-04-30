# Audit-event explorer fixtures

Worked YAML cases for the audit-event explorer contract.

- Contract: [`/docs/admin/audit_event_explorer_contract.md`](../../../docs/admin/audit_event_explorer_contract.md)
- Row schema: [`/schemas/admin/audit_event_record.schema.json`](../../../schemas/admin/audit_event_record.schema.json)
- Filter, completeness, and export schema:
  [`/schemas/admin/audit_event_filter.schema.json`](../../../schemas/admin/audit_event_filter.schema.json)

Each YAML file is a single boundary record validated by the cited
schema. Records carry opaque refs and reviewable summaries; raw
policy bundles, raw signing material, raw issuer URLs, raw SCIM
endpoints, raw mirror hostnames, raw user identifiers, raw email or
display names, raw subject claims, raw provider payloads, raw IP
addresses, raw device fingerprints, raw paths, raw tokens, raw
command lines, and raw secret material do not appear.

The cases collectively show that a reviewer can answer who, what,
where, when, and outcome for each material event without reading raw
logs, that the explorer language matches the policy center and admin
handoff vocabulary, and that an enterprise reviewer can reconstruct
a policy, entitlement, or deprovision event from the export packet
alone.

See `manifest.yaml` for the full case index and the contract sections
each case exercises.
