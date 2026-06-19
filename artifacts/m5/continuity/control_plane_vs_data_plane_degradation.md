# Artifact: control-plane-versus-data-plane outage taxonomy

**Contract ref:** `continuity:m5_control_plane_vs_data_plane_outage:v1`  
**Schema:** `schemas/continuity/control_vs_data_plane_packet.schema.json`  
**Doc:** `docs/m5/continuity/control-plane-vs-data-plane-degradation.md`  
**Runtime owner:** `aureline_continuity::m5_control_plane_vs_data_plane_outage`

## Qualification

| Condition | Status |
|---|---|
| Every optional-service family has an outage packet | ✓ Stable |
| Both a control-plane and a data-plane outage are classified | ✓ Stable |
| Every impaired lane names a narrower fallback | ✓ Stable |
| Every packet preserves local-core editing/save/search/Git | ✓ Stable |
| No packet flips a global "IDE down" state | ✓ Stable |
| Outage + local-core vocabulary identical across surfaces | ✓ Stable |
| No raw provider payloads in any record | ✓ Stable |
| **Overall** | **Stable** |

## Outage packets

| Lane | Impaired plane | Severity | Fallback | Degraded state |
|---|---|---|---|---|
| Identity and policy | `control_plane_impairment` | `degraded` | `cached_policy_read_only` | `control_plane_impaired_local_core_preserved` |
| Registry, updates, and docs | `control_plane_impairment` | `degraded` | `serve_from_cache` | `control_plane_impaired_local_core_preserved` |
| Collaboration | `data_plane_impairment` | `unavailable` | `queue_and_reconcile` | `managed_data_plane_impaired_local_core_preserved` |
| Remote control plane | `control_plane_impairment` | `unavailable` | `fail_closed_local_core_only` | `control_plane_impaired_local_core_preserved` |
| AI gateway | `data_plane_impairment` | `degraded` | `local_model_or_manual_fallback` | `managed_data_plane_impaired_local_core_preserved` |
| Telemetry and support | `control_plane_impairment` | `recovering` | `buffer_locally_and_ship_later` | `control_plane_impaired_local_core_preserved` |

The seeded page carries one simulated impairment for every claimed
optional-service family across both planes and a mix of degraded, unavailable,
and recovering severities, satisfying the requirement that the proof packet be
exercised by at least one simulated impairment per optional-service family. Every
row keeps local editing, save, search, and version control available, and none
flips a global "IDE down" state.

## Fail-closed and narrowing cases

| Fixture | Trigger | Outcome |
|---|---|---|
| `case_ide_down_conflation_withdrawn.json` | collaboration outage flips a global "IDE down" state | withdrawn (fail closed) |
| `case_local_editing_conflated_withdrawn.json` | remote control-plane outage marks local editing/save down | withdrawn (fail closed) |
| `case_fallback_undeclared_beta.json` | impaired AI gateway names no fallback | beta |
| `case_operational_inconsistent_preview.json` | operational lane still claims an active fallback | preview |
| `case_outage_evidence_stale_preview.json` | registry/updates/docs outage evidence stale | preview |
| `case_family_coverage_incomplete_beta.json` | telemetry/support family missing | beta |

In every fail-closed case only the conflating packet is withdrawn; the other
packets stay `stable` and `local_core_preserved` remains true on every row that
does not itself conflate the outage with local editing.

## Export safety

All records are metadata-only: closed-vocabulary tokens, export-safe
plain-language labels, and opaque evidence refs. The summary and support-export
records assert `raw_payloads_excluded`. No raw provider payloads, raw incident
bodies, hostnames, or secret material appear.
