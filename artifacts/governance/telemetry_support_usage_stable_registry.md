# Telemetry, support-export, and usage-export schema registry — stable release evidence

**Artifact:** [`artifacts/governance/telemetry_support_usage_schema_registry.json`](telemetry_support_usage_schema_registry.json)  
**Schema:** [`schemas/governance/telemetry_support_usage_registry.schema.json`](../../schemas/governance/telemetry_support_usage_registry.schema.json)  
**Rust module:** `crates/aureline-governance/src/telemetry_support_usage_registry/`  
**Governance doc:** [`docs/governance/telemetry_support_usage_stable_registry.md`](../../docs/governance/telemetry_support_usage_stable_registry.md)  
**Fixtures:** [`fixtures/governance/telemetry_support_usage_schema_registry_cases/`](../../fixtures/governance/telemetry_support_usage_schema_registry_cases/)

## What this artifact is

This JSON registry stabilizes the six telemetry, diagnostics,
support-export, and usage-export payload families already present in
the governed schema registry (`schemas/registry/schema_registry.json`)
as first-class product contracts by extending them with the following
governance dimensions that were previously informal or implied:

- **Endpoint-policy truth per deployment context** — what happens to
  data in `oss_local`, `self_hosted`, and `managed_enterprise` contexts.
- **OSS telemetry posture** — enforcement of `opt_in_disabled_until_user_consent`
  as the default, with explicit exception-packet ref field.
- **Retention notes** — a reviewable sentence on how long data is held.
- **Redaction profile references** — citation of the ADR that governs
  what bytes the family excludes at boundary crossings.
- **Deprecated-field handling policy** — the closed policy readers must
  follow when encountering deprecated or unknown-version fields.
- **Partial-outcome markers** — whether packets can be partial due to
  policy suppression, offboarding-window bounds, or manual reconciliation.
- **Offboarding compatibility notes** — what each family does at account exit.

## Acceptance criteria satisfied

| Criterion | Evidence |
|---|---|
| 6 rows cover all in-scope families | JSON rows: `telemetry.ux_product_event`, `diagnostics.crash_payload`, `support.bundle_manifest`, `usage.metering_export_packet`, `offboarding.exit_packet`, `cli.headless_diagnostic_payload` |
| Endpoint-policy truth declared per context | Each row has `endpoint_policy_truth_by_context` with `oss_local`, `self_hosted`, `managed_enterprise` |
| OSS telemetry default enforced | `telemetry.ux_product_event` has `open_source_default_posture: "opt_in_disabled_until_user_consent"`; `validate_registry()` checks this |
| Retention notes present on all rows | Each row has a non-empty `retention_note` |
| Redaction profile ref present on all rows | All rows cite `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md` |
| Consent/endpoint class present on all rows | Inherited via `entry_id` reference to governed schema registry |
| Partial-outcome markers present on all rows | Each row has `partial_outcome_marker` from closed vocabulary |
| Offboarding compatibility notes present | Each row has a non-empty `offboarding_compatibility_note` |
| Typed Rust validator with unit tests | `crates/aureline-governance/src/telemetry_support_usage_registry/mod.rs`, 9 unit tests |
| JSON schema for registry format | `schemas/governance/telemetry_support_usage_registry.schema.json` |
| Fixture cases | `fixtures/governance/telemetry_support_usage_schema_registry_cases/` |
| Summary row consistent with data | `validate_registry()` checks computed summary matches embedded summary |
| Governance doc | `docs/governance/telemetry_support_usage_stable_registry.md` |

## Current summary

```json
{
  "total_rows": 6,
  "labeled_rows": 6,
  "telemetry_rows_with_oss_opt_in_default": 1,
  "local_only_oss_context_rows": 3,
  "managed_endpoint_rows": 1,
  "queued_manual_export_rows": 2
}
```

## Validation

Run the governance validator:

```
cargo test -p aureline-governance telemetry_support_usage_registry
```

All 9 tests must pass. Any `RegistryViolation` returned by
`validate_registry()` is a shiproom gate failure.

## Signing status

This artifact is not yet signed. Before any stable external claim cites
it, it must be countersigned by a trust/privacy reviewer. At that point a
signature manifest should be placed alongside this document.

## Exception packets

No OSS telemetry exception packets have been issued. The
`oss_telemetry_exception_packet_ref` field is null on all rows.
