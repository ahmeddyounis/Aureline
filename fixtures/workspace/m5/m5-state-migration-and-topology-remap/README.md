# M5 state-migration and topology-remap fixtures

Scenario fixtures for migration/remap events. Each file is a single
[`MigrationRemapEvent`](../../../../crates/aureline-workspace/src/m5_state_migration_and_topology_remap/mod.rs)
that the crate's unit tests deserialize and assert against; the canonical full packet lives at
`artifacts/workspace/m5/m5-state-migration-and-topology-remap.json` and is exercised by the
embedded-packet tests and the fail-closed gate drills.

| Fixture | Proves |
| --- | --- |
| `schema_jump_forward_migrated.json` | A schema jump (v1 → v3, two forward steps) is published as a **compatible** restore with compatible-downgrade language; the prior artifact is archived, not discarded. |
| `schema_jump_unmigratable.json` | A schema this build cannot migrate is held for **manual review** with the prior artifact retained — never auto-applied, never silently discarded, and the safer review path is never hidden. |
| `foreign_machine_import.json` | A package from a foreign machine discloses its origin and machine-local exclusions before restore; its remapped paths narrow it to a **layout-only** restore with a relocate-dependency next step. |
| `mixed_channel_import.json` | A package produced on a different release channel is forward-migrated and published as a **compatible** restore, never a clean schema match. |
| `monitor_detach_reattach.json` | A monitor detach/reattach with a DPI change materially alters placement; the layout is adapted onto available displays and published as a **compatible** platform remap — a deliberate downgrade, not layout corruption. |

The fidelity labels (`exact_restore`, `compatible_restore`, `layout_only`, `manual_review`), the
artifact-class labels, the schema/dependency/topology/freshness conditions, the downgrade reasons,
the recovery paths, and the redaction-exclusion labels are reused from the serialization-and-restore
matrix vocabulary rather than redefined, so migration/remap meaning cannot fork between desktop
restore, import, crash recovery, support replay, and companion/browser re-entry.

The fail-closed rejections — an overstated fidelity, a discarded prior artifact, a silent layout
delete, a mixed channel claiming a clean schema match, a migration result that disagrees with the
schema condition, a remap that disagrees with the topology condition, a foreign import that hides
its origin or machine-local exclusions, a missing redaction exclusion, a dropped
open-details/compare/recovery action, an inaccessible or unscoped affordance, a downgrade-reason or
recovery-path mismatch, an exact event that is not a clean no-migration baseline, and a missing or
drifted consumer binding — are exercised as synthetic gate drills in the crate's
`m5_state_migration_and_topology_remap` unit tests rather than as checked-in invalid fixtures.
