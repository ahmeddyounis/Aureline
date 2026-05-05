# Admin audit export fixtures

Worked YAML cases for the admin audit export contract.

- Contract: [`/docs/admin/admin_audit_export_contract.md`](../../../docs/admin/admin_audit_export_contract.md)
- Schema: [`/schemas/admin/admin_audit_export.schema.json`](../../../schemas/admin/admin_audit_export.schema.json)
- Minimum field set:
  [`/artifacts/admin/admin_export_minimum_fields.yaml`](../../../artifacts/admin/admin_export_minimum_fields.yaml)

Each YAML file is a single boundary record validated by the cited
schema. Records carry opaque refs and reviewable summaries; raw policy
rule bodies, raw bundle bytes, raw signing material, raw issuer URLs,
raw SCIM endpoints, raw mirror hostnames, raw IP addresses, raw device
fingerprints, raw user identifiers, raw email or display names, raw
subject claims, raw provider payloads, raw paths, raw tokens, raw
command lines, and raw secret material do not appear.

See `manifest.yaml` for the full case index.

