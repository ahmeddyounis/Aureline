# Artifact: continuity-claim rows and drill schedule

**Contract ref:** `continuity:m5_locality_tenant_keymode_and_drill_matrix:v1`  
**Schema:** `schemas/continuity/m5-continuity-claim-row.schema.json`  
**Doc:** `docs/m5/continuity/locality_tenant_keymode_and_drill_matrix.md`  
**Runtime owner:** `aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix`

## Qualification

| Condition | Status |
|---|---|
| Locality + residency disclosed on every row | ✓ Stable |
| Tenant scope + key mode disclosed on managed-scope rows | ✓ Stable |
| Continuity packet family + ref named | ✓ Stable |
| Drill cadence + current/future owner named | ✓ Stable |
| Drill evidence current or reconstructable | ✓ Stable |
| Restore identity + partial loss disclosed | ✓ Stable |
| Control-plane vs data-plane impairment distinguished | ✓ Stable |
| Local-core vs managed-lane continuity distinguished | ✓ Stable |
| Surface fact reuse complete | ✓ Stable |
| **Overall** | **Stable** |

## Claim rows

| Surface | Profile | Lane | Locality (proc / store) | Tenant | Key mode | Degraded plane | Packet family | Restore identity | Partial loss |
|---|---|---|---|---|---|---|---|---|---|
| Managed cloud workspace sync and backup | `managed` | `managed_lane` | `single_region` / `single_region` | `shared_multi_tenant` | `vendor_managed_keys` | `control_plane_impairment` | `backup` | `same_identity_restore` | `bounded_recent_window_loss` |
| Managed relay and collaboration failover | `managed` | `managed_lane` | `multi_region` / `multi_region` | `dedicated_tenant` | `vendor_managed_keys` | `data_plane_impairment` | `failover` | `same_identity_restore` | `queued_action_loss` |
| Customer self-hosted restore and rebuild | `self_hosted` | `managed_lane` | `customer_region` / `customer_region` | `customer_tenant` | `customer_managed_keys` | `control_plane_impairment` | `restore` | `reissued_identity_restore` | `bounded_recent_window_loss` |
| Sovereign air-gapped snapshot and replication | `sovereign` | `managed_lane` | `in_country_sovereign` / `air_gapped_isolated` | `customer_tenant` | `customer_held_root` | `both_planes` | `snapshot_replication` | `new_install_rebind` | `cache_only_loss` |
| Local desktop core continuity | `local_only` | `local_core` | `device_local` / `device_local` | `single_user_local` | `local_os_keystore` | `data_plane_impairment` | `local_core_continuity` | `not_applicable` | `no_partial_loss` |

## Drill schedule

| Packet family | Cadence | Current owner | Future owner | Evidence state | Needs drill |
|---|---|---|---|---|---|
| `backup` | `per_release` | Managed platform on-call | Reliability guild | `current` | no |
| `failover` | `quarterly` | Managed platform on-call | Reliability guild | `current` | no |
| `restore` | `semiannual` | Customer success SRE | Field reliability owner | `reconstructable_from_snapshot` | no |
| `snapshot_replication` | `annual` | Sovereign operations lead | Customer compliance owner | `stale_within_grace` | no |
| `local_core_continuity` | `on_demand_only` | Local user | Local user | `reconstructable_from_snapshot` | no |

## Claim-narrowing drills

Each drill mutates one seeded row and shows the claim narrowing automatically:

- `drill_managed_restore_drill_stale_beta` — stale managed backup drill → **beta**
  (`drill_evidence_stale`)
- `drill_drill_never_run_preview` — managed continuity drill never run →
  **preview** (`drill_never_run`)
- `drill_sovereign_hidden_vendor_failover_withdrawn` — sovereign row hides a
  vendor-operated failover lane → **withdrawn**
  (`sovereign_continuity_overclaimed`)
- `drill_locality_undisclosed_beta` — managed relay row hides processing
  locality → **beta** (`locality_undisclosed`)
- `drill_local_only_overclaimed_preview` — local-only row names a managed backup
  family without a managed dependency → **preview**
  (`local_only_overclaimed_as_managed`)
- `drill_partial_loss_undisclosed_beta` — self-hosted restore row hides
  partial-loss behavior → **beta** (`partial_loss_undisclosed`)

## Fixture references

- `fixtures/continuity/m5-continuity-profile-cases/page.json`
- `fixtures/continuity/m5-continuity-profile-cases/summary.json`
- `fixtures/continuity/m5-continuity-profile-cases/support_export.json`
- `fixtures/continuity/m5-continuity-profile-cases/drill_managed_restore_drill_stale_beta.json`
- `fixtures/continuity/m5-continuity-profile-cases/drill_drill_never_run_preview.json`
- `fixtures/continuity/m5-continuity-profile-cases/drill_sovereign_hidden_vendor_failover_withdrawn.json`
- `fixtures/continuity/m5-continuity-profile-cases/drill_locality_undisclosed_beta.json`
- `fixtures/continuity/m5-continuity-profile-cases/drill_local_only_overclaimed_preview.json`
- `fixtures/continuity/m5-continuity-profile-cases/drill_partial_loss_undisclosed_beta.json`

## Continuity-proof freshness SLO

These claim rows state *what* each continuity claim discloses. *How fresh* the
evidence behind each managed, self-hosted, or sovereign row still is — and when a
stale, missing, unattested, or unrefreshable row narrows or holds promotion — is
tracked by the continuity-proof freshness SLO dashboard, the canonical M5 source
for continuity-proof freshness truth:

- artifact: `artifacts/m5/continuity/freshness_slo_dashboard.json`
- schema: `schemas/continuity/continuity_freshness_slo_dashboard.schema.json`
- doc: `docs/release/m5-continuity-shiproom-gates.md`
- rerun tool: `tools/continuity/run_drill_packets.py`
- CI gate: `tools/check_m5_continuity_freshness.py`
- stale-evidence fixtures: `fixtures/continuity/stale_evidence_cases/`

The local-core continuity lane never narrows or blocks promotion when a managed
continuity row goes stale; only the affected managed claim narrows.
