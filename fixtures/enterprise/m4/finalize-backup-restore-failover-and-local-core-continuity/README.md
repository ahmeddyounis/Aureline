# Fixtures: finalize-backup-restore-failover-and-local-core-continuity

Reference fixtures for the backup, restore, failover, and local-core
continuity finalize packet (`policy:backup_restore_failover_continuity:v1`).

## Files

| File | Description |
| --- | --- |
| `page.json` | Seeded finalize page covering all five enterprise profiles; qualifies stable with zero defects. |
| `summary.json` | Summary record from the seeded page. |
| `defects.json` | Empty defect list for the seeded page. |
| `drill_local_core_blocked_withdrawn.json` | Failure drill: `failover_behavior: local_core_blocked` triggers immediate withdrawal. |
| `drill_missing_profiles_preview.json` | Failure drill: missing enterprise profiles narrow the page to preview. |
| `drill_unverified_backup_beta.json` | Failure drill: `backup_state: unverified` on an enterprise row narrows toward beta. |

## Schema

All records conform to
`schemas/enterprise/finalize-backup-restore-failover-and-local-core-continuity.schema.json`.

## Canonical paths

- Doc: `docs/enterprise/m4/finalize-backup-restore-failover-and-local-core-continuity.md`
- Artifact: `artifacts/enterprise/m4/finalize-backup-restore-failover-and-local-core-continuity.md`
- Runtime owner: `aureline_policy::finalize_backup_restore_failover_and_local_core_continuity`
