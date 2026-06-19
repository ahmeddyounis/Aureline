# Artifact: locality descriptors and tenant-boundary cards

**Contract ref:** `continuity:m5_locality_descriptors_and_tenant_cards:v1`  
**Schema:** `schemas/continuity/locality_descriptor.schema.json`  
**Doc:** `docs/m5/continuity/locality-and-tenant-boundary-surfaces.md`  
**Runtime owner:** `aureline_continuity::m5_locality_descriptors_and_tenant_cards`

## Qualification

| Condition | Status |
|---|---|
| Processing + storage location disclosed on every row | ✓ Stable |
| Retention/export class disclosed on every row | ✓ Stable |
| Region pin declared, named, and honored on managed-scope rows | ✓ Stable |
| Tenant scope disclosed + boundary isolation verified | ✓ Stable |
| No self-hosted/sovereign locality overclaim | ✓ Stable |
| Every required surface projected | ✓ Stable |
| Locality + tenant vocabulary identical across surfaces | ✓ Stable |
| **Overall** | **Stable** |

## Locality descriptors

| Surface | Profile | Processing | Storage | Region pin | Honor | Retention |
|---|---|---|---|---|---|---|
| Managed cloud workspace sync and backup | `managed` | `single_region` | `single_region` | `single_region_pinned` | `honored` | `vendor_default_retention` |
| Managed relay and collaboration failover | `managed` | `multi_region` | `multi_region` | `multi_region_pinned` | `honored` | `vendor_default_retention` |
| Customer self-hosted restore and rebuild | `self_hosted` | `customer_region` | `customer_region` | `customer_region_pinned` | `honored` | `customer_configured_retention` |
| Sovereign air-gapped snapshot and replication | `sovereign` | `in_country_sovereign` | `air_gapped_isolated` | `in_country_pinned` | `honored` | `legal_hold_retention` |
| Local desktop core continuity | `local_only` | `device_local` | `device_local` | `pin_not_applicable` | `not_applicable` | `device_local_retention` |

## Tenant-boundary cards

| Surface | Tenant scope | Isolation | Key mode | Boundary verified |
|---|---|---|---|---|
| Managed cloud workspace sync and backup | `shared_multi_tenant` | `logical_multi_tenant` | `vendor_managed_keys` | yes |
| Managed relay and collaboration failover | `dedicated_tenant` | `dedicated_infrastructure` | `vendor_managed_keys` | yes |
| Customer self-hosted restore and rebuild | `customer_tenant` | `customer_boundary` | `customer_managed_keys` | yes |
| Sovereign air-gapped snapshot and replication | `customer_tenant` | `customer_boundary` | `customer_held_root` | yes |
| Local desktop core continuity | `single_user_local` | `process_local_isolation` | `local_os_keystore` | yes |

## Surface projection

Each row is projected onto every surface it must reach with byte-identical
locality and tenant summary lines: managed rows reach `desktop`, `cli_headless`,
`service_health`, `support_export`, `about_help`, and `docs_public_truth`;
local-core rows reach all but `support_export`.

## Claim-narrowing cases

Each case mutates one seeded row and shows the claim narrowing automatically:

- `case_region_pin_unhonored_withdrawn` — managed region pin cannot be honored,
  fails closed → **withdrawn** (`region_pin_unhonored`)
- `case_region_pin_undeclared_preview` — managed row declares no region pin →
  **preview** (`region_pin_undeclared_on_managed`)
- `case_self_hosted_locality_overclaimed_withdrawn` — self-hosted row claims a
  broad vendor region → **withdrawn** (`self_hosted_locality_overclaimed`)
- `case_retention_undisclosed_beta` — managed relay row hides its retention class
  → **beta** (`retention_class_undisclosed`)
- `case_tenant_boundary_unverified_preview` — managed relay row cannot verify its
  tenant boundary → **preview** (`tenant_boundary_unverified`)
- `case_surface_projection_incomplete_beta` — managed row not projected onto the
  support-export surface → **beta** (`surface_projection_incomplete`)

## Fixture references

- `fixtures/continuity/locality_tenant_cases/page.json`
- `fixtures/continuity/locality_tenant_cases/summary.json`
- `fixtures/continuity/locality_tenant_cases/support_export.json`
- `fixtures/continuity/locality_tenant_cases/case_region_pin_unhonored_withdrawn.json`
- `fixtures/continuity/locality_tenant_cases/case_region_pin_undeclared_preview.json`
- `fixtures/continuity/locality_tenant_cases/case_self_hosted_locality_overclaimed_withdrawn.json`
- `fixtures/continuity/locality_tenant_cases/case_retention_undisclosed_beta.json`
- `fixtures/continuity/locality_tenant_cases/case_tenant_boundary_unverified_preview.json`
- `fixtures/continuity/locality_tenant_cases/case_surface_projection_incomplete_beta.json`
