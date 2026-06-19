# Artifact: backup/restore/failover continuity packets

**Contract ref:** `continuity:m5_backup_restore_failover_packets:v1`  
**Schema:** `schemas/continuity/backup_restore_failover_packet.schema.json`  
**Doc:** `docs/m5/continuity/backup-restore-failover-packets.md`  
**Runtime owner:** `aureline_continuity::m5_backup_restore_failover_packets`

## Qualification

| Condition | Status |
|---|---|
| Every managed-family packet records typed exercised operations | ✓ Stable |
| Partial drills disclose what restored narrower than normal | ✓ Stable |
| Restore identity declared on every managed-family packet | ✓ Stable |
| Partial-loss disclosed on every packet | ✓ Stable |
| Drill cadence + current/future owner named | ✓ Stable |
| Last-drill timestamp + freshness-SLO expiry recorded | ✓ Stable |
| Every claimed resilience row points to a current packet | ✓ Stable |
| Surface fact reuse complete + vocabulary identical | ✓ Stable |
| No generic "DR tested" text | ✓ Stable |
| **Overall** | **Stable** |

## Packets

| Surface | Profile | Family | Scope exercised | Restore identity | Partial loss | Hosting |
|---|---|---|---|---|---|---|
| Managed cloud workspace backup | `managed` | `backup` | `fully_exercised` | `same_identity_restore` | `bounded_recent_window_loss` | `vendor_operated` |
| Managed relay and collaboration failover | `managed` | `failover` | `fully_exercised` | `same_identity_restore` | `queued_action_loss` | `vendor_operated` |
| Customer self-hosted restore and rebuild | `self_hosted` | `restore` | `partially_exercised` | `reissued_identity_restore` | `bounded_recent_window_loss` | `customer_operated` |
| Sovereign air-gapped snapshot and replication | `sovereign` | `snapshot_replication` | `fully_exercised` | `new_install_rebind` | `cache_only_loss` | `offline_snapshot` |
| Local desktop core continuity | `local_only` | `local_core_continuity` | `fully_exercised` | `not_applicable` | `no_partial_loss` | `local_core` |

## Drill schedule

| Family | Cadence | Current owner | Future owner | Evidence state | Expires |
|---|---|---|---|---|---|
| `backup` | `per_release` | Managed platform on-call | Reliability guild | `current` | 2026-07-30 |
| `failover` | `quarterly` | Managed platform on-call | Reliability guild | `current` | 2026-08-20 |
| `restore` | `semiannual` | Customer success SRE | Field reliability owner | `reconstructable_from_snapshot` | — |
| `snapshot_replication` | `annual` | Sovereign operations lead | Customer compliance owner | `stale_within_grace` | 2026-07-15 |
| `local_core_continuity` | `on_demand_only` | Local user | Local user | `reconstructable_from_snapshot` | — |

## Claim-narrowing cases

Each case mutates one seeded packet and shows the claim narrowing automatically:

- `case_generic_dr_text_withdrawn` — a packet rests on generic "DR tested" text →
  **withdrawn** (`generic_dr_text_only`)
- `case_sovereign_hidden_vendor_failover_withdrawn` — a sovereign packet hides a
  vendor-operated failover lane → **withdrawn**
  (`sovereign_continuity_overclaimed`)
- `case_scope_not_exercised_preview` — a managed failover packet exercised
  nothing → **preview** (`scope_not_exercised`)
- `case_drill_never_run_preview` — a managed backup drill has never been run →
  **preview** (`drill_never_run`)
- `case_packet_evidence_missing_preview` — a claimed resilience row carries no
  packet → **preview** (`packet_evidence_missing`)
- `case_not_exercised_disclosure_missing_beta` — a partial drill omits what
  restored narrower than normal → **beta** (`not_exercised_disclosure_missing`)
- `case_restore_identity_undeclared_beta` — a managed backup packet declares no
  restore identity → **beta** (`restore_identity_undeclared`)
- `case_drill_evidence_stale_beta` — a sovereign snapshot drill has aged out
  under its freshness SLO → **beta** (`drill_evidence_stale`)

## Canonical evidence packets

- `artifacts/m5/continuity/drill_packets/backup_restore_failover_page.json`
- `artifacts/m5/continuity/drill_packets/drill_packet_registry.json`
- `artifacts/m5/continuity/drill_packets/backup_restore_failover_support_export.json`

## Fixture references

- `fixtures/continuity/restore_identity_cases/page.json`
- `fixtures/continuity/restore_identity_cases/summary.json`
- `fixtures/continuity/restore_identity_cases/registry.json`
- `fixtures/continuity/restore_identity_cases/support_export.json`
- `fixtures/continuity/restore_identity_cases/case_generic_dr_text_withdrawn.json`
- `fixtures/continuity/restore_identity_cases/case_not_exercised_disclosure_missing_beta.json`
- `fixtures/continuity/restore_identity_cases/case_scope_not_exercised_preview.json`
- `fixtures/continuity/restore_identity_cases/case_restore_identity_undeclared_beta.json`
- `fixtures/continuity/restore_identity_cases/case_drill_never_run_preview.json`
- `fixtures/continuity/restore_identity_cases/case_drill_evidence_stale_beta.json`
- `fixtures/continuity/restore_identity_cases/case_sovereign_hidden_vendor_failover_withdrawn.json`
- `fixtures/continuity/restore_identity_cases/case_packet_evidence_missing_preview.json`
