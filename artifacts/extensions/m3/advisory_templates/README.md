# Extension advisory and emergency incident packets

This directory contains checked beta incident packets for extension
advisory, emergency-disable, quarantine, and revocation communication.

- `emergency_disable_incident_packet.json` is the canonical forced-disable
  packet for the sample extension.
- `emergency_disable_support_export.json` is the first consuming
  metadata-safe support/export projection.
- `mirror_quarantine_incident_packet.json` shows an approved-mirror
  quarantine with explicit primary and mirror trust state.
- `artifact_revocation_incident_packet.json` shows a primary-registry
  revocation that has propagated to the approved mirror.
- `security_advisory_notice_template.md` is the human-readable notice
  template that matches the machine-readable packet fields.

Refresh or inspect the generated packet records with:

```text
cargo run -q -p aureline-extensions --example dump_revocation_records
cargo run -q -p aureline-extensions --example dump_revocation_records -- incident primary_registry_emergency_disable
cargo run -q -p aureline-extensions --example dump_revocation_records -- support-export primary_registry_emergency_disable
```

The boundary schema is
`schemas/extensions/revocation_and_emergency_disable.schema.json`.
