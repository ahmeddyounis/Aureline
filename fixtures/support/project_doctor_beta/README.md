# Project Doctor beta probe-pack catalog and finding fixtures

Each fixture mirrors the boundary schema at
[`/schemas/support/project_doctor.schema.json`](../../../schemas/support/project_doctor.schema.json).

| File | Record kind | Purpose |
|---|---|---|
| `catalog.yaml` | `project_doctor_probe_pack_catalog_record` | versioned probe-pack catalog for the beta lane |
| `finding_entry_target_unavailable.yaml` | `project_doctor_finding_record` | typed entry-readiness finding emitted under cli_headless |
| `finding_toolchain_missing_component.yaml` | `project_doctor_finding_record` | typed toolchain finding with preview-only recovery handoff |
| `finding_provider_credential_expired.yaml` | `project_doctor_finding_record` | typed provider-auth finding restricted to support_guided |
| `finding_support_bundle_integrity_unsupported.yaml` | `project_doctor_finding_record` | typed unsupported finding offline_local that refuses unsafe diagnosis |

These fixtures are the canonical replay set for
[`crates/aureline-doctor/tests/project_doctor_beta.rs`](../../../crates/aureline-doctor/tests/project_doctor_beta.rs)
and the support-export consumer in
[`crates/aureline-support/src/project_doctor/mod.rs`](../../../crates/aureline-support/src/project_doctor/mod.rs).

Every finding carries a `probe_pack_ref` attribution back to the catalog
entry it was emitted from, a confidence class, and a redaction-safe
summary. Every catalog entry declares a frozen `pack_version`, a
`read_only_posture`, and explicit headless and support-guided admission
classes, so the same finding packet renders identically across UI, CLI,
support export, and headless JSON.
