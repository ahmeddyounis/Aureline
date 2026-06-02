# Open-vs-paid boundary manifest, managed-offering truth, usage export, and offboarding packet

## Status

Seeded. Schema version 1. Contract ref `policy:open_vs_paid_boundary:v1`.

## What this artifact proves

This artifact is the machine-readable evidence that the shipped build can show:

- Which capabilities are open-local.
- Which capabilities are managed or enterprise add-ons.
- What usage/export records exist for managed capabilities.
- What survives cancellation or deprovisioning without forcing support-only
  interpretation.

## Seeded stable claim

The seeded page (`policy:open_vs_paid_boundary:default`) qualifies **stable**
with zero defects. All fifteen capability families are covered:

- Seven local-core capabilities are `open_local`.
- Five managed capabilities are `managed_hosted` with offboarding and usage-export
  disclosures.
- Three enterprise capabilities are `enterprise_governed` with offboarding and
  usage-export disclosures.

## Hard guardrails

- A local-core capability classified as `managed_hosted` or `enterprise_governed`
  **withdraws** the packet immediately.
- Missing offboarding or usage-export disclosures for managed capabilities
  narrows to **beta**.
- Stale usage-export schema version narrows to **beta**.

## Companion files

- Doc: `docs/policy/m4/finalize-open-vs-paid-boundary-and-offboarding.md`
- Schema: `schemas/policy/usage-export.schema.json`
- Fixtures (policy): `fixtures/policy/m4/finalize-open-vs-paid-boundary-and-offboarding/`
- Fixtures (offboarding): `fixtures/offboarding/`
- Runtime module: `crates/aureline-policy/src/finalize_open_vs_paid_boundary_and_offboarding/`

## Fixture inventory

### Generated JSON fixtures

| Fixture | Description |
|---------|-------------|
| `page.json` | Full stable boundary page |
| `rows.json` | All 15 capability boundary rows |
| `defects.json` | Empty defect list (stable) |
| `summary.json` | Aggregate summary |
| `support_export.json` | Support-export wrapper |
| `usage_export_packets.json` | Usage-export packets for managed capabilities |
| `offboarding_packets.json` | Offboarding packets for managed capabilities |

### Drill fixtures

| Fixture | Scenario |
|---------|----------|
| `drill_local_core_managed_withdrawn.json` | `editor_core` classified as `managed_hosted` (withdrawn) |
| `drill_missing_offboarding_beta.json` | `collaboration` missing offboarding disclosure (beta) |
| `drill_stale_export_schema_beta.json` | Stale usage-export schema version (beta) |

### YAML input fixtures

| Fixture | Scenario |
|---------|----------|
| `all_capabilities_stable_input.yaml` | Clean stable input with all 15 capabilities |
| `local_core_managed_withdrawn_input.yaml` | Hard guardrail violation |
| `missing_offboarding_beta_input.yaml` | Missing offboarding disclosure |
| `stale_export_schema_beta_input.yaml` | Stale schema version |

## Verification

```sh
cargo test -p aureline-policy -- finalize_open_vs_paid_boundary
cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- page | jq '.summary.overall_qualification_token'
```

Expected output: `"stable"`
