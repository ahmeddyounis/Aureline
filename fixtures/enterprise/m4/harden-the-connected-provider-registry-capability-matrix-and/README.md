# Fixtures: harden-the-connected-provider-registry-capability-matrix-and

These fixtures document the stable connected-provider registry hardening proof
packet. The canonical source of truth is the seeded packet produced by
`aureline_remote::harden_the_connected_provider_registry_capability_matrix_and::seeded_provider_registry_page()`.

## Files

| File | Content |
|------|---------|
| `page.json` | Full `ProviderRegistryPage` proof packet (stable, zero defects) |
| `rows.json` | Per-(family, actor) `ProviderRegistryRow` records (all 9 required pairs) |
| `defects.json` | Empty defect list (clean stable packet) |
| `summary.json` | `ProviderRegistrySummary` with counts and overall qualification |
| `support_export.json` | `ProviderRegistrySupportExport` envelope for support/diagnostics |
| `drills/drill_missing_pair_preview.json` | Drill: missing `code_host:human_account` pair narrows to `preview` |
| `drills/drill_raw_material_withdrawn.json` | Drill: raw private material on descriptor withdraws packet |
| `drills/drill_no_local_core_continuity_beta.json` | Drill: descriptor without local-core continuity narrows to `beta` |
| `drills/drill_empty_object_support_beta.json` | Drill: descriptor with empty object support narrows to `beta` |

## Required (family, actor) pairs

All nine required pairs must be present for a stable claim:

| Provider family | Actor identity |
|---|---|
| `code_host` | `human_account` |
| `code_host` | `installation_grant` |
| `code_host` | `delegated_credential` |
| `issue_tracker` | `human_account` |
| `issue_tracker` | `installation_grant` |
| `issue_tracker` | `delegated_credential` |
| `ci_checks` | `human_account` |
| `ci_checks` | `installation_grant` |
| `ci_checks` | `delegated_credential` |

## Schema

`schemas/enterprise/harden-the-connected-provider-registry-capability-matrix-and.schema.json`

## Contract ref

`remote:provider_registry_harden:v1`
