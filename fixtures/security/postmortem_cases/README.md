# Postmortem and compensating-control fixtures

These fixtures anchor the postmortem and compensating-control contract
frozen in:

- `/docs/security/postmortem_and_compensating_control_contract.md`
- `/schemas/security/postmortem_record.schema.json`
- `/schemas/security/compensating_control_row.schema.json`

Fixtures use opaque refs, typed vocabulary, and export-safe summaries.
Raw evidence bodies, raw secrets, raw signing material, raw URLs or
hostnames, absolute paths, and internal identity strings do not appear.

| Fixture | Record kind | What it proves |
|---|---|---|
| [`critical_advisory_disable_bundle_postmortem.yaml`](./critical_advisory_disable_bundle_postmortem.yaml) | `postmortem_record` | critical incident mitigated via disable bundle + compensating-control review horizon |
| [`high_severity_fixed_release_postmortem.yaml`](./high_severity_fixed_release_postmortem.yaml) | `postmortem_record` | high-severity incident resolved by fixed release/build identities |
| [`long_lived_compensating_control_pending_remediation.yaml`](./long_lived_compensating_control_pending_remediation.yaml) | `postmortem_record` | long-lived compensating control with explicit must-review-by horizon and follow-up ownership |

