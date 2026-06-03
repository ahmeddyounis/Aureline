# Fixture cases: telemetry, support-export, and usage-export schema registry

This directory contains worked fixture cases for the stable telemetry,
diagnostics, support-export, and usage-export schema registry defined in
[`artifacts/governance/telemetry_support_usage_schema_registry.json`](../../../artifacts/governance/telemetry_support_usage_schema_registry.json).

## Purpose

Each fixture demonstrates a specific governance property enforced by
the registry format and the `validate_registry()` gate in the Rust module
at `crates/aureline-governance/src/telemetry_support_usage_registry/`.

Fixtures are human-readable YAML documents, not executable test cases.
The executable tests live in the Rust module's `#[cfg(test)]` block.
These fixtures serve as:

- **Design rationale** — they record *why* a row has the values it has,
  not just *what* those values are.
- **Review surface** — privacy/trust/governance reviewers can read these
  without reading Rust code.
- **Regression guard** — if a row changes and a fixture no longer matches,
  that is a signal that the change needs explicit sign-off.

## Cases

| File | Entry ID | Property |
|---|---|---|
| `oss_telemetry_local_only_default.yaml` | `telemetry.ux_product_event` | OSS telemetry local by default; opt-in required |
| `support_export_queued_manual.yaml` | `support.bundle_manifest` | Support bundles queued for manual export in all contexts |
| `usage_export_managed_endpoint.yaml` | `usage.metering_export_packet` | Usage export only in managed-enterprise context |
| `diagnostics_local_only_oss.yaml` | `diagnostics.crash_payload` | Crash diagnostics local-only in all contexts |
| `unlabeled_row_promotion_blocked.yaml` | (synthetic) | Shiproom gate rejects unlabeled rows |

See `manifest.yaml` for the machine-readable index of all cases.

## How to read a fixture case

Each fixture has:

- **`case_id`**: Stable identifier for the case.
- **`description`**: Human-readable explanation of what is being demonstrated.
- **`registry_entry_ref`** or **`synthetic_row`**: The row under test.
- **`given`**: The preconditions.
- **`when`**: The query or action being tested.
- **`then`**: The expected outcomes.
- **`assertions`**: Field-level and invariant-level assertions.

## Governance doc

The companion governance document for this registry is
[`docs/governance/telemetry_support_usage_stable_registry.md`](../../../docs/governance/telemetry_support_usage_stable_registry.md).
