# Hardened Identity and Admin Support-Export Parity — Fixtures

Generated from `aureline_policy::harden_identity_and_admin_support_export_parity_audit`.

## Files

- `page.json` — Seeded proof packet (stable, zero defects).
- `rows.json` — The five required rows.
- `summary.json` — Aggregate summary.
- `defects.json` — Empty defect list for the seeded page.
- `support_export.json` — Support-export envelope.
- `drill_raw_secret_withdrawn.json` — Withdrawn drill (raw secret material exposed).
- `drill_missing_row_class_preview.json` — Preview drill (missing required row classes).
- `drill_empty_provisioning_class_beta.json` — Beta drill (empty provisioning class).
- `drill_generic_failure_kind_beta.json` — Beta drill (generic failure kind).

## Regenerate

```sh
cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- page > fixtures/enterprise/m4/harden-identity-and-admin-support-export-parity-audit/page.json
```
