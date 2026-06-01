# Transport Governance and Egress Classification — Stable Packet

- Packet: `remote:transport_governance:default`
- Schema version: `1`
- Contract ref: `remote:transport_governance_stabilize:v1`
- Qualification: `stable` (derived, not asserted)
- Stabilize defects: 0
- Withdrawn rows: 0
- Stable rows: all (7)

## Lane coverage

| Lane | Route class | Egress decision | Offline posture | Dependency | Ctrl plane | Data plane | Local-core | Policy epoch |
|---|---|---|---|---|---|---|---|---|
| `update` | `direct` | `allowed` | `online` | `network` | `reachable` | `reachable` | ✓ | present |
| `marketplace` | `direct` | `allowed` | `online` | `network` | `reachable` | `reachable` | ✓ | present |
| `ai` | `direct` | `allowed` | `online` | `managed` | `reachable` | `reachable` | ✓ | present |
| `docs` | `direct` | `allowed` | `cached_content` | `network` | `reachable` | `cached_served` | ✓ | present |
| `provider` | `direct` | `allowed` | `online` | `managed` | `reachable` | `reachable` | ✓ | present |
| `remote` | `direct` | `allowed` | `online` | `network` | `reachable` | `reachable` | ✓ | present |
| `mirror_offline` | `mirror_first` | `mirror_routed` | `mirror_served` | `air_gapped` | `mirror_routed` | `cached_served` | ✓ | — |

## Key invariants verified

1. All seven required egress lanes are covered by typed `TransportPolicyRecord` entries.
2. No raw private material is exposed on any lane record (`raw_private_material_excluded: true` on all records).
3. Every lane explicitly declares `local_core_continuity_allowed: true`; local editing is never blocked by managed or network-dependent lane failures.
4. Every lane carries an explicit `dependency_class_token` (`network`, `managed`, or `air_gapped`).
5. Every lane carries a typed `egress_decision_token` so route selection is reconstructable from typed records without parsing raw logs.
6. Every network-dependent lane (`update`, `marketplace`, `ai`, `docs`, `provider`, `remote`) carries a non-empty `last_known_good_policy_epoch_ref`.
7. Every lane records distinct `control_plane_status_token` and `data_plane_status_token` so control-plane impairment is distinguishable from data-plane impairment.

## Hard guardrail — withdrawal condition

The following forces `Withdrawn` immediately and cannot be overridden:

- Any lane record with `raw_private_material_excluded: false`
  (narrow reason: `raw_private_material_exposed`).

## Failure / recovery drill coverage

| Scenario | Lane(s) | Expected behavior |
|---|---|---|
| Primary endpoint unreachable | `update`, `marketplace`, `docs` | Narrows to `Beta`; `policy_epoch_ref_missing` defect if epoch not preserved |
| AI provider unavailable | `ai` | Local editing continues; managed feature degrades without blocking local work |
| Mirror-first, no internet | `mirror_offline` | All traffic to declared mirror; `mirror_routed` decision token; local core unaffected |
| Raw credential on lane record | any | Immediate `Withdrawn` qualification; `raw_private_material_exposed` defect |
| Missing lane | any required | Narrows to `Preview`; `required_lane_missing` defect for each absent lane |

## Canonical paths

- Doc: `docs/enterprise/m4/stabilize-transport-governance-and-egress-classification-across-update.md`
- Runtime owner: `aureline_remote::stabilize_transport_governance_and_egress_classification_across_update`
- Fixtures: `fixtures/enterprise/m4/stabilize-transport-governance-and-egress-classification-across-update/`
- Schema: `schemas/enterprise/stabilize-transport-governance-and-egress-classification-across-update.schema.json`
