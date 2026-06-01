# Fixtures: stabilize-transport-governance-and-egress-classification-across-update

These fixtures document the stable transport governance and egress
classification proof packet. The canonical source of truth is the seeded
packet produced by
`aureline_remote::stabilize_transport_governance_and_egress_classification_across_update::seeded_transport_governance_page()`.

## Files

| File | Content |
|------|---------|
| `page.json` | Full `TransportGovernancePage` proof packet (stable, zero defects) |
| `rows.json` | Per-lane `TransportGovernanceRow` records (all 7 required lanes) |
| `defects.json` | Empty defect list (clean stable packet) |
| `summary.json` | `TransportGovernanceSummary` with counts and overall qualification |
| `support_export.json` | `TransportGovernanceSupportExport` envelope for support/diagnostics |
| `drill_missing_lane_preview.json` | Drill: missing `ai` lane narrows to `preview` |
| `drill_raw_material_withdrawn.json` | Drill: raw private material on `update` lane withdraws packet |
| `drill_no_local_core_continuity_beta.json` | Drill: `marketplace` lane without local-core continuity narrows to `beta` |

## Required lanes

All seven required egress lanes must be present for a stable claim:
`update`, `marketplace`, `ai`, `docs`, `provider`, `remote`, `mirror_offline`.

## Schema

`schemas/enterprise/stabilize-transport-governance-and-egress-classification-across-update.schema.json`

## Contract ref

`remote:transport_governance_stabilize:v1`
